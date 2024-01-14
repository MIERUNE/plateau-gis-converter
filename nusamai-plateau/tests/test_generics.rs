mod utils;

use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_generics_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/gen/53392565_gen_6697.gml");
    assert_eq!(cityobjs.len(), 4);
    let TopLevelCityObject::GenericCityObject(_gen) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a GenericCityObject");
    };
}
