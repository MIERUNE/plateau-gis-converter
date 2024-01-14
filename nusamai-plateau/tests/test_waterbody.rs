mod utils;

use nusamai_citygml::{Code, Measure};
use nusamai_plateau::models::{uro, waterbody, TopLevelCityObject};
use utils::load_cityobjs;

#[test]
fn test_load_waterbody_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/wtr/55370156_wtr_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::WaterBody(waterbody) = &cityobjs.first().unwrap().cityobj else {
        panic!("expected WaterBody");
    };

    assert_eq!(
        waterbody.class,
        Some(Code::new(
            "river / stream（河川/小川）".into(),
            "1030".into()
        ))
    );
}

#[test]
fn test_load_flood_example() {
    {
        let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/fld/52385721_fld_6697_l1_op.gml");
        assert_eq!(cityobjs.len(), 3);
        let TopLevelCityObject::WaterBody(waterbody) = &cityobjs.first().unwrap().cityobj else {
            panic!("expected SedimentDisasterProneArea");
        };
        assert_eq!(waterbody.flooding_risk_attribute.len(), 1);
        let uro::WaterBodyFloodingRiskAttributeProperty::WaterBodyRiverFloodingRiskAttribute(flood) =
            waterbody.flooding_risk_attribute.first().unwrap()
        else {
            panic!("expected WaterBodyRiverFloodingRiskAttribute");
        };
        assert_eq!(flood.admin_type.as_ref().unwrap().code(), "1");
        assert_eq!(flood.scale.as_ref().unwrap().code(), "L1");
    }

    {
        let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/tnm/523855_tnm_6697_op.gml");
        assert_eq!(cityobjs.len(), 3);
        let TopLevelCityObject::WaterBody(_waterbody) = &cityobjs.first().unwrap().cityobj else {
            panic!("expected SedimentDisasterProneArea");
        };
    }
}
