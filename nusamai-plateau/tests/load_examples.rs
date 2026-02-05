pub mod common;

use common::{load_cityobjs, load_cityobjs_from_zstd};
use nusamai_citygml::{Code, Date, Measure};
use nusamai_plateau::models::{relief, uro, TopLevelCityObject};

// #[test]
// fn load_area_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/area/523846_area_6697.gml");
//     assert_eq!(cityobjs.len(), 4);
//     let TopLevelCityObject::Zone(zone) = &cityobjs.first().unwrap().cityobj else {
//         panic!("Not a Zone");
//     };

//     assert_eq!(
//         zone.function,
//         vec![Code::new("港湾区域".into(), "0201".into())]
//     );
//     assert_eq!(zone.urf_valid_from, Date::from_ymd_opt(1, 1, 1));
//     assert_eq!(
//         zone.valid_from_type,
//         Code::new("決定".into(), "1".into()).into()
//     );
// }

// #[test]
// fn load_bridge_example() {
//     {
//         let cityobjs =
//             load_cityobjs("./tests/data/plateau-3_0/udx/brid/dorokyo_51324378_brid_6697.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::Bridge(bridge) = &cityobjs.first().unwrap().cityobj else {
//             panic!("Expected a bridge");
//         };

//         assert_eq!(
//             bridge.class,
//             Some(Code::new("アーチ橋".to_string(), "03".to_string()))
//         );
//         assert_eq!(
//             bridge.function,
//             vec![Code::new("道路橋".to_string(), "01".to_string())]
//         );
//         assert_eq!(bridge.year_of_construction, Some("1962".to_string()));
//         assert_eq!(bridge.is_movable, Some(false));
//         assert_eq!(
//             bridge.outer_bridge_construction[0].function,
//             vec![Code::new("アーチ".to_string(), "04".to_string())]
//         );
//     }

//     {
//         let cityobjs =
//             load_cityobjs("./tests/data/plateau-3_0/udx/brid/hodokyo_51324378_brid_6697.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::Bridge(bridge) = &cityobjs.first().unwrap().cityobj else {
//             panic!("Expected a bridge");
//         };

//         assert_eq!(
//             bridge.function,
//             vec![Code::new("横断歩道橋".to_string(), "07".to_string())]
//         );
//         assert_eq!(bridge.year_of_construction, Some("1968".to_string()));
//         assert_eq!(
//             bridge
//                 .brid_risk_assessment_attribute
//                 .as_ref()
//                 .unwrap()
//                 .risk_type
//                 .as_ref()
//                 .unwrap()
//                 .value(),
//             "判定区分Ⅰ（健全）"
//         );
//     }

//     {
//         let cityobjs =
//             load_cityobjs("./tests/data/plateau-3_0/udx/brid/pedeck_53360690_brid_6697.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::Bridge(bridge) = &cityobjs.first().unwrap().cityobj else {
//             panic!("Expected a bridge");
//         };

//         assert_eq!(
//             bridge.function,
//             vec![Code::new(
//                 "ペデストリアンデッキ".to_string(),
//                 "08".to_string()
//             )]
//         );
//         assert_eq!(
//             bridge
//                 .brid_structure_attribute
//                 .as_ref()
//                 .unwrap()
//                 .length
//                 .as_ref()
//                 .unwrap()
//                 .value(),
//             776.0
//         );
//     }
// }

#[test]
fn load_building_lod4_example() {
    let cityobjs = load_cityobjs_from_zstd(
        "./tests/data/tokyo23-ku/udx/bldg/53393680_bldg_6697_lod4.2_op.gml.zst",
    );

    let mut multipolygons = 0;
    let mut buildings = 0;
    let mut cityobjectgroups = 0;

    assert_eq!(cityobjs.len(), 1527);

    for cityobj in cityobjs {
        multipolygons += cityobj.geometries.multipolygon.len();
        match cityobj.cityobj {
            TopLevelCityObject::Building(_building) => {
                buildings += 1;
            }
            TopLevelCityObject::CityObjectGroup(_group) => {
                cityobjectgroups += 1;
            }
            _ => {}
        }
    }

    assert_eq!(buildings, 1485);
    assert_eq!(cityobjectgroups, 42);
    assert_eq!(multipolygons, 197633);
}

