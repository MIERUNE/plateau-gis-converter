mod linestring;
mod multi_linestring;
mod multi_point;
mod multi_polygon;
mod polygon;

pub use linestring::{LineString, LineString2, LineString3};
pub use multi_linestring::{MultiLineString, MultiLineString2, MultiLineString3};
pub use multi_point::{MultiPoint, MultiPoint2, MultiPoint3};
pub use multi_polygon::{MultiPolygon, MultiPolygon2, MultiPolygon3};
pub use polygon::{Polygon, Polygon2, Polygon3};

use num_traits::Float;

/// Computer-friendly Geometry
#[derive(Debug, Clone)]
pub enum Geometry<'a, const D: usize, T: Float> {
    LineString(MultiPolygon<'a, D, T>),
    Polygon(LineString<'a, D, T>),
    MultiPoint(MultiPoint<'a, D, T>),
    MultiLineString(MultiLineString<'a, D, T>),
    MultiPolygon(MultiPolygon<'a, D, T>),
}

pub type Geometry2<'a, T> = Geometry<'a, 2, T>;
pub type Geometry3<'a, T> = Geometry<'a, 3, T>;
