use std::{io::BufRead, mem, str};

use once_cell::sync::Lazy;
use quick_xml::{
    events::{BytesStart, Event},
    name::{Namespace, ResolveResult::Bound},
    NsReader,
};
use regex::Regex;
use thiserror::Error;
use url::Url;

use crate::{
    appearance::{TexCoordList, TextureAssociation},
    codelist::{self, CodeResolver},
    geometry::{
        GeometryCollector, GeometryParseType, GeometryRef, GeometryRefs, GeometryStore,
        GeometryType,
    },
    namespace::{wellknown_prefix_from_nsres, APP_2_NS, GML31_NS},
    CityGmlAttribute, LocalId, OrientableSurface, SurfaceSpan, XLINK_NS,
};

static PROPERTY_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([a-zA-Z0-9:_]+)\[([^\]]+)\]").unwrap());

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Broken XML: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("Invalid structure: {0}")]
    SchemaViolation(String),
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Codelist error: {0}")]
    CodelistError(String),
    #[error("canceled")]
    Canceled,
}

pub struct CityGmlReader<'a> {
    state: InternalState<'a>,
}

struct InternalState<'a> {
    /// Buffer holding the current path
    path_buf: Vec<u8>,
    /// Stack of indices of slashes '/' in `path_buf`
    path_stack_indices: Vec<usize>,
    /// General purpose buffer 1
    buf1: Vec<u8>,
    /// General purpose buffer 2
    buf2: Vec<u8>,
    /// Floating-point number buffer
    fp_buf: Vec<f64>,
    /// Data of last start tag
    current_start: Option<BytesStart<'static>>,

    /// Current geometry store
    geometry_collector: GeometryCollector,

    /// URI of the source file
    context: ParseContext<'a>,

    /// Stack of feature IDs and types (elements with gml:id)
    /// Each entry is (feature_id, feature_type, path_depth)
    feature_stack: Vec<(String, String, usize)>,
}

impl<'a> InternalState<'a> {
    fn new(context: ParseContext<'a>) -> Self {
        Self {
            path_buf: Vec::new(),
            path_stack_indices: Vec::new(),
            buf1: Vec::new(),
            buf2: Vec::new(),
            fp_buf: Vec::new(),
            current_start: None,
            geometry_collector: GeometryCollector::default(),
            context,
            feature_stack: Vec::new(),
        }
    }
}

pub struct ParseContext<'a> {
    source_uri: Url,
    code_resolver: &'a dyn CodeResolver,
}

impl<'a> ParseContext<'a> {
    pub fn new(source_uri: Url, code_resolver: &'a dyn CodeResolver) -> Self {
        Self {
            source_uri,
            code_resolver,
        }
    }

    pub fn source_url(&self) -> &Url {
        &self.source_uri
    }

    pub fn code_resolver(&self) -> &dyn CodeResolver {
        self.code_resolver
    }
}

impl Default for ParseContext<'_> {
    fn default() -> Self {
        Self {
            source_uri: Url::parse("file:///").unwrap(),
            code_resolver: &codelist::NoopResolver {},
        }
    }
}

#[allow(unknown_lints, mismatched_lifetime_syntaxes)]
impl<'a> CityGmlReader<'a> {
    pub fn new(context: ParseContext<'a>) -> Self {
        Self {
            state: InternalState::new(context),
        }
    }

    #[allow(mismatched_lifetime_syntaxes)]
    pub fn start_root<'b: 'a, R: BufRead>(
        &'a mut self,
        reader: &'b mut quick_xml::NsReader<R>,
    ) -> Result<SubTreeReader<'a, 'a, R>, ParseError> {
        let config = reader.config_mut();
        config.trim_text(true);
        config.expand_empty_elements = true;

        let state = &mut self.state;
        loop {
            match reader.read_event_into(&mut state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = reader.resolve_element(start.name());
                    state.path_stack_indices.push(state.path_buf.len());
                    state.path_buf.push(b'/');
                    state.path_buf.extend(wellknown_prefix_from_nsres(&nsres));
                    state.path_buf.extend(localname.as_ref());
                    state.current_start = Some(start.into_owned());
                    return Ok(SubTreeReader {
                        reader,
                        state,
                        path_start: 0,
                    });
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }
}

pub struct SubTreeReader<'a, 'b, R> {
    reader: &'a mut quick_xml::NsReader<R>,
    state: &'a mut InternalState<'b>,
    path_start: usize,
}

impl<'b, R: BufRead> SubTreeReader<'_, 'b, R> {
    pub fn parse_children(
        &mut self,
        logic: impl FnMut(&mut SubTreeReader<R>) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        // spawn new subtree reader and parse children
        SubTreeReader {
            path_start: self.state.path_buf.len(),
            reader: self.reader,
            state: self.state,
        }
        .parse_children_inner(logic)
    }

    fn parse_children_inner(
        &mut self,
        mut logic: impl FnMut(&mut SubTreeReader<R>) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        loop {
            self.state.current_start = None;
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    let ns = wellknown_prefix_from_nsres(&nsres);

                    // Append "/{ns_prefix}:{localname}" to the path stack
                    self.state
                        .path_stack_indices
                        .push(self.state.path_buf.len());
                    self.state.path_buf.push(b'/');
                    self.state.path_buf.extend(ns);
                    self.state.path_buf.extend(localname.as_ref());

                    // Check if this element has gml:id and push to feature stack
                    let current_depth = self.state.path_stack_indices.len();
                    let mut feature_id: Option<String> = None;
                    for attr in start.attributes().flatten() {
                        let (attr_nsres, attr_localname) = self.reader.resolve_attribute(attr.key);
                        if attr_nsres
                            == quick_xml::name::ResolveResult::Bound(quick_xml::name::Namespace(
                                b"http://www.opengis.net/gml",
                            ))
                            && attr_localname.as_ref() == b"id"
                        {
                            feature_id =
                                Some(String::from_utf8_lossy(attr.value.as_ref()).to_string());
                            break;
                        }
                    }

                    if let Some(fid) = feature_id {
                        // This element has gml:id - save it as a feature
                        // ns already includes the colon (e.g., "bldg:")
                        let feature_type = format!(
                            "{}{}",
                            String::from_utf8_lossy(ns),
                            String::from_utf8_lossy(localname.as_ref())
                        );
                        self.state
                            .feature_stack
                            .push((fid, feature_type, current_depth));
                    }

                    // save start tag
                    self.state
                        .current_start
                        .clone_from(&Some(start.into_owned()));

                    logic(self)?;
                }
                Ok(Event::End(_)) => {
                    let popped_depth = self.state.path_stack_indices.len();
                    self.state
                        .path_buf
                        .truncate(self.state.path_stack_indices.pop().unwrap());

                    // Pop from feature stack if this was a feature element
                    if let Some((_, _, depth)) = self.state.feature_stack.last() {
                        if *depth == popped_depth {
                            self.state.feature_stack.pop();
                        }
                    }

                    if self.state.path_buf.len() < self.path_start {
                        return Ok(());
                    }
                }
                Ok(Event::Eof) => return Ok(()),
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }
    }