#[test]
fn load_cityfurniture_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml");
    assert_eq!(cityobjs.len(), 28);
    let TopLevelCityObject::CityFurniture(frn) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a CityFurniture");
    };

    assert_eq!(frn.function, vec![Code::new("柱".into(), "4800".into())]);
    assert_eq!(
        frn.frn_data_quality_attribute.as_ref().unwrap().src_scale,
        vec![Code::new("地図情報レベル500".into(), "3".into(),)]
    );
}

#[test]
fn load_cityfurniture_multi_surface() {
    // Test multi surface parsing
    let cityobjs = load_cityobjs("./tests/data/tsukuba-shi/udx/frn/54400098_frn_6697_op.gml");
    assert_eq!(cityobjs.len(), 153);

    // Verify that multi-surface geometries with multiple surfaceMembers are collected correctly
    // Ground truth from GML file: 369 MultiSurface elements containing 45047 surfaceMember elements (polygons)
    let mut multipolygons = 0;

    for cityobj in cityobjs {
        multipolygons += cityobj.geometries.multipolygon.len();
    }

    assert_eq!(multipolygons, 45047);
}

// #[test]
// fn load_generics_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/gen/53392565_gen_6697.gml");
//     assert_eq!(cityobjs.len(), 4);
//     let TopLevelCityObject::GenericCityObject(_gen) = &cityobjs.first().unwrap().cityobj else {
//         panic!("Not a GenericCityObject");
//     };
// }

#[test]
fn load_landslide_example() {
    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/lsld/523857_lsld_6697_op.gml");
    assert_eq!(cityobjs.len(), 81);
    let TopLevelCityObject::SedimentDisasterProneArea(lsld) = &cityobjs.first().unwrap().cityobj
    else {
        panic!("expected SedimentDisasterProneArea");
    };
    assert_eq!(lsld.location, Some("沼津市下香貫八重".into()));
    assert_eq!(
        lsld.disaster_type.as_ref().and_then(|c| c.code()),
        Some("1")
    );
    assert_eq!(lsld.area_type.as_ref().and_then(|c| c.code()), Some("2"));
    assert_eq!(lsld.zone_number.as_ref().unwrap(), "103-Ⅰ-0648");
    assert_eq!(lsld.status.as_ref().and_then(|c| c.code()), Some("0"));
}

#[test]
fn load_landuse_example() {
    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/luse/523836_luse_6668_op.gml");
    assert_eq!(cityobjs.len(), 225);
    let TopLevelCityObject::LandUse(landuse) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Landuse");
    };

    assert_eq!(
        landuse.land_use_detail_attribute[0].prefecture,
        Some(Code::new("静岡県".into(), "22".into()))
    );
    assert_eq!(
        landuse.land_use_detail_attribute[0].city,
        Some(Code::new("静岡県沼津市".into(), "22203".into()))
    );
}

// #[test]
// fn load_other_construction_example() {
//     {
//         let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384697_cons_6697.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
//             panic!("must be OtherConstruction");
//         };
//         let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
//             panic!("must be DmGeometricAttribute");
//         };
//     }

//     {
//         let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384698_cons_6697_1.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
//             panic!("must be OtherConstruction");
//         };
//         let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
//             panic!("must be DmGeometricAttribute");
//         };
//     }

//     {
//         let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384698_cons_6697_2.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
//             panic!("must be OtherConstruction");
//         };
//         let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
//             panic!("must be DmGeometricAttribute");
//         };
//     }

//     {
//         let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/53394695_cons_6697.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
//             panic!("must be OtherConstruction");
//         };
//         let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
//             panic!("must be DmGeometricAttribute");
//         };
//     }

//     {
//         let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/53395603_cons_6697.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
//             panic!("must be OtherConstruction");
//         };
//         let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
//             panic!("must be DmGeometricAttribute");
//         };
//     }

