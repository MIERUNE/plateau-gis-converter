pub mod common;

use common::load_cityobjs;
use nusamai_plateau::models::relief;
use nusamai_plateau::models::TopLevelCityObject;

#[test]
fn test_load_tunnel_example() {
    let cityobjs = load_cityobjs("./tests/data/tokyo23-ku/udx/dem/533937_dem_6697_op.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::ReliefFeature(dem) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a ReliefFeature");
    };

    let relief::ReliefComponentProperty::TINRelief(tin) = &dem.relief_component[0] else {
        panic!("Unexpected relief component type");
    };
    assert_eq!(tin.lod, Some(1));
}
