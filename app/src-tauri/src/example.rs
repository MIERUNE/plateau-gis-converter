// nusamai-geojson の exmaple/gml2geojson を元にした、暫定的な処理
use citygml::{CityGMLElement, CityGMLReader, Geometries, ParseError, SubTreeReader};
use nusamai_geojson::{toplevel_city_object_to_geojson_features, TopLevelCityObject};
use nusamai_plateau::models::CityObject;
use std::fs;
use std::io::BufRead;
use std::io::BufWriter;

fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<Vec<(CityObject, Geometries)>, ParseError> {
    let mut items: Vec<(CityObject, Geometries)> = vec![];

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: CityObject = Default::default();
            cityobj.parse(st)?;
            let geometries = st.collect_geometries();
            items.push((cityobj, geometries));
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
        Ok(_) => Ok(items),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}

pub fn citygml_to_geojson(input_path: &str, output_path: &str) {
    let reader = std::io::BufReader::new(std::fs::File::open(input_path).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);

    let items = match CityGMLReader::new().start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(items) => items,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    let tlc_objs: Vec<_> = items
        .iter()
        .map(|(o, g)| {
            let cityobj = match o.objectify().unwrap() {
                citygml::object::ObjectValue::FeatureOrData(fod) => fod,
                _ => panic!("Not a FeatureOrData"),
            };

            TopLevelCityObject {
                cityobj,
                geometries: g.clone(),
            }
        })
        .collect();

    let geojson_features: Vec<geojson::Feature> = tlc_objs
        .iter()
        .flat_map(toplevel_city_object_to_geojson_features)
        .collect();

    let geojson_feature_collection = geojson::FeatureCollection {
        bbox: None,
        features: geojson_features,
        foreign_members: None,
    };
    let geojson = geojson::GeoJson::from(geojson_feature_collection);

    let mut file = fs::File::create(output_path).unwrap();
    let mut writer = BufWriter::new(&mut file);
    serde_json::to_writer(&mut writer, &geojson).unwrap();
}
