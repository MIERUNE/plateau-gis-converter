mod utils;

use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_underground_building_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/ubld/51324378_ubld_6697.gml");
    assert_eq!(cityobjs.len(), 3);
    let TopLevelCityObject::UndergroundBuilding(ubld) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a UndergroundBuilding");
    };
    assert_eq!(ubld.interior_room.len(), 2);
    let room = &ubld.interior_room[1];
    assert_eq!(room.room_installation.len(), 3);
}
