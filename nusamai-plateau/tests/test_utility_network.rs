pub mod common;

use common::load_cityobjs;
use nusamai_citygml::Code;
use nusamai_plateau::models::TopLevelCityObject;

#[test]
fn test_load_utility_network_example() {
    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/unf/gas_53403039_unf_6697.gml");
        assert_eq!(cityobjs.len(), 7);
        let TopLevelCityObject::OilGasChemicalsPipe(pipe) = &cityobjs[0].cityobj else {
            panic!("expected OilGasChemicalsPipe");
        };
        assert_eq!(pipe.function, vec![Code::new("管路".into(), "5500".into())]);
        let TopLevelCityObject::Appurtenance(appur) = &cityobjs[1].cityobj else {
            panic!("expected Appurtenance");
        };
        assert_eq!(
            appur.function,
            vec![Code::new("ハンドホール".into(), "5620".into())]
        );
        let TopLevelCityObject::Handhole(hole) = &cityobjs[5].cityobj else {
            panic!("expected Handhole");
        };
        assert_eq!(
            hole.function,
            vec![Code::new("ハンドホール".into(), "5620".into())]
        );
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/unf/elec_53403039_unf_6697.gml");
        assert_eq!(cityobjs.len(), 2);
        let TopLevelCityObject::Duct(_duct) = &cityobjs[0].cityobj else {
            panic!("unexpected cityobj");
        };
        let TopLevelCityObject::ElectricityCable(_cable) = &cityobjs[1].cityobj else {
            panic!("unexpected cityobj");
        };
    }

    {
        let cityobjs =
            load_cityobjs("./tests/data/plateau-3_0/udx/unf/sewer_53403039_unf_6697.gml");
        assert_eq!(cityobjs.len(), 6);
        let TopLevelCityObject::SewerPipe(_) = &cityobjs[0].cityobj else {
            panic!("expected SewerPipe");
        };
        let TopLevelCityObject::Manhole(_) = &cityobjs[1].cityobj else {
            panic!("expected Manhole");
        };
    }

    {
        let cityobjs =
            load_cityobjs("./tests/data/plateau-3_0/udx/unf/water_53403039_unf_6697.gml");
        assert_eq!(cityobjs.len(), 7);
        let TopLevelCityObject::Appurtenance(_) = &cityobjs[0].cityobj else {
            panic!("expected Appurtenance");
        };
        let TopLevelCityObject::WaterPipe(_) = &cityobjs[1].cityobj else {
            panic!("expected WaterPipe");
        };
    }
}
