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

#[tokio::main]
async fn main() {
    // Parse CityGML

    let args = Args::parse();

    let reader = std::io::BufReader::new(std::fs::File::open(args.filename).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);
    let _cityobjs = match CityGMLReader::new().start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(items) => items,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    // GeoPackage

    let output_path = "output.gpkg";
    let _handler = nusamai_gpkg::GpkgHandler::init(output_path).await.unwrap();
    // TODO: handler.add_objects(&cityobjs).await;
}
