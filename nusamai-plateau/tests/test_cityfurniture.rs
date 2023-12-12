use std::io::BufRead;

use citygml::{CityGMLElement, CityGMLReader, Geometries, ParseError, SubTreeReader};
use nusamai_plateau::models::CityObject;

#[derive(Default, Debug)]
struct ParsedData {
    cityfurnitures: Vec<CityObject>,
    geometries: Vec<Geometries>,
}

fn example_toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<ParsedData, ParseError> {
    let mut parsed_data = ParsedData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: CityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                CityObject::CityFurniture(frn) => {
                    parsed_data
                        .cityfurnitures
                        .push(CityObject::CityFurniture(frn));
                }
                CityObject::CityObjectGroup(_) => (),
                e => panic!("Unexpected city object type: {:?}", e),
            }
            let geometries = st.collect_geometries();
            parsed_data.geometries.push(geometries);
            Ok(())
        }
        b"gml:boundedBy" => {
            st.skip_current_element()?;
            Ok(())
        }
        b"app:appearanceMember" => {
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
fn test_cityfurniture() {
    let test_file_path = "./tests/data/52384698_frn_6697_op.gml";

    let reader = std::io::BufReader::new(std::fs::File::open(test_file_path).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);

    match CityGMLReader::new().start_root(&mut xml_reader) {
        Ok(mut st) => match example_toplevel_dispatcher(&mut st) {
            Ok(parsed_data) => {
                assert_eq!(parsed_data.geometries.len(), 44);
                assert_eq!(parsed_data.cityfurnitures.len(), 44);
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };
}
