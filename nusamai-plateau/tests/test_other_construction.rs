mod utils;

use nusamai_citygml::Code;
use nusamai_plateau::models::uro;
use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_other_construction_example() {
    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384697_cons_6697.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384698_cons_6697_1.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384698_cons_6697_2.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/53394695_cons_6697.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/53395603_cons_6697.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/56403133_cons_6697.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
        assert_eq!(
            cons.cons_base_attribute.as_ref().unwrap().admin_type,
            Some(Code::new("北陸地方整備局".into(), "23".into()))
        );
        assert_eq!(
            cons.cons_base_attribute.as_ref().unwrap().administorator,
            Some("信濃川河川事務所".into())
        )
    }
}
