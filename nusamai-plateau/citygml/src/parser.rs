use quick_xml::{
    events::{BytesStart, Event},
    name::{Namespace, ResolveResult},
};
use std::io::BufRead;
use thiserror::Error;

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

struct InternalState {
    /// Buffer holding the current path
    path_buf: Vec<u8>,
    /// Stack of indices of slashes '/' in `path_buf`
    path_stack_indices: Vec<usize>,
    /// General purpose byte buffer 1
    buf1: Vec<u8>,
    /// General purpose byte buffer 2
    buf2: Vec<u8>,
    /// Data of last start tag
    current: Option<BytesStart<'static>>,
}

impl CityGMLReader {
    pub fn new() -> Self {
        Self {
            state: InternalState {
                path_buf: Vec::new(),
                path_stack_indices: Vec::new(),
                buf1: Vec::new(),
                buf2: Vec::new(),
                current: None,
            },
        }
    }

    pub fn start_root<'a, R: BufRead>(
        &'a mut self,
        reader: &'a mut quick_xml::NsReader<R>,
    ) -> Result<Option<SubTreeReader<R>>, ParseError> {
        let state = &mut self.state;
        reader.trim_text(true);
        loop {
            match reader.read_event_into(&mut state.buf1) {
                Ok(Event::Start(e)) => {
                    let (nsres, localname) = reader.resolve_element(e.name());
                    let ns = normalize_ns_prefix(&nsres);
                    state.path_stack_indices.push(state.path_buf.len());
                    state.path_buf.push(b'/');
                    state.path_buf.extend(ns);
                    state.path_buf.extend(localname.as_ref());

                    return Ok(Some(SubTreeReader {
                        reader,
                        state: &mut self.state,
                        path_head: 0,
                        // current: e.into_owned(),
                    }));
                }
                Ok(Event::Eof) => return Ok(None),
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }
    }
}

impl Default for CityGMLReader {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_ns_prefix<'a>(ns: &ResolveResult<'a>) -> &'a [u8] {
    match ns {
        ResolveResult::Bound(Namespace(name)) => match *name {
            // GML
            b"http://www.opengis.net/gml" => b"gml:",
            // CityGML
            b"http://www.opengis.net/citygml/2.0" => b"core:",
            b"http://www.opengis.net/citygml/appearance/2.0" => b"app:",
            b"http://www.opengis.net/citygml/building/2.0" => b"bldg:",
            b"http://www.opengis.net/citygml/bridge/2.0" => b"brid:",
            b"http://www.opengis.net/citygml/relief/2.0" => b"dem:",
            b"http://www.opengis.net/citygml/cityfurniture/2.0" => b"frn:",
            b"http://www.opengis.net/citygml/generics/2.0" => b"gen:",
            b"http://www.opengis.net/citygml/cityobjectgroup/2.0" => b"grp:",
            b"http://www.opengis.net/citygml/landuse/2.0" => b"luse:",
            b"http://www.opengis.net/citygml/transportation/2.0" => b"tran:",
            b"http://www.opengis.net/citygml/vegetation/2.0" => b"veg:",
            b"http://www.opengis.net/citygml/waterbody/2.0" => b"wtr:",
            b"http://www.opengis.net/citygml/tunnel/2.0" => b"tun:",
            // PLATEAU 3.x
            b"https://www.geospatial.jp/iur/uro/3.0" => b"uro:",
            b"https://www.geospatial.jp/iur/urf/3.0" => b"urf:",
            // PLATEAU 2.x
            b"https://www.geospatial.jp/iur/uro/2.0" => b"uro:",
            b"https://www.geospatial.jp/iur/urf/2.0" => b"urf:",
            // PLATEAU 1.x
            b"https://www.chisou.go.jp/tiiki/toshisaisei/itoshisaisei/iur/uro/1.5" => b"uro:",
            b"https://www.chisou.go.jp/tiiki/toshisaisei/itoshisaisei/iur/urf/1.5" => b"urf:",
            b"http://www.kantei.go.jp/jp/singi/tiiki/toshisaisei/itoshisaisei/iur/uro/1.4" => {
                b"uro:"
            }
            b"http://www.kantei.go.jp/jp/singi/tiiki/toshisaisei/itoshisaisei/iur/urf/1.4" => {
                b"urf:"
            }
            _ => name,
        },
        ResolveResult::Unbound => b"",
        ResolveResult::Unknown(_name) => b"unknown:",
    }
}

