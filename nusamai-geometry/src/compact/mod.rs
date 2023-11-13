mod linestring;
mod multi_linestring;
mod multi_point;
mod multi_polygon;
mod polygon;

pub use linestring::{CompactLineString, CompactLineString2, CompactLineString3};
pub use multi_linestring::{
    CompactMultiLineString, CompactMultiLineString2, CompactMultiLineString3,
};
pub use multi_point::{CompactMultiPoint, CompactMultiPoint2, CompactMultiPoint3};
pub use multi_polygon::{CompactMultiPolygon, CompactMultiPolygon2, CompactMultiPolygon3};
pub use polygon::{CompactPolygon, CompactPolygon2, CompactPolygon3};

use num_traits::Float;

/// Computer-friendly Geometry
#[derive(Debug, Clone)]
pub enum CompactGeometry<'a, const D: usize, T: Float> {
    LineString(CompactMultiPolygon<'a, D, T>),
    Polygon(CompactLineString<'a, D, T>),
    MultiPoint(CompactMultiPoint<'a, D, T>),
    MultiLineString(CompactMultiLineString<'a, D, T>),
    MultiPolygon(CompactMultiPolygon<'a, D, T>),
}

pub type CompactGeometry2<'a, T> = CompactGeometry<'a, 2, T>;
pub type CompactGeometry3<'a, T> = CompactGeometry<'a, 3, T>;
