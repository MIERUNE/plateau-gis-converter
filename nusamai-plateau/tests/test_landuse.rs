use std::io::BufRead;
use std::path::Path;

use url::Url;

use nusamai_citygml::{
    CityGMLElement, CityGMLReader, Code, GeometryStore, ParseError, SubTreeReader,
};
use nusamai_plateau::models::landuse::LandUse;
use nusamai_plateau::models::TopLevelCityObject;
use nusamai_plateau::models::iur::uro;

#[derive(Default, Debug)]
struct ParsedData {
    landuses: Vec<LandUse>,
    geometries: Vec<GeometryStore>,
}


fn toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<ParsedData, ParseError> {
    let mut parsed_data = ParsedData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::LandUse(frn) => {
                    parsed_data.landuses.push(frn);
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
fn test_landuse() {
    let filename = "./tests/data/numazu-shi/udx/luse/523836_luse_6668_op.gml";

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

    assert_eq!(parsed_data.landuses.len(), 225);
    assert_eq!(
        parsed_data.landuses.len(),
        parsed_data.geometries.len()
    );

    let luse = parsed_data.landuses.first().unwrap();

    assert_eq!(
        luse.land_use_detail_attribute[0].prefecture,
        Some(Code::new("静岡県".to_string(), "22".to_string()))
    );

    assert_eq!(
        luse.land_use_detail_attribute[0].city,
        Some(Code::new("静岡県沼津市".to_string(), "22203".to_string()))
    );
}
