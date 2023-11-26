use nusamai_geometry::{MultiLineString, MultiPolygon};

#[derive(Debug, Clone, Copy)]
pub enum GeometryType {
    Geometry,
    Solid,
    MultiSurface,
    MultiCurve,
    Triangulated,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Default)]
pub struct GeometryReference {
    // Note: Is it necessary to keep solid distinct from multisurface?
    // pub lod1_solid: Vec<u32>,
    // pub lod2_solid: Vec<u32>,
    // pub lod3_solid: Vec<u32>,
    // pub lod4_solid: Vec<u32>,
    //
    //
    pub lod0_surfaces: Vec<u32>,
    pub lod1_surfaces: Vec<u32>,
    pub lod2_surfaces: Vec<u32>,
    pub lod3_surfaces: Vec<u32>,
    pub lod4_surfaces: Vec<u32>,
    //
    //
    // pub lod0_curves: Vec<u32>,
    // pub lod2_curves: Vec<u32>,
    // pub lod3_curves: Vec<u32>,
    // pub lod4_curves: Vec<u32>,
    //
    // pub lod0_point: Vec<u32>,
}

impl GeometryReference {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Geometries in a toplevel city object and its children.
pub struct Geometries {
    pub vertices: Vec<[f64; 3]>,
    pub polygons: MultiPolygon<'static, 1, u32>,
    pub linestrings: MultiLineString<'static, 1, u32>,
}

/// Store for collecting vertices and polygons from GML.
#[derive(Default)]
pub struct GeometryCollector {
    pub vertices: indexmap::IndexSet<[u64; 3]>,
    pub multi_polygon: MultiPolygon<'static, 1, u32>,
    pub multi_linestring: MultiLineString<'static, 1, u32>,
}

impl GeometryCollector {
    pub fn add_exterior_ring(&mut self, iter: impl Iterator<Item = [f64; 3]>) -> usize {
        self.multi_polygon.add_exterior(iter.map(|v| {
            // ...
            let vbits = [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()];
            let (index, _) = self.vertices.insert_full(vbits);
            [index as u32]
        }));

        self.multi_polygon.len() - 1
    }

    pub fn add_interior_ring(&mut self, iter: impl Iterator<Item = [f64; 3]>) {
        self.multi_polygon.add_interior(iter.map(|v| {
            // ...
            let vbits = [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()];
            let (index, _) = self.vertices.insert_full(vbits);
            [index as u32]
        }));
    }

    pub fn to_geometries(&self) -> Geometries {
        let mut vertices = Vec::with_capacity(self.vertices.len());
        for vbits in &self.vertices {
            vertices.push([
                f64::from_bits(vbits[0]),
                f64::from_bits(vbits[1]),
                f64::from_bits(vbits[2]),
            ]);
        }
        Geometries {
            vertices,
            polygons: self.multi_polygon.clone(),
            linestrings: self.multi_linestring.clone(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.multi_polygon.clear();
        self.multi_linestring.clear();
    }
}
