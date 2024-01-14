mod utils;

use nusamai_citygml::{Code, Date};
use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_waterbody_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/area/523846_area_6697.gml");
    assert_eq!(cityobjs.len(), 4);
    let TopLevelCityObject::Zone(zone) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Zone");
    };

    assert_eq!(
        zone.function,
        vec![Code::new("港湾区域".into(), "0201".into())]
    );
    assert_eq!(zone.urf_valid_from, Date::from_ymd_opt(1, 1, 1));
    assert_eq!(
        zone.valid_from_type,
        Code::new("決定".into(), "1".into()).into()
    );
}
