#[cfg(feature = "serde")]
mod tests {
    use nusamai_geometry::{
        Geometry2, LineString3, MultiLineString2, MultiPoint3, MultiPolygon2, Polygon2,
    };

    #[derive(serde::Serialize, serde::Deserialize)]
    struct MyStruct {
        mpoly: MultiPolygon2<'static>,
        poly: Polygon2<'static>,
        line: LineString3<'static>,
        mline: MultiLineString2<'static>,
        mpoint: MultiPoint3<'static>,
        geom: Geometry2<'static>,
    }

    #[test]
    fn test_serde_serialize_deserialize() {
        let m = MyStruct {
            mpoly: Default::default(),
            poly: Default::default(),
            mline: Default::default(),
            line: Default::default(),
            mpoint: Default::default(),
            geom: Geometry2::MultiPoint(Default::default()),
        };
        let serialized = serde_json::to_string(&m).unwrap();
        let _: MyStruct = serde_json::from_str(&serialized).unwrap();
    }
}