    pub fn parse_attributes(
        &mut self,
        mut logic: impl FnMut(&[u8], &[u8], &mut ParseContext) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let Some(start) = &self.state.current_start else {
            panic!("parse_attributes() must be called immediately after encountering a start tag.");
        };
        let mut property_buf = Vec::new();
        property_buf.push(b'[');
        self.state.buf1.clear();
        self.state.buf1.push(b'@');
        for (index, attr) in start.attributes().flatten().enumerate() {
            if index > 0 {
                property_buf.push(b',');
            }
            let (nsres, localname) = self.reader.resolve_attribute(attr.key);
            self.state.buf1.extend(wellknown_prefix_from_nsres(&nsres));
            self.state.buf1.extend(localname.as_ref());
            property_buf.extend(wellknown_prefix_from_nsres(&nsres));
            property_buf.extend(localname.as_ref());
            property_buf.extend(b"=");
            property_buf.extend(attr.value.as_ref());

            logic(
                self.state.buf1.as_ref(), // attribute path "@nsprefix:name"
                attr.value.as_ref(),      // attribute value
                &mut self.state.context,
            )?;
            self.state.buf1.truncate(1);
        }
        property_buf.push(b']');
        self.state.path_buf.extend(&property_buf);
        Ok(())
    }

    pub fn find_codespace_attr(&mut self) -> Option<String> {
        let Some(start) = &self.state.current_start else {
            panic!("find_codespace() must be called immediately after encountering a start tag.");
        };
        for attr in start.attributes().flatten() {
            if attr.key.as_ref() == b"codeSpace" {
                return Some(String::from_utf8_lossy(attr.value.as_ref()).into_owned());
            }
        }
        None
    }

    pub fn skip_current_element(&mut self) -> Result<(), ParseError> {
        let Some(start) = &self.state.current_start else {
            panic!(
                "skip_current_element() must be called immediately after encountering a new \
                 starting tag."
            );
        };
        self.reader
            .read_to_end_into(start.name(), &mut self.state.buf1)?;
        self.state
            .path_buf
            .truncate(self.state.path_stack_indices.pop().unwrap());
        self.state.current_start = None;
        Ok(())
    }

    /// Gets the current sub-tree path to the current element.
    pub fn current_path(&self) -> Vec<u8> {
        if self.path_start + 1 < self.state.path_buf.len() {
            let current_path = &self.state.path_buf[self.path_start + 1..];
            let path = String::from_utf8_lossy(current_path);
            if let Some(captures) = PROPERTY_PATTERN.captures(path.to_string().as_str()) {
                if let Some(value) = captures.get(1).map(|m| m.as_str()) {
                    let result = value.to_string();
                    result.into_bytes()
                } else {
                    current_path.to_vec()
                }
            } else {
                current_path.to_vec()
            }
        } else {
            Vec::new()
        }
    }

    /// Gets the current absolute path from the root to the current element.
    pub fn current_absolute_path(&self) -> &[u8] {
        &self.state.path_buf
    }

