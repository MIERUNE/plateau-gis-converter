use std::fmt::Display;

use flatgeom::{MultiLineString, MultiPoint, MultiPolygon};
use nusamai_projection::crs::*;

use crate::LocalId;

/// URI prefix for EPSG codes
const CRS_URI_EPSG_PREFIX: &str = "http://www.opengis.net/def/crs/EPSG/0/";

#[derive(Debug, Clone, Copy)]
pub enum GeometryParseType {
    Geometry,
    Solid,
    MultiSurface,
    MultiCurve,
    MultiPoint,
    Surface,
    Point,
    Triangulated,
    CompositeCurve,
}

/// GML geometry types as they appear in the XML
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GmlGeometryType {
    // Polygonal types
    Solid,
    MultiSurface,
    CompositeSurface,
    OrientableSurface,
    Polygon,
    Surface,
    TriangulatedSurface,
    Tin,

    // Linear types
    LineString,
    MultiCurve,
    CompositeCurve,

    // Point types
    Point,
    MultiPoint,

    // Generic
    Geometry,
    MultiGeometry,
}

impl GmlGeometryType {
    /// Parse from a string slice (XML element local name)
    /// This function is designed to return an option since whether an object has geometric types depends on the flattening settings.
    pub fn maybe_from_str(s: &str) -> Option<Self> {
        match s {
            "Solid" => Some(Self::Solid),
            "MultiSurface" => Some(Self::MultiSurface),
            "CompositeSurface" => Some(Self::CompositeSurface),
            "OrientableSurface" => Some(Self::OrientableSurface),
            "Polygon" => Some(Self::Polygon),
            "Surface" => Some(Self::Surface),
            "TriangulatedSurface" => Some(Self::TriangulatedSurface),
            "Tin" => Some(Self::Tin),
            "LineString" => Some(Self::LineString),
            "MultiCurve" => Some(Self::MultiCurve),
            "CompositeCurve" => Some(Self::CompositeCurve),
            "Point" => Some(Self::Point),
            "MultiPoint" => Some(Self::MultiPoint),
            "Geometry" => Some(Self::Geometry),
            "MultiGeometry" => Some(Self::MultiGeometry),
            _ => None,
        }
    }
}

impl Display for GmlGeometryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Solid => "Solid",
            Self::MultiSurface => "MultiSurface",
            Self::CompositeSurface => "CompositeSurface",
            Self::OrientableSurface => "OrientableSurface",
            Self::Polygon => "Polygon",
            Self::Surface => "Surface",
            Self::TriangulatedSurface => "TriangulatedSurface",
            Self::Tin => "Tin",
            Self::LineString => "LineString",
            Self::MultiCurve => "MultiCurve",
            Self::CompositeCurve => "CompositeCurve",
            Self::Point => "Point",
            Self::MultiPoint => "MultiPoint",
            Self::Geometry => "Geometry",
            Self::MultiGeometry => "MultiGeometry",
        };
        write!(f, "{s}")
    }
}

/// CityGML property names that contain geometry
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PropertyType {
    // Standard LOD properties
    Lod0Point,
    Lod0MultiCurve,
    Lod2MultiCurve,
    Lod3MultiCurve,
    Lod4MultiCurve,

    Lod1Solid,
    Lod2Solid,
    Lod3Solid,
    Lod4Solid,

    Lod0MultiSurface,
    Lod1MultiSurface,
    Lod2MultiSurface,
    Lod3MultiSurface,
    Lod4MultiSurface,

    Lod0Geometry,
    Lod1Geometry,
    Lod2Geometry,
    Lod3Geometry,
    Lod4Geometry,

    // Special properties
    Lod0RoofEdge,
    Lod0FootPrint,
    Lod0Network,
    Lod2Network,
    Lod3Network,
    Lod2Surface,
    Lod3Surface,
    Tin,
}