//     {
//         let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/56403133_cons_6697.gml");
//         assert_eq!(cityobjs.len(), 1);
//         let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
//             panic!("must be OtherConstruction");
//         };
//         let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
//             panic!("must be DmGeometricAttribute");
//         };
//         assert_eq!(
//             cons.cons_base_attribute.as_ref().unwrap().admin_type,
//             Some(Code::new("北陸地方整備局".into(), "23".into()))
//         );
//         assert_eq!(
//             cons.cons_base_attribute.as_ref().unwrap().administorator,
//             Some("信濃川河川事務所".into())
//         )
//     }
// }

#[test]
fn load_dem_example() {
    let cityobjs = load_cityobjs("./tests/data/yokosuka-shi/udx/dem/523965_dem_6697_05_op.gml");
    assert_eq!(cityobjs.len(), 2);
    let TopLevelCityObject::ReliefFeature(dem) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a ReliefFeature");
    };

    let relief::ReliefComponentProperty::TINRelief(tin) = &dem.relief_component[0] else {
        panic!("Unexpected relief component type");
    };
    assert_eq!(tin.lod, Some(1));

    assert_eq!(cityobjs[0].geometries.epsg, 6697);
    assert_eq!(
        cityobjs
            .iter()
            .map(|o| o.geometries.multipolygon.len())
            .sum::<usize>(),
        1066
    );
}

#[test]
fn load_road_example() {
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
        road.road_structure_attribute[0].width,
        Some(Measure::new("22".to_string(), Some("m".to_string()))),
    );
    assert_eq!(
        road.traffic_volume_attribute[0].weekday12hour_traffic_volume,
        Some(8170),
    );
}

// #[test]
// fn load_railway_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/rwy/53395527_rwy_6697.gml");
//     assert_eq!(cityobjs.len(), 4);
//     let TopLevelCityObject::Railway(railway) = &cityobjs.first().unwrap().cityobj else {
//         panic!("Not a Railway");
//     };

//     assert_eq!(
//         railway.id,
//         "rwy_f087faa5-f548-4188-aa2e-03c7a5f2d3b9".to_string()
//     );

//     assert_eq!(
//         railway.name,
//         vec![Code::new("東北線".into(), "東北線".into())]
//     );
//     assert_eq!(railway.traffic_area.len(), 7);
//     assert_eq!(
//         railway.traffic_area.first().unwrap().function,
//         vec![Code::new("軌道中心線".to_string(), "8000".to_string())]
//     );
//     assert_eq!(railway.auxiliary_traffic_area.len(), 1);
// }

// #[test]
// fn load_track_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/trk/53361601_trk_6697.gml");
//     assert_eq!(cityobjs.len(), 125);
//     let TopLevelCityObject::Track(track) = &cityobjs.first().unwrap().cityobj else {
//         panic!("Not a Track");
//     };

//     assert_eq!(track.function, vec![Code::new("徒歩道".into(), "1".into())]);
//     assert_eq!(
//         track
//             .tran_data_quality_attribute
//             .as_ref()
//             .unwrap()
//             .geometry_src_desc,
//         vec![Code::new("既成図数値化".into(), "6".into())]
//     );
//     assert_eq!(
//         track.auxiliary_traffic_area.first().unwrap().function,
//         vec![Code::new("島".into(), "3000".into())]
//     );
//     assert_eq!(
//         track.track_attribute.as_ref().unwrap().admin_type,
//         Some(Code::new("市区町村".into(), "3".into()))
//     );
// }

// #[test]
// fn load_square_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/squr/53360690_squr_6697.gml");
//     assert_eq!(cityobjs.len(), 1);
//     let TopLevelCityObject::Square(square) = &cityobjs.first().unwrap().cityobj else {
//         panic!("Not a Square");
//     };

//     assert_eq!(
//         square.class,
//         Some(Code::new("その他".into(), "1090".into()))
//     );
//     assert_eq!(
//         square.function,
//         vec![Code::new("駅前広場".into(), "1".into())]
//     );
//     assert_eq!(square.traffic_area.len(), 9);
//     assert_eq!(square.auxiliary_traffic_area.len(), 3);
//     assert_eq!(
//         square.traffic_area.first().unwrap().function,
//         vec![Code::new("歩道部".into(), "2000".into())]
//     );
//     assert_eq!(
//         square.auxiliary_traffic_area.first().unwrap().function,
//         vec![Code::new("島".into(), "3000".into())]
//     );
// }

