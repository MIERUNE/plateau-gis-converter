mod utils;

use nusamai_citygml::{Code, Measure};
use nusamai_plateau::models::TopLevelCityObject;
use utils::load_cityobjs;

#[test]
fn test_load_road_example() {
    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/tran/52385608_tran_6697_op.gml");
    assert_eq!(cityobjs.len(), 549);
    let TopLevelCityObject::Road(road) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Road");
    };

    assert_eq!(
        road.function,
        vec![Code::new("都道府県道".into(), "3".into(),)]
    );
    assert_eq!(
        road.usage,
        vec![
            Code::new("緊急輸送道路（第三次緊急輸送道路）".into(), "3".into()),
            Code::new("避難路／避難道路".into(), "5".into()),
        ]
    );
    assert_eq!(
        road.traffic_area.first().unwrap().function,
        vec![Code::new("歩道".into(), "2020".into())]
    );
    assert_eq!(
        road.auxiliary_traffic_area.first().unwrap().function,
        vec![Code::new("歩道部の段差".into(), "2000".into())]
    );
    assert_eq!(
        road.road_structure_attribute.first().unwrap().width,
        Some(Measure::new(22.0)),
    );
    assert_eq!(
        road.traffic_volume_attribute
            .first()
            .unwrap()
            .weekday12hour_traffic_volume,
        Some(8170),
    );
}

#[test]
fn test_load_railway_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/rwy/53395527_rwy_6697.gml");
    assert_eq!(cityobjs.len(), 4);
    let TopLevelCityObject::Railway(railway) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Railway");
    };

    assert_eq!(
        railway.id,
        Some("rwy_f087faa5-f548-4188-aa2e-03c7a5f2d3b9".to_string())
    );
    assert_eq!(railway.name, vec!["東北線".to_string()]);
    assert_eq!(railway.traffic_area.len(), 7);
    assert_eq!(
        railway.traffic_area.first().unwrap().function,
        vec![Code::new("軌道中心線".to_string(), "8000".to_string())]
    );
    assert_eq!(railway.auxiliary_traffic_area.len(), 1);
}

#[test]
fn test_load_track_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/trk/53361601_trk_6697.gml");
    assert_eq!(cityobjs.len(), 125);
    let TopLevelCityObject::Track(track) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Track");
    };

    assert_eq!(track.function, vec![Code::new("徒歩道".into(), "1".into())]);
    assert_eq!(
        track
            .tran_data_quality_attribute
            .as_ref()
            .unwrap()
            .geometry_src_desc,
        vec![Code::new("既成図数値化".into(), "6".into())]
    );
    assert_eq!(
        track.auxiliary_traffic_area.first().unwrap().function,
        vec![Code::new("島".into(), "3000".into())]
    );
    assert_eq!(
        track.track_attribute.first().unwrap().admin_type,
        Some(Code::new("市区町村".into(), "3".into()))
    );
}

#[test]
fn test_load_square_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/squr/53360690_squr_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::Square(square) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Square");
    };

    assert_eq!(
        square.class,
        Some(Code::new("その他".into(), "1090".into()))
    );
    assert_eq!(
        square.function,
        vec![Code::new("駅前広場".into(), "1".into())]
    );
    assert_eq!(square.traffic_area.len(), 9);
    assert_eq!(square.auxiliary_traffic_area.len(), 3);
    assert_eq!(
        square.traffic_area.first().unwrap().function,
        vec![Code::new("歩道部".into(), "2000".into())]
    );
    assert_eq!(
        square.auxiliary_traffic_area.first().unwrap().function,
        vec![Code::new("島".into(), "3000".into())]
    );
}

#[test]
fn test_load_waterway_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/wwy/52397630_wwy_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::Waterway(square) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Waterway");
    };

    assert_eq!(
        square.function,
        vec![Code::new("法定航路".into(), "01".into())]
    );
    assert_eq!(
        square.waterway_detail_attribute.as_ref().unwrap().route_id,
        Some("002".into())
    )
}
