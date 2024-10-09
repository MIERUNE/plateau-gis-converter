use std::{io::BufRead, path::Path};

use nusamai_citygml::{CityGmlElement, CityGmlReader, GeometryStore, ParseError, SubTreeReader};
use nusamai_plateau::models::{appearance::AppearanceProperty, TopLevelCityObject};
use url::Url;

pub struct CityObject {
    pub cityobj: TopLevelCityObject,
    pub geometries: GeometryStore,
}

fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<Vec<CityObject>, ParseError> {
    let mut cityobjs = Vec::new();

    match st.parse_children(|st| {
        let current_path: &[u8] = &st.current_path();
        match current_path {
            b"core:cityObjectMember" => {
                let mut cityobj: TopLevelCityObject = Default::default();
                cityobj.parse(st)?;
                let geometries = st.collect_geometries(None);
                cityobjs.push(CityObject {
                    cityobj,
                    geometries,
                });
                Ok(())
            }
            b"gml:boundedBy" => {
                st.skip_current_element()?;
                Ok(())
            }
            b"app:appearanceMember" => {
                let mut app: AppearanceProperty = Default::default();
                app.parse(st)?;
                let AppearanceProperty::Appearance(_app) = app else {
                    unreachable!();
                };
                Ok(())
            }
            other => Err(ParseError::SchemaViolation(format!(
                "Unrecognized element {}",
                String::from_utf8_lossy(other)
            ))),
        }
    }) {
        Ok(_) => Ok(cityobjs),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}

pub fn load_cityobjs_from_reader(reader: impl BufRead, path: &Path) -> Vec<CityObject> {
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);
    let code_resolver = nusamai_plateau::codelist::Resolver::new();
    let source_url = Url::from_file_path(std::fs::canonicalize(path).unwrap()).unwrap();
    let context = nusamai_citygml::ParseContext::new(source_url, &code_resolver);

    let cityobjs = match CityGmlReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(cityobjs) => cityobjs,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };
    cityobjs
}

pub fn load_cityobjs(path: impl AsRef<Path>) -> Vec<CityObject> {
    let reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
    load_cityobjs_from_reader(reader, path.as_ref())
}

pub fn load_cityobjs_from_zstd(path: impl AsRef<Path>) -> Vec<CityObject> {
    let reader = std::io::BufReader::new(
        zstd::stream::Decoder::new(std::fs::File::open(&path).unwrap()).unwrap(),
    );
    load_cityobjs_from_reader(reader, path.as_ref())
}
