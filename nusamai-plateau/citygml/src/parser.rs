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
    state: ReaderInternalState,
}

struct ReaderInternalState {
    /// Holds the current path
    path_buf: Vec<u8>,
    /// Stack of indices of slashes '/' in `path_buf`
    path_stack_indices: Vec<usize>,
    /// General purpose byte buffer 1
    buf1: Vec<u8>,
    /// General purpose byte buffer 2
    buf2: Vec<u8>,
    /// Last start event
    current: BytesStart<'static>,
}

impl CityGMLReader {
    pub fn new() -> Self {
        Self {
            state: ReaderInternalState {
                path_buf: Vec::new(),
                path_stack_indices: Vec::new(),
                buf1: Vec::new(),
                buf2: Vec::new(),
                current: BytesStart::new(""),
            },
        }
    }

    pub fn start<'a, R: BufRead>(
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
                    state.path_buf.push(b':');
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
            b"http://www.opengis.net/gml" => b"gml",
            b"http://www.opengis.net/citygml/2.0" => b"core",
            b"http://www.opengis.net/citygml/building/2.0" => b"bldg",
            // ... and more
            b"http://www.opengis.net/citygml/appearance/2.0" => b"app",
            b"https://www.geospatial.jp/iur/uro/3.0" => b"uro",
            b"https://www.geospatial.jp/iur/uro/2.0" => b"uro",
            _ => name,
        },
        ResolveResult::Unbound => b"",
        ResolveResult::Unknown(_name) => b"unknown",
    }
}

pub struct SubTreeReader<'a, R> {
    reader: &'a mut quick_xml::NsReader<R>,
    state: &'a mut ReaderInternalState,
    path_head: usize,
}

impl<R: BufRead> SubTreeReader<'_, R> {
    pub fn expect_text(&mut self) -> Result<&str, ParseError> {
        self.state.buf2.clear();
        loop {
            let ev = self.reader.read_event_into(&mut self.state.buf1);
            match ev {
                Ok(Event::Start(e) | Event::Empty(e)) => {
                    return Err(ParseError::SchemaViolation(format!(
                        "Text content is expected, but found <{}>.",
                        std::str::from_utf8(e.local_name().as_ref()).unwrap()
                    )));
                }
                Ok(Event::Eof) => return Err(ParseError::SchemaViolation("Unexpected EOF".into())),
                Ok(Event::Text(e)) => {
                    // buf.push_str(std::str::from_utf8(e.as_ref()).unwrap());
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

    pub fn parse_attrs(&mut self, f: impl Fn(&[u8])) {
        for attr in self.state.current.attributes().flatten() {
            let (nsres, localname) = self.reader.resolve_attribute(attr.key);
            let ns = normalize_ns_prefix(&nsres);
            self.state.buf2.clear();
            self.state.buf2.extend(ns);
            self.state.buf2.push(b':');
            self.state.buf2.extend(localname.as_ref());
            f(self.state.buf2.as_ref());
        }
    }

    pub fn get_next(&mut self) -> Result<Option<&[u8]>, ParseError> {
        loop {
            let ev = self.reader.read_event_into(&mut self.state.buf1);
            match ev {
                Ok(Event::Start(e)) => {
                    let (nsres, localname) = self.reader.resolve_element(e.name());
                    let ns = normalize_ns_prefix(&nsres);
                    match localname.as_ref() {
                        b"lod0RoofEdge" | b"lod0FootPrint" | b"lod1Solid" | b"lod2Solid"
                        | b"lod3Solid" | b"lod4Solid" | b"lod1Geometry" | b"lod2Geometry"
                        | b"lod3Geometry" | b"lod4Geometry" | b"lod2MultiSurface"
                        | b"lod3MultiSurface" | b"lod4MultiSurface" | b"appearanceMember" => {
                            // NOTE: Skip geometric and appearance attributes for now to skip unnecessary traversal.
                            self.reader
                                .read_to_end_into(e.name(), &mut self.state.buf2)
                                .unwrap();
                        }
                        _ => {
                            // Append "/{ns_prefix}:{localname}"
                            self.state
                                .path_stack_indices
                                .push(self.state.path_buf.len());
                            self.state.path_buf.push(b'/');
                            self.state.path_buf.extend(ns);
                            self.state.path_buf.push(b':');
                            self.state.path_buf.extend(localname.as_ref());

                            self.state.current = e.into_owned();
                            return Ok(Some(&self.state.path_buf[self.path_head + 1..]));
                        }
                    }
                }
                Ok(Event::Empty(_e)) => {
                    return Err(ParseError::SchemaViolation(
                        "Empty element is not allowed.".into(),
                    ));
                }
                Ok(Event::End(_)) => {
                    self.state
                        .path_buf
                        .truncate(self.state.path_stack_indices.pop().unwrap());
                    if self.state.path_buf.len() < self.path_head {
                        return Ok(None);
                    }
                }
                Ok(Event::Eof) => return Ok(None),
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }
    }

    pub fn parse_children(&mut self, f: impl Fn(&[u8])) -> Result<(), ParseError> {
        loop {
            let ev = self.reader.read_event_into(&mut self.state.buf1);
            match ev {
                Ok(Event::Start(e)) => {
                    let (nsres, localname) = self.reader.resolve_element(e.name());
                    let ns = normalize_ns_prefix(&nsres);
                    match localname.as_ref() {
                        b"lod0RoofEdge" | b"lod0FootPrint" | b"lod1Solid" | b"lod2Solid"
                        | b"lod3Solid" | b"lod4Solid" | b"lod1Geometry" | b"lod2Geometry"
                        | b"lod3Geometry" | b"lod4Geometry" | b"lod2MultiSurface"
                        | b"lod3MultiSurface" | b"lod4MultiSurface" | b"appearanceMember" => {
                            // NOTE: Skip geometric and appearance attributes for now to skip unnecessary traversal.
                            self.reader
                                .read_to_end_into(e.name(), &mut self.state.buf2)
                                .unwrap();
                        }
                        _ => {
                            // Append "/{ns_prefix}:{localname}"
                            self.state
                                .path_stack_indices
                                .push(self.state.path_buf.len());
                            self.state.path_buf.push(b'/');
                            self.state.path_buf.extend(ns);
                            self.state.path_buf.push(b':');
                            self.state.path_buf.extend(localname.as_ref());

                            self.state.current = e.into_owned();
                            f(&self.state.path_buf[self.path_head + 1..]);
                        }
                    }
                }
                Ok(Event::Empty(_e)) => {
                    return Err(ParseError::SchemaViolation(
                        "Empty element is not allowed.".into(),
                    ));
                }
                Ok(Event::End(_)) => {
                    self.state
                        .path_buf
                        .truncate(self.state.path_stack_indices.pop().unwrap());
                    if self.state.path_buf.len() < self.path_head {
                        return Ok(());
                    }
                }
                Ok(Event::Eof) => return Ok(()),
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }
    }

    pub fn start_new_subtree(&mut self) -> SubTreeReader<R> {
        let head = self.state.path_buf.len();
        SubTreeReader {
            reader: self.reader,
            state: self.state,
            path_head: head,
            //current: self.current.to_owned(),
        }
    }
}
