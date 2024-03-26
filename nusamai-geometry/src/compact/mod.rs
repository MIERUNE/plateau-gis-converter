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

use num_traits::ToPrimitive;

pub trait Coord: Clone + PartialEq {}
pub trait CoordNum: ToPrimitive + PartialEq + Clone {}

impl<const D: usize, N: CoordNum> Coord for [N; D] {}
impl<N: CoordNum> Coord for N {}
impl CoordNum for f32 {}
impl CoordNum for f64 {}
impl CoordNum for u8 {}
impl CoordNum for i8 {}
impl CoordNum for u16 {}
impl CoordNum for i16 {}
impl CoordNum for u32 {}
impl CoordNum for i32 {}
impl CoordNum for u64 {}
impl CoordNum for i64 {}

pub trait Coord2d: Coord {
    fn xy(&self) -> (f64, f64);
}

impl<N: CoordNum> Coord2d for [N; 2] {
    fn xy(&self) -> (f64, f64) {
        (self[0].to_f64().unwrap(), self[1].to_f64().unwrap())
    }
}

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
pub type Geometry3<'a, C = f64> = Geometry<'a, [C; 3]>;

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

    #[test]
    fn coord2d() {
        let v = [1, 2];
        assert_eq!(v.xy(), (1.0, 2.0));

        let a: LineString2<f32> =
            LineString::from_raw(vec![[0.0, 0.0], [1.0, 1.0], [-2.0, 3.0]].into());
        assert_eq!(a.ring_area(), 2.5);
    }
}
