use nusamai_geometry::{MultiLineString, MultiPolygon};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy)]
pub enum GeometryParseType {
    Geometry,
    Solid,
    MultiSurface,
    MultiCurve,
    Triangulated,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, Default)]
pub enum GeometryType {
    #[default]
    Unknown,
    Solid,
    Surface,
    Triangle,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct GeometryRefEntry {
    #[serde(rename = "type")]
    pub ty: GeometryType,
    pub lod: u8,
    pub pos: u32,
    pub len: u32,
}

pub type GeometryRef = Vec<GeometryRefEntry>;

/// Geometries in a toplevel city object and its children.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Default)]
pub struct Geometries {
    pub vertices: Vec<[f64; 3]>,
    pub multipolygon: MultiPolygon<'static, 1, u32>,
    pub multilinestring: MultiLineString<'static, 1, u32>,
}

/// Store for collecting vertices and polygons from GML.
#[derive(Default)]
pub struct GeometryCollector {
    pub vertices: indexmap::IndexSet<[u64; 3]>,
    pub multipolygon: MultiPolygon<'static, 1, u32>,
    pub multilinestring: MultiLineString<'static, 1, u32>,
}

impl GeometryCollector {
    pub fn add_exterior_ring(&mut self, iter: impl Iterator<Item = [f64; 3]>) -> usize {
        self.multipolygon.add_exterior(iter.map(|v| {
            // ...
            let vbits = [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()];
            let (index, _) = self.vertices.insert_full(vbits);
            [index as u32]
        }));

        self.multipolygon.len() - 1
    }

    pub fn add_interior_ring(&mut self, iter: impl Iterator<Item = [f64; 3]>) {
        self.multipolygon.add_interior(iter.map(|v| {
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
            multipolygon: self.multipolygon.clone(),
            multilinestring: self.multilinestring.clone(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.multipolygon.clear();
        self.multilinestring.clear();
    }
}
