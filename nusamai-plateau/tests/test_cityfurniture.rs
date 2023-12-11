use std::io::BufRead;

use citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use nusamai_plateau::models::CityObject;

#[derive(Default, Debug)]
struct Counter {
    city_objects: usize,
    appearances: usize,
    multipolygons: usize,
    cityobjectgroups: usize,
    cityfurniture: usize,
}

fn example_toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<Counter, ParseError> {
    let mut counter = Counter::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: CityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                CityObject::CityObjectGroup(_) => counter.cityobjectgroups += 1,
                CityObject::CityFurniture(frn) => {
                    println!("frn: {:?}", frn);
                    counter.cityfurniture += 1;
                }
                e => panic!("Unexpected city object type: {:?}", e),
            }
            let geometries = st.collect_geometries();
            counter.city_objects += 1;
            counter.multipolygons += geometries.multipolygon.len();
            Ok(())
        }
        b"gml:boundedBy" => {
            st.skip_current_element()?;
            Ok(())
        }
        b"app:appearanceMember" => {
            st.skip_current_element()?;
            counter.appearances += 1;
            Ok(())
        }
        other => Err(ParseError::SchemaViolation(format!(
            "Unrecognized element {}",
            String::from_utf8_lossy(other)
        ))),
    }) {
        Ok(_) => Ok(counter),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}

#[test]
fn test_cityfurniture() {
    // let test_file_path = "./tests/data/52384698_frn_6697_op.gml"; // 沼津市
    let test_file_path = "./tests/data/53394525_frn_6697_op.gml"; // 東京都23区
    let reader = std::io::BufReader::new(std::fs::File::open(test_file_path).unwrap());

    let mut xml_reader = quick_xml::NsReader::from_reader(reader);
    match CityGMLReader::new().start_root(&mut xml_reader) {
        Ok(mut st) => match example_toplevel_dispatcher(&mut st) {
            Ok(counter) => {
                // assert_eq!(counter.city_objects, 0);
                // assert_eq!(counter.appearances, 0);
                // assert_eq!(counter.multipolygons, 0);
                // assert_eq!(counter.cityobjectgroups, 0);
                assert_eq!(counter.cityfurniture, 2);
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };
}
