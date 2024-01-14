mod utils;

use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_vegetation_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/veg/52385628_veg_6697_op.gml");
    assert_eq!(cityobjs.len(), 28);
    let TopLevelCityObject::PlantCover(veg) = &cityobjs[0].cityobj else {
        panic!("expected PlantCover");
    };
    assert_eq!(veg.average_height.as_ref().unwrap().value(), 0.5);
    let dq = veg.vegetation_data_quality_attribute.as_ref().unwrap();
    assert_eq!(dq.appearance_src_desc.first().unwrap().code(), "4");

    let TopLevelCityObject::SolitaryVegetationObject(veg) = &cityobjs[9].cityobj else {
        panic!("expected SolitaryVegetationObject");
    };
    assert_eq!(veg.height.as_ref().unwrap().value(), 12.5);
    let dq = veg.vegetation_data_quality_attribute.as_ref().unwrap();
    assert_eq!(dq.appearance_src_desc.first().unwrap().code(), "4");
}
