pub mod common;

use common::load_cityobjs;
use nusamai_citygml::{Code, Measure};
use nusamai_plateau::models::{uro, waterbody, TopLevelCityObject};

#[test]
fn test_load_urf_example() {
    let cityobjs = load_cityobjs("./tests/data/takeo-shi/udx/urf/493060_urf_6668_op.gml");
    assert_eq!(cityobjs.len(), 140);

    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/urf/523857_urf_6668_op.gml");
    assert_eq!(cityobjs.len(), 47);

    let cityobjs = load_cityobjs("./tests/data/tokyo23-ku/udx/urf/533957_urf_6668_op.gml");
    assert_eq!(cityobjs.len(), 38);
}
