use std::io::BufRead;
use std::path::Path;

use url::Url;

use nusamai_citygml::{
    CityGMLElement, CityGMLReader, Code, GeometryStore, ParseError, SubTreeReader,
};
use nusamai_plateau::models::Railway;
use nusamai_plateau::models::TopLevelCityObject;

#[derive(Default, Debug)]
struct ParsedData {
    railways: Vec<Railway>,
    geometries: Vec<GeometryStore>,
}

fn toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<ParsedData, ParseError> {
    let mut parsed_data = ParsedData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::Railway(rwy) => {
                    parsed_data.railways.push(rwy);
                }
                TopLevelCityObject::CityObjectGroup(_) => (),
                e => panic!("Unexpected city object type: {:?}", e),
            }
            let geometries = st.collect_geometries();
            parsed_data.geometries.push(geometries);
            Ok(())
        }
        b"gml:boundedBy" | b"app:appearanceMember" => {
            st.skip_current_element()?;
            Ok(())
        }
        other => Err(ParseError::SchemaViolation(format!(
            "Unrecognized element {}",
            String::from_utf8_lossy(other)
        ))),
    }) {
        Ok(_) => Ok(parsed_data),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}

#[test]
fn test_track() {
    let filename = "./tests/data/plateau-3_0/udx/rwy/53395518_rwy_6697.gml";

    let reader = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);

    let code_resolver = nusamai_plateau::codelist::Resolver::new();
    let source_url =
        Url::from_file_path(std::fs::canonicalize(Path::new(filename)).unwrap()).unwrap();
    let context = nusamai_citygml::ParseContext::new(source_url, &code_resolver);

    let parsed_data = match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(parsed_data) => parsed_data,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    assert_eq!(parsed_data.railways.len(), 1);
    assert_eq!(parsed_data.railways.len(), parsed_data.geometries.len());

    let railway = parsed_data.railways.first().unwrap();

    assert_eq!(
        railway.id,
        Some("rwy_58634d44-b0bd-4b22-bb6b-fafadda9203f".to_string())
    );

    assert_eq!(railway.name, vec!["東北線".to_string()]);

    assert_eq!(
        railway.traffic_area.first().unwrap().function,
        vec![Code::new("軌道中心線".to_string(), "8000".to_string())]
    );
}
