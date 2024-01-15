pub mod common;

use common::load_cityobjs;
use nusamai_citygml::Code;
use nusamai_plateau::models::TopLevelCityObject;

#[test]
fn test_load_tunnel_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/tun/53361613_tun_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::Tunnel(tunnel) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Tunnel");
    };

    assert_eq!(tunnel.class, Some(Code::new("交通".into(), "1000".into())));
    assert_eq!(
        tunnel.function,
        vec![Code::new("道路用トンネル".into(), "1010".into())]
    );
    assert_eq!(tunnel.year_of_construction, Some("1989".into()));
    assert_eq!(
        tunnel.outer_tunnel_installation[0].function,
        vec![Code::new("その他".into(), "90".into())]
    );
    assert_eq!(
        tunnel.outer_tunnel_installation[0].function,
        vec![Code::new("その他".into(), "90".into())]
    );
}
