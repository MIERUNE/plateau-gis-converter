mod utils;

use nusamai_citygml::Code;
use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_landuse_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml");
    assert_eq!(cityobjs.len(), 28);
    let TopLevelCityObject::CityFurniture(frn) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a CityFurniture");
    };

    assert_eq!(frn.function, vec![Code::new("柱".into(), "4800".into())]);
    assert_eq!(
        frn.city_furniture_data_quality_attribute
            .as_ref()
            .unwrap()
            .src_scale,
        vec![Code::new("地図情報レベル500".into(), "3".into(),)]
    );
}
