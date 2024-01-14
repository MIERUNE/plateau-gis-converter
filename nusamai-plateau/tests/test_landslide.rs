mod utils;

use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_landslide_example() {
    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/lsld/523857_lsld_6668_op.gml");
    assert_eq!(cityobjs.len(), 81);
    let TopLevelCityObject::SedimentDisasterProneArea(lsld) = &cityobjs.first().unwrap().cityobj
    else {
        panic!("expected SedimentDisasterProneArea");
    };
    assert_eq!(lsld.location, Some("沼津市下香貫八重".into()));
    assert_eq!(lsld.disaster_type.as_ref().unwrap().code(), "1");
    assert_eq!(lsld.area_type.as_ref().unwrap().code(), "2");
    assert_eq!(lsld.zone_number.as_ref().unwrap(), "103-Ⅰ-0648");
    assert_eq!(lsld.status.as_ref().unwrap().code(), "0");
}
