use std::io::BufRead;

use citygml::{CityGMLElement, CityGMLReader, Code, Geometries, ParseError, SubTreeReader};
use nusamai_plateau::models::Road;
use nusamai_plateau::models::TopLevelCityObject;

#[derive(Default, Debug)]
struct ParsedData {
    roads: Vec<Road>,
    geometries: Vec<Geometries>,
}

fn toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<ParsedData, ParseError> {
    let mut parsed_data = ParsedData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::Road(road) => {
                    parsed_data.roads.push(road);
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
fn test_road() {
    let test_file_path = "./tests/data/52385608_tran_6697_op.gml";

    let reader = std::io::BufReader::new(std::fs::File::open(test_file_path).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);
    let parsed_data = match CityGMLReader::new().start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(parsed_data) => parsed_data,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    assert_eq!(parsed_data.roads.len(), 549);
    assert_eq!(
        parsed_data.roads.len(),
        parsed_data.geometries.len()
    );

    let road = parsed_data.roads.first().unwrap();

    assert_eq!(
        road.function,
        vec![Code {
            value: "3".to_string(),
            code: "3".to_string(),
        }]
    );

    assert_eq!(
        road.usage,
        vec![Code {
            value: "3".to_string(),
            code: "3".to_string(),
        },
        Code {
            value: "5".to_string(),
            code: "5".to_string(),
        },]
    );

    assert_eq!(
        road.traffic_area.first().unwrap().function,
        vec![Code {
            value: "2020".to_string(),
            code: "2020".to_string(),
        }]
    );

    assert_eq!(
        road.auxiliary_traffic_area.first().unwrap().function,
        vec![Code {
            value: "2000".to_string(),
            code: "2000".to_string(),
        }]
    );

    assert_eq!(
        road.road_structure_attribute.first().unwrap().width,
        Some(22.0),
    );

    assert_eq!(
        road.traffic_volume_attribute.first().unwrap().weekday12hour_traffic_volume,
        Some(8170),
    );
}
