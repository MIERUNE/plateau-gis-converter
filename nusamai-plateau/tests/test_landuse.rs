pub mod common;

use common::load_cityobjs;
use nusamai_citygml::Code;
use nusamai_plateau::models::TopLevelCityObject;

#[test]
fn test_load_landuse_example() {
    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/luse/523836_luse_6668_op.gml");
    assert_eq!(cityobjs.len(), 225);
    let TopLevelCityObject::LandUse(landuse) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Landuse");
    };

    assert_eq!(
        landuse.land_use_detail_attribute[0].prefecture,
        Some(Code::new("静岡県".into(), "22".into()))
    );
    assert_eq!(
        landuse.land_use_detail_attribute[0].city,
        Some(Code::new("静岡県沼津市".into(), "22203".into()))
    );
}