// #[test]
// fn load_waterway_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/wwy/52397630_wwy_6697.gml");
//     assert_eq!(cityobjs.len(), 1);
//     let TopLevelCityObject::Waterway(square) = &cityobjs.first().unwrap().cityobj else {
//         panic!("Not a Waterway");
//     };

//     assert_eq!(
//         square.function,
//         vec![Code::new("法定航路".into(), "01".into())]
//     );
//     assert_eq!(
//         square.waterway_detail_attribute.as_ref().unwrap().route_id,
//         Some("002".into())
//     )
// }

// #[test]
// fn load_tunnel_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/tun/53361613_tun_6697.gml");
//     assert_eq!(cityobjs.len(), 1);
//     let TopLevelCityObject::Tunnel(tunnel) = &cityobjs.first().unwrap().cityobj else {
//         panic!("Not a Tunnel");
//     };

//     assert_eq!(tunnel.class, Some(Code::new("交通".into(), "1000".into())));
//     assert_eq!(
//         tunnel.function,
//         vec![Code::new("道路用トンネル".into(), "1010".into())]
//     );
//     assert_eq!(tunnel.year_of_construction, Some("1989".into()));
//     assert_eq!(
//         tunnel.outer_tunnel_installation[0].function,
//         vec![Code::new("その他".into(), "90".into())]
//     );
//     assert_eq!(
//         tunnel.outer_tunnel_installation[0].function,
//         vec![Code::new("その他".into(), "90".into())]
//     );
// }

// #[test]
// fn load_underground_building_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/ubld/51324378_ubld_6697.gml");
//     assert_eq!(cityobjs.len(), 3);
//     let TopLevelCityObject::UndergroundBuilding(ubld) = &cityobjs.first().unwrap().cityobj else {
//         panic!("Not a UndergroundBuilding");
//     };
//     assert_eq!(ubld.interior_room.len(), 2);
//     let room = &ubld.interior_room[1];
//     assert_eq!(room.room_installation.len(), 3);
// }

#[test]
fn load_urf_example() {
    let cityobjs = load_cityobjs("./tests/data/takeo-shi/udx/urf/493060_urf_6697_11_op.gml");
    assert_eq!(cityobjs.len(), 95);

    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/urf/523857_urf_6697_08_op.gml");
    assert_eq!(cityobjs.len(), 44);

    let cityobjs = load_cityobjs("./tests/data/tokyo23-ku/udx/urf/533957_urf_6668_op.gml");
    assert_eq!(cityobjs.len(), 38);
}

// #[test]
// fn load_utility_network_example() {
//     {
//         let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/unf/gas_53403039_unf_6697.gml");
//         assert_eq!(cityobjs.len(), 7);
//         let TopLevelCityObject::OilGasChemicalsPipe(pipe) = &cityobjs[0].cityobj else {
//             panic!("expected OilGasChemicalsPipe");
//         };
//         assert_eq!(pipe.function, vec![Code::new("管路".into(), "5500".into())]);
//         let TopLevelCityObject::Appurtenance(appur) = &cityobjs[1].cityobj else {
//             panic!("expected Appurtenance");
//         };
//         assert_eq!(
//             appur.function,
//             vec![Code::new("ハンドホール".into(), "5620".into())]
//         );
//         let TopLevelCityObject::Handhole(hole) = &cityobjs[5].cityobj else {
//             panic!("expected Handhole");
//         };
//         assert_eq!(
//             hole.function,
//             vec![Code::new("ハンドホール".into(), "5620".into())]
//         );
//     }

//     {
//         let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/unf/elec_53403039_unf_6697.gml");
//         assert_eq!(cityobjs.len(), 2);
//         let TopLevelCityObject::Duct(_duct) = &cityobjs[0].cityobj else {
//             panic!("unexpected cityobj");
//         };
//         let TopLevelCityObject::ElectricityCable(_cable) = &cityobjs[1].cityobj else {
//             panic!("unexpected cityobj");
//         };
//     }

