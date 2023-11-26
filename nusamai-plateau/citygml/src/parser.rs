use crate::geometric::{Geometries, GeometryCollector, GeometryReference, GeometryType};
use crate::namespace::normalize_ns_prefix;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::Namespace;
use quick_xml::name::ResolveResult::Bound;
use quick_xml::NsReader;
use std::io::BufRead;
use std::str;
use thiserror::Error;

const GML_NS: Namespace = Namespace(b"http://www.opengis.net/gml");

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Broken XML: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("Invalid structure: {0}")]
    SchemaViolation(String),
    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

pub struct CityGMLReader {
    state: InternalState,
}

#[derive(Default)]
struct InternalState {
    /// Buffer holding the current path
    path_buf: Vec<u8>,
    /// Stack of indices of slashes '/' in `path_buf`
    path_stack_indices: Vec<usize>,
    /// General purpose buffer 1
    buf1: Vec<u8>,
    /// General purpose buffer 2
    buf2: Vec<u8>,
    /// Data of last start tag
    fp_buf: Vec<f64>,
    /// Data of last start tag
    current_start: Option<BytesStart<'static>>,
    /// Current geometry store
    geometry_collector: GeometryCollector,
}

impl CityGMLReader {
    #[inline]
    pub fn new() -> Self {
        Self {
            state: InternalState::default(),
        }
    }

