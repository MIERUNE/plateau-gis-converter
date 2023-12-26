//! This example is extract the geometry and attribute for each semantic child element of a CityGML file.

use citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use clap::Parser;
use nusamai_plateau::TopLevelCityObject;
use std::io::BufRead;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    filename: String,
}

fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<Vec<TopLevelCityObject>, ParseError> {
    let mut cityobjs: Vec<TopLevelCityObject> = vec![];

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: nusamai_plateau::models::TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            let geometries = st.collect_geometries();

            if let Some(root) = cityobj.into_object() {
                let obj = TopLevelCityObject { root, geometries };
                cityobjs.push(obj);
            }

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
        Ok(_) => Ok(cityobjs),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}

fn main() {
    let args = Args::parse();

    let reader = std::io::BufReader::new(std::fs::File::open(args.filename).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);

    let context = citygml::ParseContext::default();
    let cityobjs = match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(items) => items,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    for obj in cityobjs {
        let root = obj.root;
        let mpolys = &obj.geometries.multipolygon.into_iter().collect::<Vec<_>>();

        if let citygml::object::ObjectValue::FeatureOrData(fod) = root {
            println!("id: {:?}\n", fod.id);
            println!("vertices: {:?}\n", obj.geometries.vertices);
            println!("mpolys: {:?}\n", mpolys);
            println!("attributes: {:?}\n", fod.attributes);
            println!("geometries: {:?}\n", fod.geometries);

            // if fod.id == Some("bldg_eaaf7f0f-ed24-424d-aa92-3d811b327ccc".to_string()) {
            //     println!("{:?}\n", fod.id);
            //     println!("{:?}\n", obj.geometries);
            //     println!("{:?}\n", fod.attributes);
            // }
        }
    }
}

// 仮実装
#[derive(Debug, Clone, Default)]
struct Feature {
    geometries: Vec<[f64; 3]>,
    properties: serde_json::Map<String, serde_json::Value>,
}
