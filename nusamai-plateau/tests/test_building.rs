mod utils;

use nusamai_plateau::models::TopLevelCityObject;

use utils::load_cityobjs_from_zstd;

#[test]
fn test_load_building_lod4_example() {
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