//     {
//         let cityobjs =
//             load_cityobjs("./tests/data/plateau-3_0/udx/unf/sewer_53403039_unf_6697.gml");
//         assert_eq!(cityobjs.len(), 6);
//         let TopLevelCityObject::SewerPipe(_) = &cityobjs[0].cityobj else {
//             panic!("expected SewerPipe");
//         };
//         let TopLevelCityObject::Manhole(_) = &cityobjs[1].cityobj else {
//             panic!("expected Manhole");
//         };
//     }

//     {
//         let cityobjs =
//             load_cityobjs("./tests/data/plateau-3_0/udx/unf/water_53403039_unf_6697.gml");
//         assert_eq!(cityobjs.len(), 7);
//         let TopLevelCityObject::Appurtenance(_) = &cityobjs[0].cityobj else {
//             panic!("expected Appurtenance");
//         };
//         let TopLevelCityObject::WaterPipe(_) = &cityobjs[1].cityobj else {
//             panic!("expected WaterPipe");
//         };
//     }
// }

// #[test]
// fn load_vegetation_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/veg/52385628_veg_6697_op.gml");
//     assert_eq!(cityobjs.len(), 28);
//     let TopLevelCityObject::PlantCover(veg) = &cityobjs[0].cityobj else {
//         panic!("expected PlantCover");
//     };
//     assert_eq!(veg.average_height.as_ref().unwrap().value(), 0.5);
//     let dq = veg.vegetation_data_quality_attribute.as_ref().unwrap();
//     assert_eq!(dq.appearance_src_desc.first().unwrap().code(), "4");

//     let TopLevelCityObject::SolitaryVegetationObject(veg) = &cityobjs[9].cityobj else {
//         panic!("expected SolitaryVegetationObject");
//     };
//     assert_eq!(veg.height.as_ref().unwrap().value(), 12.5);
//     let dq = veg.vegetation_data_quality_attribute.as_ref().unwrap();
//     assert_eq!(dq.appearance_src_desc.first().unwrap().code(), "4");
// }

// #[test]
// fn load_waterbody_example() {
//     let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/wtr/55370156_wtr_6697.gml");
//     assert_eq!(cityobjs.len(), 1);
//     let TopLevelCityObject::WaterBody(waterbody) = &cityobjs.first().unwrap().cityobj else {
//         panic!("expected WaterBody");
//     };

//     assert_eq!(
//         waterbody.class,
//         Some(Code::new(
//             "river / stream（河川/小川）".into(),
//             "1030".into()
//         ))
//     );
// }

#[test]
fn load_flood_example() {
    {
        let cityobjs = load_cityobjs(
            "./tests/data/numazu-shi/udx/fld/natl/kanogawa_kisegawa/52385721_fld_6697_l1_op.gml",
        );
        assert_eq!(cityobjs.len(), 3);
        let TopLevelCityObject::WaterBody(waterbody) = &cityobjs.first().unwrap().cityobj else {
            panic!("expected SedimentDisasterProneArea");
        };
        let uro::FloodingRiskAttributeProperty::RiverFloodingRiskAttribute(flood) =
            waterbody.flooding_risk_attribute.first().unwrap()
        else {
            panic!("expected WaterBodyRiverFloodingRiskAttribute");
        };
        assert_eq!(flood.admin_type.as_ref().and_then(|c| c.code()), Some("1"));
        assert_eq!(flood.scale.as_ref().and_then(|c| c.code()), Some("1"));
        assert_eq!(
            flood.scale.as_ref().map(|c| c.value()),
            Some("L1（計画規模）")
        );
    }

    {
        let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/tnm/523855_tnm_6697_op.gml");
        assert_eq!(cityobjs.len(), 3);
        let TopLevelCityObject::WaterBody(_waterbody) = &cityobjs.first().unwrap().cityobj else {
            panic!("expected SedimentDisasterProneArea");
        };
    }
}

