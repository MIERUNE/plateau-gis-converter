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
pub use polygon::{Polygon, Polygon2, Polygon3};

use num_traits::{Num, NumCast};

pub trait CoordNum: Num + Copy + NumCast + PartialOrd + Default + Debug {}
impl<T: Num + Copy + NumCast + PartialOrd + Default + Debug> CoordNum for T {}

/// Computer-friendly Geometry
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum Geometry<'a, const D: usize, T: CoordNum> {
    MultiPoint(MultiPoint<'a, D, T>),
    LineString(LineString<'a, D, T>),
    MultiLineString(MultiLineString<'a, D, T>),
    Polygon(Polygon<'a, D, T>),
    MultiPolygon(MultiPolygon<'a, D, T>),
}

pub type Geometry2<'a, T = f64> = Geometry<'a, 2, T>;
pub type Geometry3<'a, T = f64> = Geometry<'a, 3, T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_num_trait() {
        // 2D LineString with floating point numbers
        let a: LineString<2, f32> = LineString::from_raw(vec![1.2, 2.3, 3.4, 4.5].into());
        assert_eq!(a.len(), 2);
        // Can also be used to store integer values (e.g., vertex indices)
        let b: LineString<1, u32> = LineString::from_raw(vec![1, 2, 3].into());
        assert_eq!(b.len(), 3);
    }
}