    /// Expect a XML text content and return it.
    pub fn parse_text(&mut self) -> Result<&str, ParseError> {
        self.state.buf2.clear();
        self.state.current_start = None;
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    return Err(ParseError::SchemaViolation(format!(
                        "Text content is expected, but a <{}> is found.",
                        String::from_utf8_lossy(start.local_name().as_ref())
                    )));
                }
                Ok(Event::Text(text)) => self.state.buf2.extend(text.as_ref()),
                Ok(Event::End(_)) => {
                    self.state
                        .path_buf
                        .truncate(self.state.path_stack_indices.pop().unwrap());
                    return str::from_utf8(self.state.buf2.as_ref())
                        .map_err(|e| ParseError::InvalidValue(format!("Invalid UTF-8: {e}")));
                }
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }
    }

    pub fn context(&self) -> &ParseContext<'_> {
        &self.state.context
    }

    pub fn context_mut(&mut self) -> &mut ParseContext<'b> {
        &mut self.state.context
    }

    pub fn id_to_integer_id(&mut self, id: String) -> LocalId {
        LocalId::from(id)
    }

    pub fn collect_geometries(&mut self, envelope_crs_uri: Option<String>) -> GeometryStore {
        let collector = std::mem::take(&mut self.state.geometry_collector);
        collector.into_geometries(envelope_crs_uri)
    }

    /// Extract feature ID and feature type from the current parsing context.
    ///
    /// # Important
    /// The returned `feature_type` is **guaranteed to be a CityGML Feature Type** defined in the
    /// CityGML specification (e.g., `bldg:Building`, `bldg:GroundSurface`, `tran:Road`).
    ///
    /// This function is only called from `parse_geometric_attr()`, which itself is only invoked
    /// for fields marked with `#[citygml(geom = ...)]` attribute in structs decorated with
    /// `#[citygml_feature]` macro. These macros are used exclusively for CityGML Feature Types
    /// defined in the specification.
    ///
    /// **Therefore, arbitrary XML tag names will never be returned as feature_type.**
    ///
    /// # Returns
    /// * `(Some(id), Some(type))` - feature_id from parent with gml:id, feature_type from current element
    /// * `(None, Some(type))` - No parent with gml:id, feature_type from current element
    /// * `(None, None)` - When extraction fails
    ///
    /// # Behavior
    /// - `feature_id`: Extracted from the nearest parent element with gml:id attribute (from feature_stack)
    /// - `feature_type`: Extracted from the current element's tag name (the element containing the geometry)
    ///
    /// This ensures that for structures like `Building[gml:id=X] → boundedBy → GroundSurface → geometry`:
    /// - `feature_id` = "X" (from Building)
    /// - `feature_type` = "bldg:GroundSurface" (from the current element with geometry)
    pub fn extract_feature_id_and_type(&self) -> (Option<String>, Option<String>) {
        // Extract feature_id from the feature stack (nearest parent with gml:id)
        let feature_id = self.state.feature_stack.last().map(|(id, _, _)| id.clone());

        // Extract feature_type from current element's path
        if self.state.path_stack_indices.len() < 2 {
            return (feature_id, None);
        }

        let paths = String::from_utf8_lossy(self.state.path_buf.as_ref());
        let start = self.state.path_stack_indices[self.state.path_stack_indices.len() - 2];
        let end = self.state.path_stack_indices[self.state.path_stack_indices.len() - 1];
        let before_tag = &paths[start + 1..end];

        // Extract feature_type from the current element's tag
        let feature_type = if !before_tag.is_empty() {
            if !before_tag.contains('[') {
                // No brackets at all - simple tag name
                Some(before_tag.to_string())
            } else {
                // Has brackets - extract tag name before the bracket
                before_tag.split('[').next().map(|tag| tag.to_string())
            }
        } else {
            None
        };

        (feature_id, feature_type)
    }

    /// Expect a geometric attribute of CityGML
    #[inline(never)]
    pub fn parse_geometric_attr(
        &mut self,
        geomref: &mut GeometryRefs,
        lod: u8,
        geomtype: GeometryParseType,
    ) -> Result<(), ParseError> {
        use GeometryParseType::*;
        let (feature_id, feature_type) = self.extract_feature_id_and_type();

        match geomtype {
            Solid => self.parse_solid_prop(geomref, lod, feature_id, feature_type)?,
            MultiSurface => {
                self.parse_multi_surface_prop(geomref, lod, feature_id, feature_type)?
            }
            Surface => self.parse_surface_prop(geomref, lod, feature_id, feature_type)?, // FIXME
            Geometry => self.parse_geometry_prop(geomref, lod, feature_id, feature_type)?, // FIXME: not only surfaces
            Triangulated => self.parse_triangulated_prop(geomref, lod, feature_id, feature_type)?, // FIXME
            Point => todo!(),      // FIXME
            MultiPoint => todo!(), // FIXME
            MultiCurve => self.parse_multi_curve_prop(geomref, lod, feature_id, feature_type)?, // FIXME
            CompositeCurve => {
                self.parse_composite_curve_prop(geomref, lod, feature_id, feature_type)?
            }
        }

        self.state
            .path_buf
            .truncate(self.state.path_stack_indices.pop().unwrap());

        Ok(())
    }

    fn parse_multi_curve_prop(
        &mut self,
        geomrefs: &mut GeometryRefs,
        lod: u8,
        feature_id: Option<String>,
        feature_type: Option<String>,
    ) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    let line_begin = self.state.geometry_collector.multilinestring.len();

                    let geomtype = match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"MultiCurve") => {
                            self.parse_multi_curve()?;
                            GeometryType::Curve
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Expected MultiCurve but found <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )))
                        }
                    };

                    let line_end = self.state.geometry_collector.multilinestring.len();
                    if line_end - line_begin > 0 {
                        geomrefs.push(GeometryRef {
                            ty: geomtype,
                            lod,
                            pos: line_begin as u32,
                            len: (line_end - line_begin) as u32,
                            id: None,
                            solid_ids: Vec::new(),
                            feature_id: feature_id.clone(),
                            feature_type: feature_type.clone(),
                        });
                    }
                }
                Ok(Event::End(_)) => break,
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn parse_composite_curve_prop(
        &mut self,
        geomrefs: &mut GeometryRefs,
        lod: u8,
        feature_id: Option<String>,
        feature_type: Option<String>,
    ) -> Result<(), ParseError> {
        // CompositeCurve has stricter semantics than MultiCurve:
        // - Segments MUST be topologically connected (end-to-end)
        // - Order matters (forms a continuous path)
        // However, we reuse parse_multi_curve() because the XML structure is identical (both use <gml:curveMember>).
        // If needed, topology validation should be handled separately. Here, we just parse the geometry for efficiency.
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    let line_begin = self.state.geometry_collector.multilinestring.len();

                    let geomtype = match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"CompositeCurve") => {
                            // CompositeCurve uses curveMember elements just like MultiCurve
                            self.parse_multi_curve()?;
                            GeometryType::Curve
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Expected CompositeCurve but found <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )))
                        }
                    };

                    let line_end = self.state.geometry_collector.multilinestring.len();
                    if line_end - line_begin > 0 {
                        geomrefs.push(GeometryRef {
                            ty: geomtype,
                            lod,
                            pos: line_begin as u32,
                            len: (line_end - line_begin) as u32,
                            id: None,
                            solid_ids: Vec::new(),
                            feature_id: feature_id.clone(),
                            feature_type: feature_type.clone(),
                        });
                    }
                }
                Ok(Event::End(_)) => break,
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn parse_multi_surface_prop(
        &mut self,
        geomrefs: &mut GeometryRefs,
        lod: u8,
        feature_id: Option<String>,
        feature_type: Option<String>,
    ) -> Result<(), ParseError> {
        let mut surface_id = None;
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    // surface id
                    for attr in start.attributes().flatten() {
                        let (nsres, localname) = self.reader.resolve_attribute(attr.key);
                        if nsres == Bound(GML31_NS) && localname.as_ref() == b"id" {
                            let id = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            surface_id = Some(LocalId::from(id));
                            break;
                        }
                    }

                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    let poly_begin = self.state.geometry_collector.multipolygon.len();

                    let geomtype = match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"MultiSurface") => {
                            let id = self.parse_multi_surface()?;
                            if id.is_some() {
                                surface_id = id;
                            }
                            GeometryType::Surface
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Expected MultiSurface but found <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )))
                        }
                    };

                    let poly_end = self.state.geometry_collector.multipolygon.len();
                    if poly_end - poly_begin > 0 {
                        geomrefs.push(GeometryRef {
                            ty: geomtype,
                            lod,
                            pos: poly_begin as u32,
                            len: (poly_end - poly_begin) as u32,
                            id: surface_id.clone(),
                            solid_ids: Vec::new(),
                            feature_id: feature_id.clone(),
                            feature_type: feature_type.clone(),
                        });

                        // record a partial surface span
                        if let Some(id) = surface_id.clone() {
                            self.state
                                .geometry_collector
                                .surface_spans
                                .push(SurfaceSpan {
                                    id,
                                    start: poly_begin as u32,
                                    end: poly_end as u32,
                                });
                        }
                    }
                }
                Ok(Event::End(_)) => break,
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn parse_surface_prop(
        &mut self,
        geomrefs: &mut GeometryRefs,
        lod: u8,
        feature_id: Option<String>,
        feature_type: Option<String>,
    ) -> Result<(), ParseError> {
        let poly_begin = self.state.geometry_collector.multipolygon.len();
        let (surface_id, _) = self.parse_surface()?;
        let poly_end = self.state.geometry_collector.multipolygon.len();
        if poly_end - poly_begin > 0 {
            geomrefs.push(GeometryRef {
                ty: GeometryType::Surface,
                lod,
                pos: poly_begin as u32,
                len: (poly_end - poly_begin) as u32,
                id: surface_id,
                solid_ids: Vec::new(),
                feature_id,
                feature_type,
            });
        }
        Ok(())
    }

    fn parse_solid_prop(
        &mut self,
        geomrefs: &mut GeometryRefs,
        lod: u8,
        feature_id: Option<String>,
        feature_type: Option<String>,
    ) -> Result<(), ParseError> {
        let poly_begin = self.state.geometry_collector.multipolygon.len();
        let mut surface_id = None;
        let mut solid_ids = Vec::new();

        if expect_start(self.reader, &mut self.state.buf1, GML31_NS, b"Solid")? {
            (surface_id, solid_ids) = self.parse_solid()?;
            expect_end(self.reader, &mut self.state.buf1)?;
        }

        let poly_end = self.state.geometry_collector.multipolygon.len();
        if poly_end - poly_begin > 0 {
            geomrefs.push(GeometryRef {
                ty: GeometryType::Solid,
                lod,
                pos: poly_begin as u32,
                len: (poly_end - poly_begin) as u32,
                id: surface_id,
                solid_ids,
                feature_id,
                feature_type,
            });
        }
        Ok(())
    }

    fn parse_multi_geometry(
        &mut self,
        geomrefs: &mut GeometryRefs,
        lod: u8,
        feature_id: Option<String>,
        feature_type: Option<String>,
    ) -> Result<(), ParseError> {
        let mut inside_member = false;
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());

                    match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"geometryMember") => {
                            inside_member = true;
                            self.parse_geometry_prop(
                                geomrefs,
                                lod,
                                feature_id.clone(),
                                feature_type.clone(),
                            )?;
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected geometry elements <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )))
                        }
                    };
                }
                Ok(Event::End(_)) => {
                    if inside_member {
                        inside_member = false;
                    } else {
                        break;
                    }
                }
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn parse_geometry_prop(
        &mut self,
        geomrefs: &mut GeometryRefs,
        lod: u8,
        feature_id: Option<String>,
        feature_type: Option<String>,
    ) -> Result<(), ParseError> {
        let mut surface_id = None;
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    let poly_begin = self.state.geometry_collector.multipolygon.len();
                    let line_begin = self.state.geometry_collector.multilinestring.len();
                    let mut poly_end: Option<usize> = None;
                    let mut line_end: Option<usize> = None;

                    for attr in start.attributes().flatten() {
                        let (nsres, localname) = self.reader.resolve_attribute(attr.key);
                        // surface id
                        if nsres == Bound(GML31_NS) && localname.as_ref() == b"id" {
                            let id = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            surface_id = Some(LocalId::from(id));
                        }
                        if localname.as_ref() == b"srsName" {
                            self.state.geometry_collector.geometry_crs_uri =
                                Some(String::from_utf8_lossy(attr.value.as_ref()).to_string());
                        }
                    }

                    let geomtype = match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"MultiGeometry") => {
                            self.parse_multi_geometry(
                                geomrefs,
                                lod,
                                feature_id.clone(),
                                feature_type.clone(),
                            )?;
                            return Ok(());
                        }
                        (Bound(GML31_NS), b"Solid") => {
                            self.parse_solid()?;
                            poly_end = Some(self.state.geometry_collector.multipolygon.len());
                            GeometryType::Solid
                        }
                        (Bound(GML31_NS), b"MultiSurface") => {
                            let id = self.parse_multi_surface()?;
                            if surface_id.is_none() {
                                surface_id = id;
                            }
                            poly_end = Some(self.state.geometry_collector.multipolygon.len());
                            GeometryType::Surface
                        }
                        (Bound(GML31_NS), b"CompositeSurface") => {
                            self.parse_composite_surface()?;
                            poly_end = Some(self.state.geometry_collector.multipolygon.len());
                            GeometryType::Surface
                        }
                        (Bound(GML31_NS), b"OrientableSurface") => {
                            let mut orientation = "+".to_string();
                            for attr in start.attributes().flatten() {
                                let (_, localname) = self.reader.resolve_attribute(attr.key);
                                if localname.as_ref() == b"orientation" {
                                    let value =
                                        String::from_utf8_lossy(attr.value.as_ref()).to_string();
                                    orientation = value;
                                    break;
                                }
                            }
                            let base_surface_id = self.parse_orientable_surface()?;
                            if let Some(base_surface_id) = base_surface_id {
                                surface_id = Some(base_surface_id.clone());
                                self.state.geometry_collector.orientable_surfaces.push(
                                    OrientableSurface {
                                        surface_id: base_surface_id,
                                        reverse: orientation == "-",
                                    },
                                );
                            }
                            poly_end = Some(self.state.geometry_collector.multipolygon.len());
                            GeometryType::Surface
                        }
                        (Bound(GML31_NS), b"Polygon") => {
                            self.parse_polygon()?;
                            poly_end = Some(self.state.geometry_collector.multipolygon.len());
                            GeometryType::Surface
                        }
                        (Bound(GML31_NS), b"LineString") => {
                            self.parse_linestring()?;
                            line_end = Some(self.state.geometry_collector.multilinestring.len());
                            GeometryType::Curve
                        }
                        (Bound(GML31_NS), b"MultiCurve") => {
                            self.parse_multi_curve_prop(
                                geomrefs,
                                lod,
                                feature_id.clone(),
                                feature_type.clone(),
                            )?;
                            line_end = Some(self.state.geometry_collector.multilinestring.len());
                            GeometryType::Curve
                        }
                        (Bound(GML31_NS), b"CompositeCurve") => {
                            self.parse_composite_curve_prop(
                                geomrefs,
                                lod,
                                feature_id.clone(),
                                feature_type.clone(),
                            )?;
                            line_end = Some(self.state.geometry_collector.multilinestring.len());
                            GeometryType::Curve
                        }
                        (Bound(GML31_NS), b"TriangulatedSurface") => unimplemented!(),
                        (Bound(GML31_NS), b"Tin") => unimplemented!(),
                        (Bound(GML31_NS), b"Point") => unimplemented!(), // FIXME:
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected geometry elements <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )))
                        }
                    };
                    if let Some(poly_end) = poly_end {
                        if poly_end - poly_begin > 0 {
                            geomrefs.push(GeometryRef {
                                ty: geomtype,
                                lod,
                                pos: poly_begin as u32,
                                len: (poly_end - poly_begin) as u32,
                                id: surface_id.clone(),
                                solid_ids: Vec::new(),
                                feature_id: feature_id.clone(),
                                feature_type: feature_type.clone(),
                            });
                        }
                        // record a partial surface span
                        if let Some(id) = surface_id.clone() {
                            self.state
                                .geometry_collector
                                .surface_spans
                                .push(SurfaceSpan {
                                    id,
                                    start: poly_begin as u32,
                                    end: poly_end as u32,
                                });
                        }
                    }
                    if let Some(line_end) = line_end {
                        if line_end - line_begin > 0 {
                            geomrefs.push(GeometryRef {
                                ty: geomtype,
                                lod,
                                pos: line_begin as u32,
                                len: (line_end - line_begin) as u32,
                                id: surface_id.clone(),
                                solid_ids: Vec::new(),
                                feature_id: feature_id.clone(),
                                feature_type: feature_type.clone(),
                            });
                        }
                    }
                }
                Ok(Event::End(_)) => break,
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn parse_triangulated_prop(
        &mut self,
        geomrefs: &mut GeometryRefs,
        lod: u8,
        feature_id: Option<String>,
        feature_type: Option<String>,
    ) -> Result<(), ParseError> {
        let poly_begin = self.state.geometry_collector.multipolygon.len();

        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"TriangulatedSurface") => {
                            self.parse_triangulated_surface()?
                        }
                        (Bound(GML31_NS), b"Tin") => self.parse_triangulated_surface()?,
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}> by parse triangulated prop",
                                String::from_utf8_lossy(localname.as_ref())
                            )))
                        }
                    }
                }
                Ok(Event::End(_)) => break,
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }

        let poly_end = self.state.geometry_collector.multipolygon.len();
        if poly_end - poly_begin > 0 {
            geomrefs.push(GeometryRef {
                ty: GeometryType::Triangle,
                lod,
                pos: poly_begin as u32,
                len: (poly_end - poly_begin) as u32,
                id: None,
                solid_ids: Vec::new(),
                feature_id,
                feature_type,
            });
        }
        Ok(())
    }

    fn parse_multi_curve(&mut self) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"curveMember") => {
                            self.parse_linestring()?;
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(
                                "Unexpected element. Because only surface member".into(),
                            ))
                        }
                    };
                }
                Ok(Event::End(_)) => return Ok(()),
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn parse_solid(&mut self) -> Result<(Option<LocalId>, Vec<LocalId>), ParseError> {
        if expect_start(self.reader, &mut self.state.buf1, GML31_NS, b"exterior")? {
            let (surface_id, surface_ids) = self.parse_surface()?;
            expect_end(self.reader, &mut self.state.buf1)?;
            return Ok((surface_id, surface_ids));
        }
        Ok((None, Vec::new()))
    }

    fn parse_triangulated_surface(&mut self) -> Result<(), ParseError> {
        if expect_start(
            self.reader,
            &mut self.state.buf1,
            GML31_NS,
            b"trianglePatches",
        )? {
            self.parse_triangle_patch_array()?;
            expect_end(self.reader, &mut self.state.buf1)?;
        }
        Ok(())
    }

    fn parse_triangle_patch_array(&mut self) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"Triangle") => self.parse_polygon()?,
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}> by parsing triangle patch array",
                                String::from_utf8_lossy(localname.as_ref())
                            )))
                        }
                    }
                }
                Ok(Event::End(_)) => return Ok(()),
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn parse_multi_surface(&mut self) -> Result<Option<LocalId>, ParseError> {
        let mut surface_id = None;
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    surface_id = match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"surfaceMember") => {
                            let (surface_id, _) = self.parse_surface()?;
                            surface_id
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(
                                "Unexpected element. Because only surface member".into(),
                            ))
                        }
                    };
                }
                Ok(Event::End(_)) => return Ok(surface_id),
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn parse_composite_surface(&mut self) -> Result<Vec<LocalId>, ParseError> {
        let mut surface_id = None;
        let mut result = Vec::new();
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    for attr in start.attributes().flatten() {
                        let (nsres, localname) = self.reader.resolve_attribute(attr.key);
                        if nsres == Bound(XLINK_NS) && localname.as_ref() == b"href" {
                            let href = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            surface_id = Some(LocalId::from(href));
                            break;
                        }
                    }
                    match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"surfaceMember") => {
                            let (surface_member_id, _) = self.parse_surface()?;
                            if surface_id.is_none() {
                                surface_id = surface_member_id;
                            }
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}> by parsing composite surface",
                                String::from_utf8_lossy(localname.as_ref())
                            )))
                        }
                    };
                    // record a partial surface span
                    if let Some(ref id) = surface_id {
                        result.push(id.clone());
                        self.state
                            .geometry_collector
                            .composite_surfaces
                            .push(id.clone());
                    }
                }
                Ok(Event::End(end)) => {
                    let (nsres, localname) = self.reader.resolve_element(end.name());
                    if nsres == Bound(GML31_NS) && localname.as_ref() == b"CompositeSurface" {
                        return Ok(result);
                    }
                }
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn parse_surface(&mut self) -> Result<(Option<LocalId>, Vec<LocalId>), ParseError> {
        let mut surface_id = None;
        let mut surface_ids = Vec::new();
        let poly_begin = self.state.geometry_collector.multipolygon.len();

        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    // surface id
                    for attr in start.attributes().flatten() {
                        let (nsres, localname) = self.reader.resolve_attribute(attr.key);
                        if nsres == Bound(GML31_NS) && localname.as_ref() == b"id" {
                            let id = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            surface_id = Some(LocalId::from(id));
                            break;
                        }
                    }

                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"Polygon") => self.parse_polygon()?,
                        (Bound(GML31_NS), b"CompositeSurface") => {
                            surface_ids = self.parse_composite_surface()?;
                        }
                        (Bound(GML31_NS), b"OrientableSurface") => {
                            let mut orientation = "+".to_string();
                            for attr in start.attributes().flatten() {
                                let (_, localname) = self.reader.resolve_attribute(attr.key);
                                if localname.as_ref() == b"orientation" {
                                    let value =
                                        String::from_utf8_lossy(attr.value.as_ref()).to_string();
                                    orientation = value;
                                    break;
                                }
                            }
                            let base_surface_id = self.parse_orientable_surface()?;
                            if let Some(base_surface_id) = base_surface_id {
                                surface_id = Some(base_surface_id.clone());
                                self.state.geometry_collector.orientable_surfaces.push(
                                    OrientableSurface {
                                        surface_id: base_surface_id,
                                        reverse: orientation == "-",
                                    },
                                );
                            }
                        }
                        (Bound(GML31_NS), b"surfaceMember") => {
                            (surface_id, _) = self.parse_surface()?;
                        }
                        // (Bound(GML_NS), b"TriangulatedSurface") =>
                        // (Bound(GML_NS), b"Tin") =>
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}> by parsing surface",
                                String::from_utf8_lossy(localname.as_ref())
                            )))
                        }
                    }

                    // record a partial surface span
                    if let Some(id) = surface_id.clone() {
                        let poly_end = self.state.geometry_collector.multipolygon.len() as u32;
                        if poly_end > poly_begin as u32 {
                            self.state
                                .geometry_collector
                                .surface_spans
                                .push(SurfaceSpan {
                                    id,
                                    start: poly_begin as u32,
                                    end: poly_end,
                                });
                        }
                    }
                }
                Ok(Event::End(_)) => return Ok((surface_id, surface_ids)),
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn parse_linestring(&mut self) -> Result<(), ParseError> {
        let mut depth = 1;
        loop {
            match self.reader.read_resolved_event_into(&mut self.state.buf1) {
                Ok((Bound(GML31_NS), Event::Start(_))) => {
                    depth += 1;
                }
                Ok((_, Event::Start(start))) => {
                    return Err(ParseError::SchemaViolation(format!(
                        "Only GML elements are allowed but found <{}>",
                        String::from_utf8_lossy(start.name().as_ref())
                    )));
                }
                Ok((_, Event::End(_))) => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                Ok((_, Event::Text(text))) => {
                    // check poslist
                    if depth != 3 {
                        return Err(ParseError::SchemaViolation(
                            "Unexpected text content".into(),
                        ));
                    }
                    // parse coordinate sequence
                    self.state.fp_buf.clear();
                    let unescaped_text = text
                        .unescape()
                        .map_err(|e| ParseError::InvalidValue(format!("Unescape error: {e}")))?;
                    for s in unescaped_text.split_ascii_whitespace() {
                        if let Ok(v) = s.parse() {
                            self.state.fp_buf.push(v);
                        } else {
                            return Err(ParseError::InvalidValue(format!(
                                "Invalid floating point number: {s}"
                            )));
                        }
                    }

                    if !self.state.fp_buf.len().is_multiple_of(3) {
                        return Err(ParseError::InvalidValue(
                            "Length of coordinate numbers must be multiple of 3".into(),
                        ));
                    }

                    let iter = self
                        .state
                        .fp_buf
                        .chunks_exact(3)
                        .map(|c| [c[0], c[1], c[2]]);

                    self.state.geometry_collector.add_linestring(iter);
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn parse_orientable_surface(&mut self) -> Result<Option<LocalId>, ParseError> {
        let mut surface_id = None;
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML31_NS), b"baseSurface") => {
                            for attr in start.attributes().flatten() {
                                let (nsres, localname) = self.reader.resolve_attribute(attr.key);
                                if nsres == Bound(XLINK_NS) && localname.as_ref() == b"href" {
                                    let href =
                                        String::from_utf8_lossy(attr.value.as_ref()).to_string();
                                    surface_id = Some(LocalId::from(href));
                                    break;
                                }
                            }
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(
                                "Unexpected element. Because only base surface".into(),
                            ))
                        }
                    };
                }
                Ok(Event::End(_)) => return Ok(surface_id),
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn parse_polygon(&mut self) -> Result<(), ParseError> {
        let mut depth = 1;
        let mut expect_exterior = true;
        let mut is_exterior = true;
        let mut ring_id = None;
        loop {
            match self.reader.read_resolved_event_into(&mut self.state.buf1) {
                Ok((Bound(GML31_NS), Event::Start(start))) => {
                    depth += 1;
                    match (depth, start.local_name().as_ref()) {
                        (2, b"exterior") => {
                            if expect_exterior {
                                expect_exterior = false;
                            } else {
                                return Err(ParseError::SchemaViolation(
                                    "Exterior ring is expected only once".into(),
                                ));
                            }
                        }
                        (2, b"interior") => {
                            is_exterior = false;
                            if expect_exterior {
                                return Err(ParseError::SchemaViolation(
                                    "Exterior ring is expected before interior".into(),
                                ));
                            }
                        }
                        (2, _) => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Expected <exterior> or <interior> but found <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )));
                        }
                        (3, b"LinearRing") => {
                            ring_id = None;
                            for attr in start.attributes().flatten() {
                                let (nsres, localname) = self.reader.resolve_attribute(attr.key);
                                if nsres == Bound(GML31_NS) && localname.as_ref() == b"id" {
                                    let id =
                                        String::from_utf8_lossy(attr.value.as_ref()).to_string();
                                    ring_id = Some(LocalId::from(id));
                                    break;
                                }
                            }
                        }
                        (3, _) => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Expected <LinearRing> but found <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )));
                        }
                        (4, b"posList") => (),
                        (4, _) => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Expected <posList> but found <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )));
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Coordinate sequence text is expected but found <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )));
                        }
                    }
                }
                Ok((_, Event::Start(start))) => {
                    return Err(ParseError::SchemaViolation(format!(
                        "Only GML elements are allowed but found <{}>",
                        String::from_utf8_lossy(start.name().as_ref())
                    )));
                }
                Ok((_, Event::End(_))) => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                Ok((_, Event::Text(text))) => {
                    if depth != 4 {
                        return Err(ParseError::SchemaViolation(
                            "Unexpected text content".into(),
                        ));
                    }

                    // parse coordinate sequence
                    self.state.fp_buf.clear();
                    let unescaped_text = text
                        .unescape()
                        .map_err(|e| ParseError::InvalidValue(format!("Unescape error: {e}")))?;
                    for s in unescaped_text.split_ascii_whitespace() {
                        if let Ok(v) = s.parse() {
                            self.state.fp_buf.push(v);
                        } else {
                            return Err(ParseError::InvalidValue(format!(
                                "Invalid floating point number: {s}"
                            )));
                        }
                    }

                    if !self.state.fp_buf.len().is_multiple_of(3) {
                        return Err(ParseError::InvalidValue(
                            "Length of coordinate numbers must be multiple of 3".into(),
                        ));
                    }

                    let iter = self
                        .state
                        .fp_buf
                        .chunks_exact(3)
                        .map(|c| [c[0], c[1], c[2]]);

                    if is_exterior {
                        // add a new polygon
                        self.state
                            .geometry_collector
                            .add_exterior_ring(iter, ring_id.take());
                    } else {
                        // append an interior ring
                        self.state
                            .geometry_collector
                            .add_interior_ring(iter, ring_id.take());
                    }
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
    }

    /// Parses <app:target> of ParameterizedTexture
    pub(crate) fn parse_texture_association(&mut self) -> Result<TextureAssociation, ParseError> {
        // uri attribute (required)
        let mut target = None;
        if let Some(start) = &self.state.current_start {
            for attr in start.attributes().flatten() {
                if attr.key.as_ref() == b"uri" {
                    target = Some(LocalId::parse_attribute_value(
                        std::str::from_utf8(attr.value.as_ref()).unwrap(),
                        &mut self.state.context,
                    )?);
                }
            }
        }

        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(APP_2_NS), b"TexCoordList") => {
                            let mut tex_coords = TexCoordList {
                                target: target.take().unwrap(),
                                ..Default::default()
                            };
                            self.parse_tex_coord_list(&mut tex_coords)?;
                            return Ok(TextureAssociation::TexCoordList(tex_coords));
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "TexCoordList is expected but found <{}>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )))
                        }
                    };
                }
                Ok(Event::End(_)) => break,
                Ok(Event::Text(_)) => {
                    return Err(ParseError::SchemaViolation(
                        "Unexpected text content".into(),
                    ))
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }

        Err(ParseError::SchemaViolation(
            "Expected <app:TexCoordList> or <app:TexCoordGen> but not found".into(),
        ))
    }

    fn parse_tex_coord_list(&mut self, tex_coords: &mut TexCoordList) -> Result<(), ParseError> {
        let mut inside_coordinates = false;
        let mut ring = None;
        let mut coords = None;
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    if inside_coordinates {
                        return Err(ParseError::SchemaViolation(format!(
                            "Unexpected elements <{}> inside <app:textureCoordinates>",
                            String::from_utf8_lossy(start.name().as_ref())
                        )));
                    }

                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(APP_2_NS), b"textureCoordinates") => {
                            inside_coordinates = true;
                            for attr in start.attributes().flatten() {
                                if attr.key.as_ref() == b"ring" {
                                    ring = Some(LocalId::parse_attribute_value(
                                        std::str::from_utf8(attr.value.as_ref()).unwrap(),
                                        &mut self.state.context,
                                    )?);
                                }
                            }

                            if ring.is_none() {
                                return Err(ParseError::SchemaViolation(
                                    "<app:textureCoordinates> must have a 'ring' attribute.".into(),
                                ));
                            }
                        }
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected elements <{}> inside <app:TexCoordList>",
                                String::from_utf8_lossy(start.name().as_ref())
                            )));
                        }
                    };
                }
                Ok(Event::End(_)) => match inside_coordinates {
                    true => {
                        tex_coords.rings.push(ring.take().ok_or_else(|| {
                            ParseError::SchemaViolation(
                                "<app:textureCoordinates> must have a 'ring' attribute.".into(),
                            )
                        })?);
                        let a = coords.take().ok_or_else(|| {
                            ParseError::SchemaViolation(
                                "<app:textureCoordinates> must have a <app:textureCoordinate>."
                                    .into(),
                            )
                        })?;
                        tex_coords.coords_list.push(a);
                        inside_coordinates = false;
                    }
                    false => {
                        break;
                    }
                },
                Ok(Event::Text(text)) => {
                    if !inside_coordinates {
                        return Err(ParseError::SchemaViolation(
                            "Unexpected text content".into(),
                        ));
                    }

                    self.state.fp_buf.clear();
                    for s in text.unescape().unwrap().split_ascii_whitespace() {
                        if let Ok(v) = s.parse() {
                            self.state.fp_buf.push(v);
                        } else {
                            return Err(ParseError::InvalidValue(format!(
                                "Invalid floating point number: {s}"
                            )));
                        }
                    }

                    if !self.state.fp_buf.len().is_multiple_of(2) {
                        return Err(ParseError::InvalidValue(
                            "Length of UV coordinates must be multiple of 2".into(),
                        ));
                    }

                    // remove closing point
                    {
                        let len = self.state.fp_buf.len();
                        if len >= 4
                            && self.state.fp_buf[0] == self.state.fp_buf[len - 2]
                            && self.state.fp_buf[1] == self.state.fp_buf[len - 1]
                        {
                            self.state.fp_buf.pop();
                            self.state.fp_buf.pop();
                        } else {
                            return Err(ParseError::InvalidValue(format!(
                                "The last UV coord must be the same as the first: {:?}",
                                self.state.fp_buf
                            )));
                        }
                    }

                    coords = Some(mem::take(&mut self.state.fp_buf));
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }
}