#[test]
fn load_urf_kodo_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533915_urf_6668_kodo_op.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::HeightControlDistrict(hcd) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a HeightControlDistrict");
    };

    assert_eq!(
        hcd.function,
        vec![Code::new("高度地区".to_string(), "18".to_string(),)]
    );

    assert_eq!(
        hcd.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2009, 11, 11).unwrap())
    );

    assert_eq!(
        hcd.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}

#[test]
fn load_urf_kuiki_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533915_urf_6668_kuiki_op.gml");
    assert_eq!(cityobjs.len(), 2);
    let TopLevelCityObject::AreaClassification(acf) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a AreaClassification");
    };

    assert_eq!(
        acf.function,
        vec![Code::new("市街化区域".to_string(), "22".to_string(),)]
    );

    assert_eq!(
        acf.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2009, 9, 18).unwrap())
    );

    assert_eq!(
        acf.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}

#[test]
fn load_urf_rinko_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533915_urf_6668_rinko_op.gml");
    assert_eq!(cityobjs.len(), 2);
    let TopLevelCityObject::PortZone(pz) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a PortZone");
    };

    assert_eq!(
        pz.function,
        vec![Code::new("臨港地区".to_string(), "30".to_string(),)]
    );

    assert_eq!(
        pz.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2009, 9, 18).unwrap())
    );

    assert_eq!(
        pz.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}

#[test]
fn load_urf_yoto_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533915_urf_6697_yoto_op.gml");
    assert_eq!(cityobjs.len(), 4);
    let TopLevelCityObject::UseDistrict(ud) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a UseDistrict");
    };

    assert_eq!(
        ud.function,
        vec![Code::new("工業専用地域".to_string(), "13".to_string(),)]
    );

    assert_eq!(
        ud.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2009, 11, 11).unwrap())
    );

    assert_eq!(
        ud.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}

#[test]
fn load_urf_boka_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533916_urf_6668_boka_op.gml");
    assert_eq!(cityobjs.len(), 2);
    let TopLevelCityObject::FirePreventionDistrict(fpd) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a FirePreventionDistrict");
    };

    assert_eq!(
        fpd.function,
        vec![Code::new("準防火地域".to_string(), "25".to_string(),)]
    );

    assert_eq!(
        fpd.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2009, 9, 18).unwrap())
    );

    assert_eq!(
        fpd.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}

#[test]
fn load_urf_seisan_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533923_urf_6668_seisan_op.gml");
    assert_eq!(cityobjs.len(), 27);
    let TopLevelCityObject::ProductiveGreenZone(pgz) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a ProductiveGreenZone");
    };

    assert_eq!(
        pgz.function,
        vec![Code::new("生産緑地地区".to_string(), "38".to_string(),)]
    );

    assert_eq!(
        pgz.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2010, 12, 21).unwrap())
    );

    assert_eq!(
        pgz.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}

#[test]
fn load_urf_tokuryoku_example() {
    let cityobjs =
        load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533923_urf_6668_tokuryoku_op.gml");
    assert_eq!(cityobjs.len(), 7);
    let TopLevelCityObject::SpecialGreenSpaceConservationDistrict(sgcd) =
        &cityobjs.first().unwrap().cityobj
    else {
        panic!("Not a SpecialGreenSpaceConservationDistrict");
    };

    assert_eq!(
        sgcd.function,
        vec![Code::new("特別緑地保存地区".to_string(), "35".to_string(),)]
    );

    assert_eq!(
        sgcd.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2022, 4, 7).unwrap())
    );

    assert_eq!(
        sgcd.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}
#[test]
fn load_urf_chusha_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533925_urf_6668_chusha_op.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::ParkingPlaceDevelopmentZone(ppdz) = &cityobjs.first().unwrap().cityobj
    else {
        panic!("Not a ParkingPlaceDevelopmentZone");
    };

    assert_eq!(
        ppdz.function,
        vec![Code::new("駐車場整備地区".to_string(), "29".to_string(),)]
    );

    assert_eq!(
        ppdz.urf_valid_from,
        Option::Some(Date::from_ymd_opt(1969, 3, 13).unwrap())
    );

    assert_eq!(
        ppdz.valid_from_type,
        Some(Code::new("決定".to_string(), "1".to_string()))
    );
}

