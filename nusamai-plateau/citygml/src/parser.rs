use crate::namespace::normalize_ns_prefix;
use quick_xml::events::{BytesStart, Event};
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
    #[inline]
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
    ) -> Result<SubTreeReader<R>, ParseError> {
        let state = &mut self.state;
        reader.trim_text(true);
        reader.expand_empty_elements(true);

        loop {
            match reader.read_event_into(&mut state.buf1) {
                Ok(Event::Start(e)) => {
                    let (nsres, localname) = reader.resolve_element(e.name());
                    let ns = normalize_ns_prefix(&nsres);
                    state.path_stack_indices.push(state.path_buf.len());
                    state.path_buf.push(b'/');
                    state.path_buf.extend(ns);
                    state.path_buf.extend(localname.as_ref());

                    return Ok(SubTreeReader {
                        reader,
                        state: &mut self.state,
                        path_head: 0,
                    });
                }
                Ok(Event::Eof) => {
                    return Err(ParseError::XmlError(quick_xml::Error::UnexpectedEof(
                        "Unexpected EOF".into(),
                    )));
                }
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
            st.state.current = None;
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

    #[inline]
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
        self.state.current = None;
        loop {
            match self.reader.read_event_into(&mut self.state.buf1) {
                Ok(Event::Start(e)) => {
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

    #[inline]
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

    #[inline]
    pub fn current_path(&self) -> &[u8] {
        &self.state.path_buf[self.path_head + 1..]
    }
}
