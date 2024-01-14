mod utils;

use nusamai_citygml::Code;
use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_bridge_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/brid/51324378_brid_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::Bridge(bridge) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a CityFurniture");
    };

    assert_eq!(
        bridge.class,
        Some(Code::new("アーチ橋".to_string(), "03".to_string()))
    );
    assert_eq!(
        bridge.function,
        vec![Code::new("道路橋".to_string(), "01".to_string())]
    );
    assert_eq!(bridge.year_of_construction, Some("1962".to_string()));
    assert_eq!(bridge.is_movable, Some(false));
    assert_eq!(
        bridge.outer_bridge_construction[0].function,
        vec![Code::new("アーチ".to_string(), "04".to_string())]
    )
}
