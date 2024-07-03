use std::io::BufRead;

use hashbrown::HashMap;
use nusamai_citygml::{namespace::GML31_NS, ParseError};
use quick_xml::{events::Event, name::ResolveResult::Bound};

#[derive(Debug)]
pub struct Definition {
    value: String,
}

impl Definition {
    pub fn value(&self) -> &str {
        &self.value
    }
}

fn expect_text<R: BufRead>(
    reader: &mut quick_xml::NsReader<R>,
    buf: &mut Vec<u8>,
) -> Result<String, ParseError> {
    let mut s = String::new();
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(start)) => {
                return Err(ParseError::SchemaViolation(format!(
                    "Text content is expected, but a <{}> is found.",
                    String::from_utf8_lossy(start.local_name().as_ref())
                )));
            }
            Ok(Event::Text(text)) => {
                s.push_str(std::str::from_utf8(text.as_ref()).map_err(|e| {
                    ParseError::InvalidValue(format!("Invalid UTF-8 found: {:?}", e))
                })?);
            }
            Ok(Event::End(_)) => {
                if s.is_empty() {
                    return Err(ParseError::SchemaViolation(
                        "Text content is expected, but an empty string is found.".into(),
                    ));
                }
                return Ok(s);
            }
            Err(e) => return Err(e.into()),
            _ => (),
        }
    }
}

fn parse_definition<R: BufRead>(
    reader: &mut quick_xml::NsReader<R>,
    definitions: &mut HashMap<String, Definition>,
    buf: &mut Vec<u8>,
    buf2: &mut Vec<u8>,
) -> Result<(), ParseError> {
    let mut depth = 1;
    let mut identifier = None;
    let mut value = None;
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(start)) => {
                depth += 1;
                let (nsres, localname) = reader.resolve_element(start.name());
                match (depth, nsres, localname.as_ref()) {
                    (2, Bound(GML31_NS), b"name") => {
                        identifier = Some(expect_text(reader, buf)?);
                        depth -= 1;
                    }
                    (2, Bound(GML31_NS), b"description") => {
                        value = Some(expect_text(reader, buf)?);
                        depth -= 1;
                    }
                    _ => {
                        reader.read_to_end_into(start.name(), buf2)?;
                        depth -= 1;
                    }
                }
            }
            Ok(Event::End(_)) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Ok(Event::Eof) => {
                return Err(ParseError::SchemaViolation(
                    "Unexpected EOF while parsing a definition.".into(),
                ));
            }
            Ok(_) => {}
            Err(e) => return Err(ParseError::XmlError(e)),
        }
    }

    match (identifier, value) {
        (Some(key), Some(value)) => {
            definitions.insert(key, Definition { value });
            Ok(())
        }
        _ => Err(ParseError::SchemaViolation(
            "Both <name> and <description> are required in a code definition.".into(),
        )),
    }
}

