use std::io::BufRead;
use std::path::Path;

use url::Url;

use nusamai_citygml::{
    CityGMLElement, CityGMLReader, Code, Geometries, Measure, ParseError, SubTreeReader,
};
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
    let filename = "./tests/data/numazu-shi/udx/tran/52385608_tran_6697_op.gml";

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

    assert_eq!(parsed_data.roads.len(), 549);
    assert_eq!(parsed_data.roads.len(), parsed_data.geometries.len());

    let road = parsed_data.roads.first().unwrap();

    assert_eq!(
        road.function,
        vec![Code::new("都道府県道".to_string(), "3".to_string(),)]
    );

    assert_eq!(
        road.usage,
        vec![
            Code::new(
                "緊急輸送道路（第三次緊急輸送道路）".to_string(),
                "3".to_string(),
            ),
            Code::new("避難路／避難道路".to_string(), "5".to_string(),),
        ]
    );

    assert_eq!(
        road.traffic_area.first().unwrap().function,
        vec![Code::new("歩道".to_string(), "2020".to_string(),)]
    );

    assert_eq!(
        road.auxiliary_traffic_area.first().unwrap().function,
        vec![Code::new("歩道部の段差".to_string(), "2000".to_string(),)]
    );

    assert_eq!(
        road.road_structure_attribute.first().unwrap().width,
        Some(Measure { value: 22.0 }),
    );

    assert_eq!(
        road.traffic_volume_attribute
            .first()
            .unwrap()
            .weekday12hour_traffic_volume,
        Some(8170),
    );
}