#[test]
fn load_urf_tokuyoto_example() {
    let cityobjs =
        load_cityobjs("./tests/data/kawasaki-shi/udx/urf/533925_urf_6668_tokuyoto_op.gml");
    assert_eq!(cityobjs.len(), 4);
    let TopLevelCityObject::SpecialUseDistrict(sud) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a SpecialUseDistrict");
    };

    assert_eq!(
        sud.function,
        vec![Code::new("特別用途地区".to_string(), "14".to_string(),)]
    );

    assert_eq!(
        sud.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2005, 10, 7).unwrap())
    );

    assert_eq!(
        sud.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}

#[test]
fn load_urf_toshikeikaku_example() {
    let cityobjs = load_cityobjs("./tests/data/sapporo-shi/udx/urf/644131_urf_6668_op.gml");
    assert_eq!(cityobjs.len(), 12);
    let TopLevelCityObject::UrbanPlanningArea(upa) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a UrbanPlanningArea");
    };

    assert_eq!(
        upa.function,
        vec![Code::new("都市計画区域".to_string(), "21".to_string(),)]
    );

    assert_eq!(
        upa.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2022, 3, 23).unwrap())
    );

    assert_eq!(
        upa.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );
}

#[test]
fn load_urf_huchi_example() {
    let cityobjs = load_cityobjs("./tests/data/sendai-shi/udx/urf/574026_urf_6668_huchi_op.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::ScenicDistrict(sd) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a ScenicDistrict");
    };

    assert_eq!(
        sd.function,
        vec![Code::new("風致地区".to_string(), "28".to_string(),)]
    );

    assert_eq!(
        sd.prefecture,
        Some(Code::new("宮城県".to_string(), "04".to_string()))
    );

    assert_eq!(
        sd.city,
        Some(Code::new("宮城県仙台市".to_string(), "04100".to_string()))
    );
}

#[test]
fn load_urf_kodoriyou_example() {
    let cityobjs =
        load_cityobjs("./tests/data/sendai-shi/udx/urf/574027_urf_6697_kodoriyou_op.gml");
    assert_eq!(cityobjs.len(), 3);
    let TopLevelCityObject::HighLevelUseDistrict(hlud) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a HighLevelUseDistrict");
    };

    assert_eq!(
        hlud.function,
        vec![Code::new("高度利用地区".to_string(), "19".to_string(),)]
    );

    assert_eq!(
        hlud.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2013, 3, 8).unwrap())
    );

    assert_eq!(
        hlud.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );

    assert_eq!(hlud.custodian, Some("仙台市".to_string()));

    assert_eq!(
        hlud.prefecture,
        Some(Code::new("宮城県".to_string(), "04".to_string()))
    );

    assert_eq!(
        hlud.city,
        Some(Code::new("宮城県仙台市".to_string(), "04100".to_string()))
    );
}

#[test]
fn load_urf_keikan_example() {
    let cityobjs = load_cityobjs("./tests/data/sendai-shi/udx/urf/574036_urf_6668_keikan_op.gml");
    assert_eq!(cityobjs.len(), 2);
    let TopLevelCityObject::LandscapeZone(lz) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a LandscapeZone");
    };

    assert_eq!(
        lz.function,
        vec![Code::new("景観地区".to_string(), "27".to_string(),)]
    );

    assert_eq!(
        lz.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2011, 12, 16).unwrap())
    );

    assert_eq!(
        lz.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );

    assert_eq!(lz.custodian, Some("仙台市".to_string()));

    assert_eq!(
        lz.prefecture,
        Some(Code::new("宮城県".to_string(), "04".to_string()))
    );

    assert_eq!(
        lz.city,
        Some(Code::new("宮城県仙台市".to_string(), "04100".to_string()))
    );
}

