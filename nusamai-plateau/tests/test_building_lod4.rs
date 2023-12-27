use std::io::BufRead;

use nusamai_citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use nusamai_plateau::models::TopLevelCityObject;

#[derive(Default, Debug)]
struct Counter {
    city_objects: usize,
    appearances: usize,
    multipolygons: usize,
    buildings: usize,
    cityobjectgroups: usize,
}

fn example_toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<Counter, ParseError> {
    let mut counter = Counter::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::Building(_) => counter.buildings += 1,
                TopLevelCityObject::CityObjectGroup(_) => counter.cityobjectgroups += 1,
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
fn simple_read() {
    let reader = std::io::BufReader::new(
        zstd::stream::Decoder::new(
            std::fs::File::open(
                "./tests/data/tokyo23-ku/udx/bldg/53393680_bldg_6697_lod4.2_op.gml.zst",
            )
            .unwrap(),
        )
        .unwrap(),
    );

    let mut xml_reader = quick_xml::NsReader::from_reader(reader);
    let context = nusamai_citygml::ParseContext::default();
    match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => match example_toplevel_dispatcher(&mut st) {
            Ok(counter) => {
                assert_eq!(counter.city_objects, 1527);
                assert_eq!(counter.appearances, 1);
                assert_eq!(counter.multipolygons, 197633);
                assert_eq!(counter.buildings, 1485);
                assert_eq!(counter.cityobjectgroups, 42);
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };
}