pub struct SubTreeReader<'a, R> {
    reader: &'a mut quick_xml::NsReader<R>,
    state: &'a mut InternalState,
    path_head: usize,
}

impl<R: BufRead> SubTreeReader<'_, R> {
    pub fn parse_children(
        &mut self,
        mut logic: impl FnMut(&mut SubTreeReader<R>) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        // spawn new subtree reader
        let mut st = SubTreeReader {
            path_head: self.state.path_buf.len(),
            reader: self.reader,
            state: self.state,
        };

        loop {
            match st.reader.read_event_into(&mut st.state.buf1) {
                Ok(Event::Start(start)) => {
                    let (nsres, localname) = st.reader.resolve_element(start.name());
                    let ns = normalize_ns_prefix(&nsres);

                    // Append "/{ns_prefix}:{localname}" to the path stack
                    st.state.path_stack_indices.push(st.state.path_buf.len());
                    st.state.path_buf.push(b'/');
                    st.state.path_buf.extend(ns);
                    st.state.path_buf.extend(localname.as_ref());

                    if localname.as_ref().starts_with(b"lod")
                        || localname.as_ref() == b"appearanceMember"
                    {
                        // NOTE: Skip geometric elements for now to avoid unnecessary heavy traversal.
                        st.state.current.clone_from(&Some(start.into_owned()));
                        st.skip_current_element()?;
                    } else {
                        // call the privided processing logic
                        st.state.current.clone_from(&Some(start.into_owned()));
                        logic(&mut st)?;
                    }
                }
                Ok(Event::Empty(_e)) => {
                    st.state.current = None;
                    return Err(ParseError::SchemaViolation(
                        "Empty element is not allowed.".into(),
                    ));
                }
                Ok(Event::End(_)) => {
                    st.state
                        .path_buf
                        .truncate(st.state.path_stack_indices.pop().unwrap());
                    if st.state.path_buf.len() < st.path_head {
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
        let Some(start) = &self.state.current else {
            panic!("parse_attributes() must be called immediately after encountering a new starting tag.");
        };
        self.state.buf2.clear();
        self.state.buf2.push(b'@');
        for attr in start.attributes().flatten() {
            let (nsres, localname) = self.reader.resolve_attribute(attr.key);
            let ns = normalize_ns_prefix(&nsres);
            self.state.buf2.extend(ns);
            self.state.buf2.extend(localname.as_ref());
            logic(self.state.buf2.as_ref(), attr.value.as_ref())?;
            self.state.buf2.truncate(1);
        }
        Ok(())
    }

    /// Expect a XML text content and return it.
    pub fn expect_text(&mut self) -> Result<&str, ParseError> {
        self.state.buf2.clear();
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(e) | Event::Empty(e)) => {
                    return Err(ParseError::SchemaViolation(format!(
                        "Text content is expected, but a <{}> element is found.",
                        std::str::from_utf8(e.local_name().as_ref()).unwrap()
                    )));
                }
                Ok(Event::Eof) => return Err(ParseError::SchemaViolation("Unexpected EOF".into())),
                Ok(Event::Text(e)) => {
                    self.state.buf2.extend(e.as_ref());
                }
                Ok(Event::End(_)) => {
                    self.state
                        .path_buf
                        .truncate(self.state.path_stack_indices.pop().unwrap());
                    return Ok(std::str::from_utf8(self.state.buf2.as_ref()).unwrap());
                }
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }
    }

    pub fn skip_current_element(&mut self) -> Result<(), ParseError> {
        let Some(start) = &self.state.current else {
            panic!("skip_current_element() must be called immediately after encountering a new starting tag.");
        };
        self.reader
            .read_to_end_into(start.name(), &mut self.state.buf2)?;
        self.state
            .path_buf
            .truncate(self.state.path_stack_indices.pop().unwrap());
        self.state.current = None;
        Ok(())
    }

    pub fn current_path(&self) -> &[u8] {
        &self.state.path_buf[self.path_head + 1..]
    }
}
