use std::io::BufRead;

use citygml::{CityGMLElement, CityGMLReader, Code, Geometries, ParseError, SubTreeReader};
use nusamai_plateau::models::cityfurniture::CityFurniture;
use nusamai_plateau::models::CityObject;

#[derive(Default, Debug)]
struct ParsedData {
    cityfurnitures: Vec<CityFurniture>,
    geometries: Vec<Geometries>,
}

fn toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<ParsedData, ParseError> {
    let mut parsed_data = ParsedData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: CityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                CityObject::CityFurniture(frn) => {
                    parsed_data.cityfurnitures.push(frn);
                }
                CityObject::CityObjectGroup(_) => (),
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
fn test_cityfurniture() {
    let test_file_path = "./tests/data/52384698_frn_6697_op.gml";

    let reader = std::io::BufReader::new(std::fs::File::open(test_file_path).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);
    let parsed_data = match CityGMLReader::new().start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(parsed_data) => parsed_data,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    assert_eq!(parsed_data.cityfurnitures.len(), 44);
    assert_eq!(
        parsed_data.cityfurnitures.len(),
        parsed_data.geometries.len()
    );

    let frn = parsed_data.cityfurnitures.get(0).unwrap();
    assert_eq!(
        frn.class,
        Some(Code {
            value: "1000".to_string(),
            code: "1000".to_string(),
        })
    );
    assert_eq!(
        frn.function,
        vec![Code {
            value: "1010".to_string(),
            code: "1010".to_string(),
        }]
    );
}
