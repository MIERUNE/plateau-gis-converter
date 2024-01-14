mod utils;

use nusamai_citygml::Code;
use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_waterbody_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/wtr/55370156_wtr_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::WaterBody(waterbody) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a WaterBody");
    };

    assert_eq!(
        waterbody.class,
        Some(Code::new(
            "river / stream（河川/小川）".into(),
            "1030".into()
        ))
    );
}