fn expect_start<R: BufRead>(
    reader: &mut NsReader<R>,
    buf: &mut Vec<u8>,
    expect_ns: Namespace,
    expect_name: &[u8],
) -> Result<bool, ParseError> {
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(start)) => {
                let (nsres, localname) = reader.resolve_element(start.name());
                if nsres == Bound(expect_ns) && localname.as_ref() == expect_name {
                    return Ok(true);
                } else {
                    return Err(ParseError::SchemaViolation(format!(
                        "Expected <{}> but found <{}>",
                        String::from_utf8_lossy(expect_name),
                        String::from_utf8_lossy(localname.as_ref())
                    )));
                }
            }
            Ok(Event::End(_)) => {
                return Ok(false);
            }
            Ok(Event::Text(_)) => {
                return Err(ParseError::SchemaViolation(format!(
                    "start tag <{}> is expected",
                    String::from_utf8_lossy(expect_name)
                )))
            }
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        }
    }
}

fn expect_end<R: BufRead>(reader: &mut NsReader<R>, buf: &mut Vec<u8>) -> Result<(), ParseError> {
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::End(_)) => return Ok(()),
            Ok(Event::Start(_)) => {
                return Err(ParseError::SchemaViolation("End tag is expected".into()))
            }
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(doc: &str, f: impl Fn(&mut SubTreeReader<std::io::Cursor<&str>>)) {
        let mut reader = quick_xml::NsReader::from_reader(std::io::Cursor::new(doc));
        let mut citygml_reader = CityGmlReader::new(ParseContext::default());
        let mut subtree_reader = citygml_reader
            .start_root(&mut reader)
            .expect("Failed to start root");
        f(&mut subtree_reader);
    }

    #[test]
    fn parse_text() {
        parse(
            r#"
            <foo>bar</foo>
        "#,
            |sr| {
                assert_eq!(sr.parse_text().unwrap(), "bar");
            },
        );
    }

    #[test]
    fn parse_text_invalid() {
        parse(
            r#"
            <foo><unexpected></unexpected></foo>
        "#,
            |sr| {
                sr.parse_text().expect_err("error expected");
            },
        );
    }
}