pub fn parse_dictionary<R: BufRead>(
    src_reader: R,
) -> Result<HashMap<String, Definition>, ParseError> {
    let mut reader = quick_xml::NsReader::from_reader(src_reader);
    reader.config_mut().trim_text(true);
    reader.config_mut().expand_empty_elements = true;
    let mut depth = 0;
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();
    let mut definitions = HashMap::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(start)) => {
                depth += 1;
                let (nsres, localname) = reader.resolve_element(start.name());
                match (depth, nsres, localname.as_ref()) {
                    (1, Bound(GML31_NS), b"Dictionary") => {}
                    (1, _, _) => {
                        return Err(ParseError::SchemaViolation(format!(
                            "<Dictionary> is expected, but found {}",
                            String::from_utf8_lossy(localname.as_ref())
                        )))
                    }
                    (2, Bound(GML31_NS), b"name") => {
                        // Just ignore it for now.
                        let _name = expect_text(&mut reader, &mut buf)?;
                        depth -= 1;
                    }
                    (2, Bound(GML31_NS), b"dictionaryEntry") => {}
                    (3, Bound(GML31_NS), b"Definition") => {
                        parse_definition(&mut reader, &mut definitions, &mut buf, &mut buf2)?;
                        depth -= 1;
                    }
                    _ => {
                        reader.read_to_end_into(start.name(), &mut buf2)?;
                        depth -= 1;
                    }
                }
            }
            Ok(Event::End(_)) => {
                depth -= 1;
            }
            Ok(Event::Eof) => {
                return Ok(definitions);
            }
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn parse_example() {
        const EXAMPLE_1: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <gml:Dictionary xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:gml="http://www.opengis.net/gml" xsi:schemaLocation="http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/profiles/SimpleDictionary/1.0.0/gmlSimpleDictionaryProfile.xsd" gml:id="xxxxx">
                <gml:name>Test_test</gml:name>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id1">
                        <gml:description>業務施設</gml:description>
                        <gml:name>401</gml:name>
                        <gml:unsupportedTag>401</gml:unsupportedTag>
                        <gml:unsupportedTag>401</gml:unsupportedTag>
                        <gml:unsupportedTag>401</gml:unsupportedTag>
                    </gml:Definition>
                </gml:dictionaryEntry>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id2">
                        <gml:description>商業施設</gml:description>
                        <gml:name>402</gml:name>
                    </gml:Definition>
                </gml:dictionaryEntry>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id5">
                        <gml:description>住宅</gml:description>
                        <gml:name>411</gml:name>
                    </gml:Definition>
                </gml:dictionaryEntry>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id17">
                        <gml:description>その他</gml:description>
                        <gml:name>454</gml:name>
                    </gml:Definition>
                </gml:dictionaryEntry>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id18">
                        <gml:description>不明</gml:description>
                        <gml:name>461</gml:name>
                    </gml:Definition>
                </gml:dictionaryEntry>
            </gml:Dictionary>
        "#;

        let reader = BufReader::new(std::io::Cursor::new(EXAMPLE_1));
        let definitions = parse_dictionary(reader).unwrap();
        assert_eq!(definitions.len(), 5);
        assert_eq!(definitions.get("401").unwrap().value(), "業務施設");
        assert_eq!(definitions.get("454").unwrap().value(), "その他");
        assert_eq!(definitions.get("461").unwrap().value(), "不明");

        // Should not be found
        assert!(definitions.get("123").is_none()); // not exists
        assert!(definitions.get("不明").is_none()); // not exists
        assert!(definitions.get("業務施設").is_none()); // not exists
    }

    #[test]
    fn broken_1() {
        // lack of name tag
        const BROKEN: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <gml:Dictionary xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:gml="http://www.opengis.net/gml" xsi:schemaLocation="http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/profiles/SimpleDictionary/1.0.0/gmlSimpleDictionaryProfile.xsd" gml:id="xxxxx">
                <gml:name>Test_test</gml:name>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id1">
                        <gml:description>業務施設</gml:description>
                    </gml:Definition>
                </gml:dictionaryEntry>
            </gml:Dictionary>
        "#;

        let reader = BufReader::new(std::io::Cursor::new(BROKEN));
        parse_dictionary(reader).unwrap_err();
    }

    #[test]
    fn broken_2() {
        // lack of text content
        const BROKEN: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <gml:Dictionary xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:gml="http://www.opengis.net/gml" xsi:schemaLocation="http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/profiles/SimpleDictionary/1.0.0/gmlSimpleDictionaryProfile.xsd" gml:id="xxxxx">
                <gml:name>Test_test</gml:name>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id1">
                        <gml:description>業務施設</gml:description>
                        <gml:name></gml:name>
                    </gml:Definition>
                </gml:dictionaryEntry>
            </gml:Dictionary>
        "#;
        let reader = BufReader::new(std::io::Cursor::new(BROKEN));
        parse_dictionary(reader).unwrap_err();
    }

    #[test]
    fn broken_3() {
        // gml:description should only have text content
        const BROKEN: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <gml:Dictionary xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:gml="http://www.opengis.net/gml" xsi:schemaLocation="http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/profiles/SimpleDictionary/1.0.0/gmlSimpleDictionaryProfile.xsd" gml:id="xxxxx">
                <gml:name>Test_test</gml:name>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id1">
                        <gml:description><gml:name></gml:name>業務施設</gml:description>
                        <gml:name>1234</gml:name>
                    </gml:Definition>
                </gml:dictionaryEntry>
            </gml:Dictionary>
        "#;
        let reader = BufReader::new(std::io::Cursor::new(BROKEN));
        parse_dictionary(reader).unwrap_err();
    }

    #[test]
    fn broken_4() {
        // invalid root node
        const BROKEN: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <gml:FooBar1234 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:gml="http://www.opengis.net/gml" xsi:schemaLocation="http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/profiles/SimpleDictionary/1.0.0/gmlSimpleDictionaryProfile.xsd" gml:id="xxxxx">
                <gml:name>Test_test</gml:name>
                <gml:dictionaryEntry>
                    <gml:Definition gml:id="id1">
                        <gml:description>業務施設</gml:description>
                        <gml:name>1234</gml:name>
                    </gml:Definition>
                </gml:dictionaryEntry>
            </gml:FooBar1234>
        "#;
        let reader = BufReader::new(std::io::Cursor::new(BROKEN));
        parse_dictionary(reader).unwrap_err();
    }
}
