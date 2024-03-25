mod linestring;
mod multi_linestring;
mod multi_point;
mod multi_polygon;
mod polygon;

use core::fmt::Debug;

pub use linestring::{LineString, LineString2, LineString3};
pub use multi_linestring::{MultiLineString, MultiLineString2, MultiLineString3};
pub use multi_point::{MultiPoint, MultiPoint2, MultiPoint3};
pub use multi_polygon::{MultiPolygon, MultiPolygon2, MultiPolygon3};
use num_traits::{Num, NumCast};
pub use polygon::{Polygon, Polygon2, Polygon3};

pub trait Coord: Clone + PartialEq {}

impl<const D: usize> Coord for [f64; D] {}
impl<const D: usize> Coord for [f32; D] {}
impl Coord for f64 {}
impl Coord for f32 {}
impl Coord for u64 {}
impl Coord for u32 {}

pub trait Coord2d: Coord {
    fn xy(&self) -> (f64, f64);
}

impl Coord2d for [f64; 2] {
    fn xy(&self) -> (f64, f64) {
        (self[0], self[1])
    }
}

impl Coord2d for [f32; 2] {
    fn xy(&self) -> (f64, f64) {
        (self[0] as f64, self[1] as f64)
    }
}

pub trait CoordNum: Num + Copy + NumCast + PartialOrd + Default + Debug {}
impl<T: Num + Copy + NumCast + PartialOrd + Default + Debug> CoordNum for T {}

/// Computer-friendly Geometry
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
#[derive(Debug, Clone)]
pub enum Geometry<'a, T: Coord> {
    MultiPoint(MultiPoint<'a, T>),
    LineString(LineString<'a, T>),
    MultiLineString(MultiLineString<'a, T>),
    Polygon(Polygon<'a, T>),
    MultiPolygon(MultiPolygon<'a, T>),
}

pub type Geometry2<'a, C = f64> = Geometry<'a, [C; 2]>;
pub type Geometry3<'a, C = f64> = Geometry<'a, [C; 2]>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_num_trait() {
        // 2D LineString with floating point numbers
        let a: LineString2<f32> = LineString::from_raw(vec![[1.2, 2.3], [3.4, 4.5]].into());
        assert_eq!(a.len(), 2);
        // Can also be used to store integer values (e.g., vertex indices)
        let b: LineString<u32> = LineString::from_raw(vec![1, 2, 3].into());
        assert_eq!(b.len(), 3);
    }
}
