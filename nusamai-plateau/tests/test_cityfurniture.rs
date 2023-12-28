use std::io::BufRead;
use std::path::Path;

use url::Url;

use nusamai_citygml::{CityGMLElement, CityGMLReader, Code, Geometries, ParseError, SubTreeReader};
use nusamai_plateau::models::cityfurniture::CityFurniture;
use nusamai_plateau::models::TopLevelCityObject;

#[derive(Default, Debug)]
struct ParsedData {
    cityfurnitures: Vec<CityFurniture>,
    geometries: Vec<Geometries>,
}

fn toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<ParsedData, ParseError> {
    let mut parsed_data = ParsedData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::CityFurniture(frn) => {
                    parsed_data.cityfurnitures.push(frn);
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
fn test_cityfurniture() {
    let filename = "./tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml";

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

    assert_eq!(parsed_data.cityfurnitures.len(), 28);
    assert_eq!(
        parsed_data.cityfurnitures.len(),
        parsed_data.geometries.len()
    );

    let frn = parsed_data.cityfurnitures.first().unwrap();
    assert_eq!(
        frn.function,
        vec![Code {
            value: "柱".to_string(),
            code: "4800".to_string(),
        }]
    );

    assert_eq!(
        frn.city_furniture_data_quality_attribute
            .as_ref()
            .unwrap()
            .src_scale,
        vec![Code {
            value: "地図情報レベル500".to_string(),
            code: "3".to_string(),
        }]
    );
}
