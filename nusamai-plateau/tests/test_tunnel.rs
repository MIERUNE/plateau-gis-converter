use std::io::BufRead;
use std::path::Path;

use url::Url;

use nusamai_citygml::{
    CityGMLElement, CityGMLReader, Code, GeometryStore, ParseError, SubTreeReader,
};
use nusamai_plateau::models::tunnel::Tunnel;
use nusamai_plateau::models::TopLevelCityObject;

#[derive(Default, Debug)]
struct ParsedData {
    tunnels: Vec<Tunnel>,
    geometries: Vec<GeometryStore>,
}

fn toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<ParsedData, ParseError> {
    let mut parsed_data = ParsedData::default();

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            match cityobj {
                TopLevelCityObject::Tunnel(tun) => {
                    parsed_data.tunnels.push(tun);
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
fn test_tunnel() {
    let filename = "./tests/data/plateau-3_0/udx/tun/53361613_tun_6697.gml";

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

    let tun = parsed_data.tunnels.first().unwrap();

    assert_eq!(tun.class, Some(Code::new("交通".into(), "1000".into())));

    assert_eq!(
        tun.function,
        vec![Code::new("道路用トンネル".into(), "1010".into())]
    );

    assert_eq!(tun.year_of_construction, Some("1989".into()));

    assert_eq!(
        tun.outer_tunnel_installation[0].function,
        vec![Code::new("その他".into(), "90".into())]
    );

    assert_eq!(
        tun.outer_tunnel_installation[0].function,
        vec![Code::new("その他".into(), "90".into())]
    );
}
