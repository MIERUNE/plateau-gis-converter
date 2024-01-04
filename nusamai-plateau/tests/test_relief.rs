use std::io::BufRead;
use std::path::Path;

use url::Url;

use nusamai_citygml::{CityGMLElement, CityGMLReader, Geometries, ParseError, SubTreeReader};
use nusamai_plateau::models::{relief, ReliefFeature, TopLevelCityObject};

#[derive(Default, Debug)]
struct ParsedData {
    relief: Vec<ReliefFeature>,
    geometries: Vec<Geometries>,
}

fn toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<ParsedData, ParseError> {
    let mut parsed_data = ParsedData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::ReliefFeature(dem) => {
                    parsed_data.relief.push(dem);
                }
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
fn test_relief() {
    let filename = "./tests/data/tokyo23-ku/udx/dem/533937_dem_6697_op.gml";

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

    assert_eq!(parsed_data.relief.len(), 1);
    assert_eq!(parsed_data.relief.len(), parsed_data.geometries.len());

    let dem = parsed_data.relief.first().unwrap();
    let relief::ReliefComponentProperty::TINRelief(tin) = &dem.relief_component[0] else {
        panic!("Unexpected relief component type");
    };
    assert_eq!(tin.lod, Some(1));
}