impl PropertyType {
    /// Parse from a string slice (property name without namespace)
    /// This function is designed to return an option since whether an object has geometric properties depends on the flattening settings.
    pub fn maybe_from_str(s: &str) -> Option<Self> {
        let out = match s {
            "lod0Point" => Self::Lod0Point,
            "lod0MultiCurve" => Self::Lod0MultiCurve,
            "lod2MultiCurve" => Self::Lod2MultiCurve,
            "lod3MultiCurve" => Self::Lod3MultiCurve,
            "lod4MultiCurve" => Self::Lod4MultiCurve,

            "lod1Solid" => Self::Lod1Solid,
            "lod2Solid" => Self::Lod2Solid,
            "lod3Solid" => Self::Lod3Solid,
            "lod4Solid" => Self::Lod4Solid,

            "lod0MultiSurface" => Self::Lod0MultiSurface,
            "lod1MultiSurface" => Self::Lod1MultiSurface,
            "lod2MultiSurface" => Self::Lod2MultiSurface,
            "lod3MultiSurface" => Self::Lod3MultiSurface,
            "lod4MultiSurface" => Self::Lod4MultiSurface,

            "lod0Geometry" => Self::Lod0Geometry,
            "lod1Geometry" => Self::Lod1Geometry,
            "lod2Geometry" => Self::Lod2Geometry,
            "lod3Geometry" => Self::Lod3Geometry,
            "lod4Geometry" => Self::Lod4Geometry,

            "lod0RoofEdge" => Self::Lod0RoofEdge,
            "lod0FootPrint" => Self::Lod0FootPrint,
            "lod0Network" => Self::Lod0Network,
            "lod2Network" => Self::Lod2Network,
            "lod3Network" => Self::Lod3Network,
            "lod2Surface" => Self::Lod2Surface,
            "lod3Surface" => Self::Lod3Surface,
            "tin" => Self::Tin,

            &_ => return None,
        };
        Some(out)
    }
}