#[test]
fn load_urf_tosisai_example() {
    let cityobjs = load_cityobjs("./tests/data/sendai-shi/udx/urf/574036_urf_6697_tosisai_op.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::SpecialUrbanRenaissanceDistrict(surd) =
        &cityobjs.first().unwrap().cityobj
    else {
        panic!("Not a SpecialUrbanRenaissanceDistrict");
    };

    assert_eq!(
        surd.function,
        vec![Code::new("都市再生特別地区".to_string(), "21".to_string(),)]
    );

    assert_eq!(
        surd.urf_valid_from,
        Option::Some(Date::from_ymd_opt(2020, 9, 16).unwrap())
    );

    assert_eq!(
        surd.valid_from_type,
        Some(Code::new("変更".to_string(), "3".to_string()))
    );

    assert_eq!(surd.custodian, Some("仙台市".to_string()));

    assert_eq!(
        surd.prefecture,
        Some(Code::new("宮城県".to_string(), "04".to_string()))
    );

    assert_eq!(
        surd.city,
        Some(Code::new("宮城県仙台市".to_string(), "04100".to_string()))
    );
}

#[test]
fn load_urf_sigaidev_example() {
    let cityobjs = load_cityobjs("./tests/data/kofu-shi/udx/urf/533834_urf_6668_sigaidev_op.gml");
    assert_eq!(cityobjs.len(), 27);
    let TopLevelCityObject::UrbanDevelopmentProject(udp) = &cityobjs.first().unwrap().cityobj
    else {
        panic!("Not a UrbanDevelopmentProject");
    };

    assert_eq!(
        udp.function,
        vec![Code::new("工業地域".to_string(), "12".to_string(),)]
    );

    assert_eq!(
        udp.urf_valid_from,
        Option::Some(Date::from_ymd_opt(1, 1, 1).unwrap())
    );

    assert_eq!(
        udp.valid_from_type,
        Some(Code::new("不明".to_string(), "4".to_string()))
    );

    assert_eq!(
        udp.prefecture,
        Some(Code::new("山梨県".to_string(), "19".to_string()))
    );

    assert_eq!(
        udp.city,
        Some(Code::new("山梨県甲府市".to_string(), "19201".to_string()))
    );
}

#[test]
fn load_vegetation_multisolid_example() {
    // Test MultiSolid geometry parsing for vegetation (PlantCover with lod1MultiSolid)
    let cityobjs = load_cityobjs("./tests/data/tsukuba-shi/udx/veg/54401008_veg_6697_op.gml");

    // Verify that PlantCover objects are loaded successfully
    assert_eq!(cityobjs.len(), 2);

    let TopLevelCityObject::PlantCover(plant_cover) = &cityobjs.first().unwrap().cityobj else {
        panic!("Expected PlantCover");
    };

    // todo: check solid id and surface id

    // Verify basic PlantCover attributes
    assert_eq!(plant_cover.class.as_ref().and_then(|c| c.code()), Some("5"));
}

#[test]
fn load_cityfurniture_curvemembers_example() {
    // Test CurveMembers parsing for CityFurniture with MultiCurve geometry
    // The GML contains: <gml:MultiCurve><gml:curveMembers><gml:Curve>...
    let cityobjs = load_cityobjs("./tests/data/nagaoka-shi/udx/frn/56380598_frn_6697_op.gml");

    assert_eq!(cityobjs.len(), 1);

    let geometries = &cityobjs.first().unwrap().geometries;

    // Verify that curveMembers with Curve containing LineStringSegment was parsed as multilinestring
    assert_eq!(geometries.multilinestring.len(), 1);
}

#[test]
fn load_track_lod0network_example() {
    // Test Track with lod0Network containing CompositeCurve
    // The GML structure: <tran:lod0Network><gml:CompositeCurve><gml:curveMember>...
    let cityobjs = load_cityobjs("./tests/data/mikurajima-mura/udx/trk/50396427_trk_6697_op.gml");

    assert_eq!(cityobjs.len(), 1);

    let TopLevelCityObject::Track(track) = &cityobjs.first().unwrap().cityobj else {
        panic!("Expected Track");
    };

    // Verify basic Track attributes
    assert_eq!(track.class.as_ref().map(|c| c.value()), Some("1020"));
    assert_eq!(track.function.first().map(|c| c.value()), Some("9020"));

    let geometries = &cityobjs.first().unwrap().geometries;

    // Verify that lod0Network with CompositeCurve was parsed as multilinestring
    assert_eq!(geometries.multilinestring.len(), 1);
}
