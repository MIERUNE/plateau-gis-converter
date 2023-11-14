use std::borrow::Cow;

use super::linestring::LineString;
use num_traits::Float;

/// Computer-friendly Polygon
#[derive(Debug, Clone, Default)]
pub struct Polygon<'a, const D: usize, T: Float = f64> {
    /// 座標データ
    coords: Cow<'a, [T]>,
    /// 各 hole が何番目の頂点から始まるかの列
    hole_indices: Cow<'a, [u32]>,
}

pub type Polygon3<'a, T> = Polygon<'a, 3, T>;
pub type Polygon2<'a, T> = Polygon<'a, 2, T>;

impl<'a, const D: usize, T: Float> Polygon<'a, D, T> {
    pub fn new(all_coords: Cow<'a, [T]>, hole_indices: Cow<'a, [u32]>) -> Self {
        Self {
            coords: all_coords,
            hole_indices,
        }
    }

    #[inline]
    pub fn coords(&self) -> &[T] {
        self.coords.as_ref()
    }

    #[inline]
    pub fn hole_indices(&self) -> &[u32] {
        self.hole_indices.as_ref()
    }

    #[inline]
    pub fn exterior(&self) -> LineString<D, T> {
        LineString::new(if self.hole_indices.is_empty() {
            self.coords.as_ref().into()
        } else {
            self.coords[..self.hole_indices[0] as usize * D].into()
        })
    }

    pub fn interiors(&self) -> impl Iterator<Item = LineString<D, T>> {
        self.hole_indices
            .windows(2)
            .map(|a| (a[0] as usize * D, a[1] as usize * D))
            .chain(match self.hole_indices.is_empty() {
                true => None,
                false => Some((
                    self.hole_indices[self.hole_indices.len() - 1] as usize * D,
                    self.coords.len(),
                )),
            })
            .map(|(start, end)| LineString::new(self.coords[start..end].into()))
    }

    #[inline]
    pub fn clear(&mut self) {
        self.coords.to_mut().clear();
        self.hole_indices.to_mut().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_empty() {
        let polygon: Polygon2<f64> = Default::default();
        assert_eq!(polygon.exterior().len(), 0);
        assert_eq!(polygon.interiors().count(), 0);
    }

    #[test]
    fn test_polygon_no_interior() {
        let all_coords: Vec<f64> = (0..=10).flat_map(|i| vec![i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![];
        let polygon: Polygon2<f64> = Polygon2::new(all_coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 11);
        assert_eq!(polygon.interiors().count(), 0);
        for _ in polygon.interiors() {
            unreachable!();
        }
    }

    #[test]
    fn test_polygon_one_interior() {
        let all_coords: Vec<f64> = (0..=10).flat_map(|i| vec![i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![4];
        let polygon: Polygon2<f64> = Polygon2::new(all_coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 4);
        assert_eq!(polygon.interiors().count(), 1);
        for (i, interior) in polygon.interiors().enumerate() {
            match i {
                0 => assert_eq!(interior.len(), 7),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_polygon_multiple_interiors() {
        let all_coords: Vec<f64> = (0..=10).flat_map(|i| vec![i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![4, 7];
        let polygon: Polygon2<f64> = Polygon2::new(all_coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 4);
        assert_eq!(polygon.interiors().count(), 2);
        for (i, interior) in polygon.interiors().enumerate() {
            match i {
                0 => assert_eq!(interior.len(), 3),
                1 => assert_eq!(interior.len(), 4),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_polygon_clear() {
        let all_coords: Vec<f64> = (0..=10).flat_map(|i| vec![i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![4, 7];
        let mut polygon: Polygon2<f64> = Polygon2::new(all_coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 4);
        assert_eq!(polygon.interiors().count(), 2);

        polygon.clear();
        assert_eq!(polygon.exterior().len(), 0);
        assert_eq!(polygon.interiors().count(), 0);
    }

    /// Currently, it does not check whether the exterior is valid (have at least three vertices) or not
    #[test]
    fn test_polygon_invalid_exterior() {
        let all_coords: Vec<f64> = (0..1).flat_map(|i| vec![i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![];
        let polygon: Polygon2<f64> = Polygon2::new(all_coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 1); // only one vertex
        assert_eq!(polygon.interiors().count(), 0);
    }

    /// Currently, it does not check whether the interior is valid (have at least three vertices) or not
    #[test]
    fn test_polygon_invalid_interior() {
        let all_coords: Vec<f64> = (0..=10).flat_map(|i| vec![i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![10]; // only one vertex
        let polygon: Polygon2<f64> = Polygon2::new(all_coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 10);
        assert_eq!(polygon.interiors().count(), 1);
        for (i, interior) in polygon.interiors().enumerate() {
            match i {
                0 => assert_eq!(interior.len(), 1), // only one vertex
                _ => unreachable!(),
            }
        }
    }

    /// Currently, it does not check whether the `hole_indices` value is valid (within the range of `all_coords`) or not
    #[test]
    fn test_polygon_invalid_hole_indices_1() {
        let all_coords: Vec<f64> = (0..=2).flat_map(|i| vec![i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![3]; // out of `all_coords` range
        let polygon: Polygon2<f64> = Polygon2::new(all_coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 3);
        assert_eq!(polygon.interiors().count(), 1);
        for (i, interior) in polygon.interiors().enumerate() {
            match i {
                0 => assert_eq!(interior.len(), 0), // zero vertex
                _ => unreachable!(),
            }
        }
    }

    /// Currently, it does not check whether the `hole_indices` value is valid (within the range of `all_coords`) or not
    #[test]
    #[should_panic]
    fn test_polygon_invalid_hole_indices_2() {
        let all_coords: Vec<f64> = (0..=2).flat_map(|i| vec![i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![99]; // out of `all_coords` range
        let polygon: Polygon2<f64> = Polygon2::new(all_coords.into(), hole_indices.into());

        polygon.exterior(); // panic, as `all_coords[0..hole_indices[0]]` is invalid
    }
}