impl Display for PropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Lod0Point => "lod0Point",
            Self::Lod0MultiCurve => "lod0MultiCurve",
            Self::Lod2MultiCurve => "lod2MultiCurve",
            Self::Lod3MultiCurve => "lod3MultiCurve",
            Self::Lod4MultiCurve => "lod4MultiCurve",

            Self::Lod1Solid => "lod1Solid",
            Self::Lod2Solid => "lod2Solid",
            Self::Lod3Solid => "lod3Solid",
            Self::Lod4Solid => "lod4Solid",

            Self::Lod0MultiSurface => "lod0MultiSurface",
            Self::Lod1MultiSurface => "lod1MultiSurface",
            Self::Lod2MultiSurface => "lod2MultiSurface",
            Self::Lod3MultiSurface => "lod3MultiSurface",
            Self::Lod4MultiSurface => "lod4MultiSurface",

            Self::Lod0Geometry => "lod0Geometry",
            Self::Lod1Geometry => "lod1Geometry",
            Self::Lod2Geometry => "lod2Geometry",
            Self::Lod3Geometry => "lod3Geometry",
            Self::Lod4Geometry => "lod4Geometry",

            Self::Lod0RoofEdge => "lod0RoofEdge",
            Self::Lod0FootPrint => "lod0FootPrint",
            Self::Lod0Network => "lod0Network",
            Self::Lod2Network => "lod2Network",
            Self::Lod3Network => "lod3Network",
            Self::Lod2Surface => "lod2Surface",
            Self::Lod3Surface => "lod3Surface",
            Self::Tin => "tin",
        };
        write!(f, "{s}")
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GeometryType {
    /// Polygons (solids)
    Solid,
    /// Polygons (surfaces)
    Surface,
    /// Polygons (triangles)
    Triangle,
    /// Line-strings
    Curve,
    /// Points
    Point,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GeometryRef {
    pub id: Option<LocalId>,
    #[serde(rename = "type")]
    pub ty: GeometryType,
    /// CityGML property name (e.g., Lod2MultiSurface, Lod1Solid, Lod3Geometry)
    pub property_name: Option<PropertyType>,
    /// GML geometry type (e.g., MultiSurface, Polygon, CompositeSurface)
    pub gml_geometry_type: Option<GmlGeometryType>,
    pub lod: u8,
    pub pos: u32,
    pub len: u32,
    pub solid_ids: Vec<LocalId>,
    pub feature_id: Option<String>,
    pub feature_type: Option<String>,
}

pub type GeometryRefs = Vec<GeometryRef>;

/// Geometries in a single city object and all its children.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Default)]
pub struct GeometryStore {
    /// EPSG code of the Coordinate Reference System (CRS) for this geometry
    pub epsg: EpsgCode,

    /// Shared vertex buffer for all geometries in this store
    pub vertices: Vec<[f64; 3]>,

    /// All polygons, referenced by `GeometryRefs`
    pub multipolygon: MultiPolygon<'static, u32>,
    /// All line-strings, referenced by `GeometryRefs`
    pub multilinestring: MultiLineString<'static, u32>,
    /// All points, referenced by `GeometryRefs`
    pub multipoint: MultiPoint<'static, u32>,

    /// Ring ids of the all polygons
    pub ring_ids: Vec<Option<LocalId>>,
    /// List of surface ids and their spans in `multipolygon`
    pub surface_spans: Vec<SurfaceSpan>,

    /// Lists of surface for composite surface
    pub composite_surfaces: Vec<LocalId>,

    /// Orientable surfaces for each surface
    pub orientable_surfaces: Vec<OrientableSurface>,

    /// Assigned materials for each polygon. Empty if appearance resolution is not enabled.
    pub polygon_materials: Vec<Option<u32>>,
    /// Assigned textures for each polygon. Empty if appearance resolution is not enabled.
    pub polygon_textures: Vec<Option<u32>>,
    /// Assigned texture UVs for each polygon. Empty if appearance resolution is not enabled.
    pub polygon_uvs: MultiPolygon<'static, [f64; 2]>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SurfaceSpan {
    pub id: LocalId,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct OrientableSurface {
    pub surface_id: LocalId,
    pub reverse: bool,
}

/// Temporary storage for the parser to collect geometries.
#[derive(Default)]
pub(crate) struct GeometryCollector {
    pub vertices: indexmap::IndexSet<[u64; 3], ahash::RandomState>,
    pub geometry_crs_uri: Option<String>,
    pub multipolygon: MultiPolygon<'static, u32>,
    pub multilinestring: MultiLineString<'static, u32>,
    pub multipoint: MultiPoint<'static, u32>,

    /// ring ids of the all polygons
    pub ring_ids: Vec<Option<LocalId>>,

    /// surface polygon spans in `multipolygon`
    pub surface_spans: Vec<SurfaceSpan>,

    /// Lists of surface for composite surface
    pub composite_surfaces: Vec<LocalId>,

    /// Orientable surfaces for each surface
    pub orientable_surfaces: Vec<OrientableSurface>,
}

impl GeometryCollector {
    pub fn add_exterior_ring(
        &mut self,
        iter: impl IntoIterator<Item = [f64; 3]>,
        ring_id: Option<LocalId>,
    ) {
        self.ring_ids.push(ring_id);
        self.multipolygon.add_exterior(iter.into_iter().map(|v| {
            let vbits = [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()];
            let (index, _) = self.vertices.insert_full(vbits);
            index as u32
        }));
    }

    pub fn add_interior_ring(
        &mut self,
        iter: impl IntoIterator<Item = [f64; 3]>,
        ring_id: Option<LocalId>,
    ) {
        self.ring_ids.push(ring_id);
        self.multipolygon.add_interior(iter.into_iter().map(|v| {
            let vbits = [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()];
            let (index, _) = self.vertices.insert_full(vbits);
            index as u32
        }));
    }

    pub fn add_linestring(&mut self, iter: impl IntoIterator<Item = [f64; 3]>) {
        self.multilinestring
            .add_linestring(iter.into_iter().map(|v| {
                let vbits = [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()];
                let (index, _) = self.vertices.insert_full(vbits);
                index as u32
            }));
    }

    pub fn add_point(&mut self, point: [f64; 3]) {
        let vbits = [point[0].to_bits(), point[1].to_bits(), point[2].to_bits()];
        let (index, _) = self.vertices.insert_full(vbits);
        self.multipoint.push(index as u32);
    }

    pub fn into_geometries(self, envelope_crs_uri: Option<String>) -> GeometryStore {
        let mut vertices = Vec::with_capacity(self.vertices.len());
        for vbits in &self.vertices {
            vertices.push([
                f64::from_bits(vbits[0]),
                f64::from_bits(vbits[1]),
                f64::from_bits(vbits[2]),
            ]);
        }

        let crs_uri = envelope_crs_uri.unwrap_or(self.geometry_crs_uri.unwrap_or_default());

        let epsg = if crs_uri.starts_with(CRS_URI_EPSG_PREFIX) {
            if let Some(stripped) = crs_uri.strip_prefix(CRS_URI_EPSG_PREFIX) {
                stripped.parse::<EpsgCode>().ok()
            } else {
                None
            }
        } else {
            None
        }
        .unwrap_or(EPSG_JGD2011_GEOGRAPHIC_3D);

        GeometryStore {
            epsg,
            vertices,
            multipolygon: self.multipolygon,
            multilinestring: self.multilinestring,
            multipoint: self.multipoint,
            ring_ids: self.ring_ids,
            surface_spans: self.surface_spans,
            composite_surfaces: self.composite_surfaces,
            orientable_surfaces: self.orientable_surfaces,
            ..Default::default()
        }
    }
}
