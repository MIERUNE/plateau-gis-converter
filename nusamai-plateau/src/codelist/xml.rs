use std::collections::HashMap;
use std::io::BufRead;

use citygml::ParseError;
use quick_xml::events::Event;
use quick_xml::name::Namespace;
use quick_xml::name::ResolveResult::Bound;

const GML_NS: Namespace = Namespace(b"http://www.opengis.net/gml");

pub struct Definition {
    value: String,
}

impl Definition {
    pub fn value(&self) -> &str {
        &self.value
    }
}

pub fn expect_text<R: BufRead>(
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
            Ok(Event::End(_)) => return Ok(s),
            Err(e) => return Err(e.into()),
            _ => (),
        }
    }
}

pub fn parse_definition<R: BufRead>(
    reader: &mut quick_xml::NsReader<R>,
    definitions: &mut HashMap<String, Definition>,
    buf: &mut Vec<u8>,
    buf2: &mut Vec<u8>,
) -> Result<(), ParseError> {
    let mut depth = 1;
    let mut key = None;
    let mut value = None;
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(start)) => {
                depth += 1;
                let (nsres, localname) = reader.resolve_element(start.name());
                match (depth, nsres, localname.as_ref()) {
                    (2, Bound(GML_NS), b"name") => {
                        key = Some(expect_text(reader, buf)?);
                        depth -= 1;
                    }
                    (2, Bound(GML_NS), b"description") => {
                        value = Some(expect_text(reader, buf)?);
                        depth -= 1;
                    }
                    a => {
                        println!("{:?}", a);
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
                return Err(quick_xml::Error::UnexpectedEof("Unexpected EOF".into()).into())
            }
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
    }

    match (key, value) {
        (Some(key), Some(value)) => {
            definitions.insert(key, Definition { value });
            Ok(())
        }
        _ => Err(ParseError::SchemaViolation(
            "Both <name> and <description> are required in a code definition.".into(),
        )),
    }
}

pub fn parse_dictionary<R: BufRead>(src_reader: R) -> Result<(), ParseError> {
    let mut reader = quick_xml::NsReader::from_reader(src_reader);
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    let mut depth = 0;
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();
    let mut definitions = HashMap::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(start)) => {
                depth += 1;
                let (nsres, localname) = reader.resolve_element(start.name());
                match (depth, nsres, localname.as_ref()) {
                    (1, Bound(GML_NS), b"Dictionary") => {
                        // ...
                    }
                    (1, _, _) => {
                        return Err(ParseError::SchemaViolation(format!(
                            "<Dictionary> is expected, but found {}",
                            String::from_utf8_lossy(localname.as_ref())
                        )))
                    }
                    (2, Bound(GML_NS), b"name") => {
                        let name = expect_text(&mut reader, &mut buf)?;
                        depth -= 1;
                        println!("{}", name);
                    }
                    (2, Bound(GML_NS), b"dictionaryEntry") => {}
                    (3, Bound(GML_NS), b"Definition") => {
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
                if depth == 0 {
                    return Ok(());
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => panic!("FIXME {:?}", e),
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    const EXAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <gml:Dictionary xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:gml="http://www.opengis.net/gml" xsi:schemaLocation="http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/profiles/SimpleDictionary/1.0.0/gmlSimpleDictionaryProfile.xsd" gml:id="cl_dc9314d0-8e10-11ec-b909-0242ac120002">
        <gml:name>Building_usage</gml:name>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id1">
                <gml:description>業務施設</gml:description>
                <gml:name>401</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id2">
                <gml:description>商業施設</gml:description>
                <gml:name>402</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id3">
                <gml:description>宿泊施設</gml:description>
                <gml:name>403</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id4">
                <gml:description>商業系複合施設</gml:description>
                <gml:name>404</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id5">
                <gml:description>住宅</gml:description>
                <gml:name>411</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id6">
                <gml:description>共同住宅</gml:description>
                <gml:name>412</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id7">
                <gml:description>店舗等併用住宅</gml:description>
                <gml:name>413</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id8">
                <gml:description>店舗等併用共同住宅</gml:description>
                <gml:name>414</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id9">
                <gml:description>作業所併用住宅</gml:description>
                <gml:name>415</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id10">
                <gml:description>官公庁施設</gml:description>
                <gml:name>421</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id11">
                <gml:description>文教厚生施設</gml:description>
                <gml:name>422</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id12">
                <gml:description>運輸倉庫施設</gml:description>
                <gml:name>431</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id13">
                <gml:description>工場</gml:description>
                <gml:name>441</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id14">
                <gml:description>農林漁業用施設</gml:description>
                <gml:name>451</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id15">
                <gml:description>供給処理施設</gml:description>
                <gml:name>452</gml:name>
            </gml:Definition>
        </gml:dictionaryEntry>
        <gml:dictionaryEntry>
            <gml:Definition gml:id="id16">
                <gml:description>防衛施設</gml:description>
                <gml:name>453</gml:name>
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

    #[test]
    fn parse_example() {
        let reader = BufReader::new(std::io::Cursor::new(EXAMPLE));
        parse_dictionary(reader).unwrap();
        // ...
    }
}