    pub fn start_root<'a, R: BufRead>(
        &'a mut self,
        reader: &'a mut quick_xml::NsReader<R>,
    ) -> Result<SubTreeReader<R>, ParseError> {
        reader.trim_text(true);
        reader.expand_empty_elements(true);

        let state = &mut self.state;
        loop {
            match reader.read_event_into(&mut state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = reader.resolve_element(start.name());
                    state.path_stack_indices.push(state.path_buf.len());
                    state.path_buf.push(b'/');
                    state.path_buf.extend(normalize_ns_prefix(&nsres));
                    state.path_buf.extend(localname.as_ref());
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

impl Default for CityGMLReader {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SubTreeReader<'a, R> {
    reader: &'a mut quick_xml::NsReader<R>,
    state: &'a mut InternalState,
    path_start: usize,
}

impl<R: BufRead> SubTreeReader<'_, R> {
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
                    let ns = normalize_ns_prefix(&nsres);

                    // Append "/{ns_prefix}:{localname}" to the path stack
                    self.state
                        .path_stack_indices
                        .push(self.state.path_buf.len());
                    self.state.path_buf.push(b'/');
                    self.state.path_buf.extend(ns);
                    self.state.path_buf.extend(localname.as_ref());

                    // save start tag
                    self.state
                        .current_start
                        .clone_from(&Some(start.into_owned()));

                    logic(self)?;
                }
                Ok(Event::End(_)) => {
                    self.state
                        .path_buf
                        .truncate(self.state.path_stack_indices.pop().unwrap());
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
        mut logic: impl FnMut(&[u8], &[u8]) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let Some(start) = &self.state.current_start else {
            panic!("parse_attributes() must be called immediately after encountering a start tag.");
        };
        self.state.buf1.clear();
        self.state.buf1.push(b'@');
        for attr in start.attributes().flatten() {
            let (nsres, localname) = self.reader.resolve_attribute(attr.key);
            self.state.buf1.extend(normalize_ns_prefix(&nsres));
            self.state.buf1.extend(localname.as_ref());
            logic(
                self.state.buf1.as_ref(), // attribute path "@nsprefix:name"
                attr.value.as_ref(),      // attribute value
            )?;
            self.state.buf1.truncate(1);
        }
        Ok(())
    }

    pub fn skip_current_element(&mut self) -> Result<(), ParseError> {
        let Some(start) = &self.state.current_start else {
            panic!("skip_current_element() must be called immediately after encountering a new starting tag.");
        };
        self.reader
            .read_to_end_into(start.name(), &mut self.state.buf1)?;
        self.state
            .path_buf
            .truncate(self.state.path_stack_indices.pop().unwrap());
        self.state.current_start = None;
        Ok(())
    }

    pub fn current_path(&self) -> &[u8] {
        &self.state.path_buf[self.path_start + 1..]
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
                        .map_err(|e| ParseError::InvalidValue(format!("Invalid UTF-8: {}", e)));
                }
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }
    }

    pub fn collect_geometries(&mut self) -> Geometries {
        let geometries: Geometries = self.state.geometry_collector.to_geometries();
        self.state.geometry_collector.clear();
        geometries
    }

    /// Expect a geometric attribute of CityGML
    pub fn parse_geometric_attr(
        &mut self,
        geomref: &mut GeometryReference,
        lod: u8,
        geomtype: GeometryType,
    ) -> Result<(), ParseError> {
        use GeometryType::*;
        match (lod, geomtype) {
            (1, Solid) => self.parse_solid_prop(&mut geomref.lod1_surfaces)?,
            (1, MultiSurface) => self.parse_multi_surface_prop(&mut geomref.lod1_surfaces)?,
            (2, MultiSurface) => self.parse_multi_surface_prop(&mut geomref.lod2_surfaces)?,
            (3, MultiSurface) => self.parse_multi_surface_prop(&mut geomref.lod3_surfaces)?,
            (4, MultiSurface) => self.parse_multi_surface_prop(&mut geomref.lod4_surfaces)?,
            (0, Geometry) => self.parse_geometry_prop(&mut geomref.lod0_surfaces)?, // FIXME: not only surfaces
            (2, Geometry) => self.parse_geometry_prop(&mut geomref.lod2_surfaces)?, // FIXME: not only surfaces
            (3, Geometry) => self.parse_geometry_prop(&mut geomref.lod3_surfaces)?, // FIXME: not only surfaces
            (4, Geometry) => self.parse_geometry_prop(&mut geomref.lod4_surfaces)?, // FIXME: not only surfaces
            (1, Triangulated) => self.parse_triangulated_prop(&mut geomref.lod1_surfaces)?, // FIXME
            _ => {
                return Err(ParseError::SchemaViolation(format!(
                    "Unsupported geometry type: lod={}, geomtype={:?}",
                    lod, &geomtype
                )));
            }
        }
        self.state
            .path_buf
            .truncate(self.state.path_stack_indices.pop().unwrap());
        Ok(())
    }

    fn parse_multi_surface_prop(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        if expect_start(self.reader, &mut self.state.buf1, GML_NS, b"MultiSurface")? {
            self.parse_multi_surface(geomidx)?;
            expect_end(self.reader, &mut self.state.buf1)?;
        }
        Ok(())
    }

    fn parse_solid_prop(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        if expect_start(self.reader, &mut self.state.buf1, GML_NS, b"Solid")? {
            self.parse_solid(geomidx)?;
            expect_end(self.reader, &mut self.state.buf1)?;
        }
        Ok(())
    }

    fn parse_geometry_prop(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML_NS), b"Solid") => self.parse_solid(geomidx)?,
                        (Bound(GML_NS), b"MultiSurface") => self.parse_multi_surface(geomidx)?,
                        (Bound(GML_NS), b"CompositeSurface") => {
                            self.parse_composite_surface(geomidx)?
                        }
                        // (Bound(GML_NS), b"OrientableSurface") => ...
                        // (Bound(GML_NS), b"Polygon") => ...
                        // (Bound(GML_NS), b"TriangulatedSurface") => ...
                        // (Bound(GML_NS), b"Tin") => ...
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}>",
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

    fn parse_triangulated_prop(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML_NS), b"TriangulatedSurface") => {
                            self.parse_triangulated_surface(geomidx)?
                        }
                        (Bound(GML_NS), b"Tin") => self.parse_triangulated_surface(geomidx)?,
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}>",
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

    fn parse_solid(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        if expect_start(self.reader, &mut self.state.buf1, GML_NS, b"exterior")? {
            self.parse_surface_prop(geomidx)?;
            expect_end(self.reader, &mut self.state.buf1)?;
        }
        Ok(())
    }

    fn parse_triangulated_surface(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        if expect_start(
            self.reader,
            &mut self.state.buf1,
            GML_NS,
            b"trianglePatches",
        )? {
            self.parse_triangle_patch_array(geomidx)?;
            expect_end(self.reader, &mut self.state.buf1)?;
        }
        Ok(())
    }

    fn parse_triangle_patch_array(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML_NS), b"Triangle") => self.parse_polygon(geomidx)?,
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}>",
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

    fn parse_multi_surface(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML_NS), b"surfaceMember") => self.parse_surface_prop(geomidx)?,
                        _ => return Err(ParseError::SchemaViolation("Unexpected element".into())),
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

    fn parse_composite_surface(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML_NS), b"surfaceMember") => self.parse_surface_prop(geomidx)?,
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}>",
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

    fn parse_surface_prop(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = self.reader.resolve_element(start.name());
                    match (nsres, localname.as_ref()) {
                        (Bound(GML_NS), b"Polygon") => self.parse_polygon(geomidx)?,
                        (Bound(GML_NS), b"CompositeSurface") => {
                            self.parse_composite_surface(geomidx)?
                        }
                        // (Bound(GML_NS), b"OrientableSurface") =>
                        // (Bound(GML_NS), b"TriangulatedSurface") =>
                        // (Bound(GML_NS), b"Tin") =>
                        _ => {
                            return Err(ParseError::SchemaViolation(format!(
                                "Unexpected element <{}>",
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

    fn parse_polygon(&mut self, geomidx: &mut Vec<u32>) -> Result<(), ParseError> {
        let mut depth = 1;
        let mut expect_exterior = true;
        let mut is_exterior = true;
        loop {
            match self.reader.read_resolved_event_into(&mut self.state.buf1) {
                Ok((Bound(GML_NS), Event::Start(start))) => {
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
                        (3, b"LinearRing") => (),
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
                    for s in text.unescape().unwrap().split_ascii_whitespace() {
                        if let Ok(v) = s.parse() {
                            self.state.fp_buf.push(v);
                        } else {
                            return Err(ParseError::InvalidValue(format!(
                                "Invalid floating point number: {}",
                                s
                            )));
                        }
                    }

                    if self.state.fp_buf.len() % 3 != 0 {
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
                        geomidx.push(self.state.geometry_collector.add_exterior_ring(iter) as u32);
                    } else {
                        // append an interior ring
                        self.state.geometry_collector.add_interior_ring(iter);
                    }
                }
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
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
