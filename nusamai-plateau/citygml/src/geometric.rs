use nusamai_geometry::{MultiLineString, MultiPoint3, MultiPolygon};

pub struct FeatureGeometries {
    pub vertices: MultiPoint3<'static>,
    pub polygons: MultiPolygon<'static, 1, u32>,
    pub linestrings: MultiLineString<'static, 1, u32>,
}

pub struct GeometryIndices {
    pub lod0_surfaces: Vec<u32>,
    pub lod1_surfaces: Vec<u32>,
    pub lod2_surfaces: Vec<u32>,
    pub lod3_surfaces: Vec<u32>,
    pub lod4_surfaces: Vec<u32>,
    pub lod0_curves: Vec<u32>,
    // pub lod2_curves: Vec<u32>,
    // pub lod3_curves: Vec<u32>,
    // pub lod4_curves: Vec<u32>,
}
