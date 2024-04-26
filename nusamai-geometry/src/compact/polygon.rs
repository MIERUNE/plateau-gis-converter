use std::{borrow::Cow, hash::Hash};

use crate::Coord2d;

use super::{linestring::LineString, Coord};

/// Computer-friendly Polygon
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Polygon<'a, T: Coord> {
    /// Coordinates
    coords: Cow<'a, [T]>,

    /// A sequence of indices from which each hole starts
    hole_indices: Cow<'a, [u32]>,
}

pub type Polygon3<'a, C = f64> = Polygon<'a, [C; 3]>;
pub type Polygon2<'a, C = f64> = Polygon<'a, [C; 2]>;

impl PartialEq for Polygon3<'_, f64> {
    fn eq(&self, other: &Self) -> bool {
        self.exterior() == other.exterior()
            && self.interiors().zip(other.interiors()).all(|(a, b)| a == b)
    }
}

impl PartialEq for Polygon2<'_, f64> {
    fn eq(&self, other: &Self) -> bool {
        self.exterior() == other.exterior()
            && self.interiors().zip(other.interiors()).all(|(a, b)| a == b)
    }
}

impl Hash for Polygon2<'_, f64> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.exterior().hash(state);
        for interior in self.interiors() {
            interior.hash(state);
        }
    }
}

impl Hash for Polygon3<'_, f64> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.exterior().hash(state);
        for interior in self.interiors() {
            interior.hash(state);
        }
    }
}

impl Eq for Polygon3<'_, f64> {}
impl Eq for Polygon2<'_, f64> {}

impl<'a, T: Coord> Polygon<'a, T> {
    /// Creates an empty Polygon.
    pub fn new() -> Self {
        Self {
            coords: Cow::Borrowed(&[]),
            hole_indices: Cow::Borrowed(&[]),
        }
    }

    pub fn from_raw(coords: Cow<'a, [T]>, hole_indices: Cow<'a, [u32]>) -> Self {
        // Check if all span values are within range and monotonically increasing
        if let Some(&last) = hole_indices.last() {
            let len = (coords.len()) as u32;
            if last >= len || hole_indices.windows(2).any(|a| a[0] >= a[1]) {
                panic!("invalid hole_indices")
            }
        }
        Self {
            coords,
            hole_indices,
        }
    }

    pub fn from_raw_unchecked(coords: Cow<'a, [T]>, hole_indices: Cow<'a, [u32]>) -> Self {
        Self {
            coords,
            hole_indices,
        }
    }

    pub fn raw_coords(&self) -> &[T] {
        self.coords.as_ref()
    }

    /// A sequence of indices from which each hole starts
    pub fn hole_indices(&self) -> &[u32] {
        self.hole_indices.as_ref()
    }

    /// Returns the exterior ring of the polygon.
    pub fn exterior(&self) -> LineString<T> {
        LineString::from_raw(if self.hole_indices.is_empty() {
            self.coords[..].into()
        } else {
            self.coords[..self.hole_indices[0] as usize].into()
        })
    }

    /// Returns an iterator over the interior rings of the polygon.
    pub fn interiors(&self) -> Iter<T> {
        Iter { poly: self, pos: 1 }
    }

    /// Returns an iterator over the exterior and interior rings of the polygon.
    pub fn rings(&self) -> Iter<T> {
        Iter { poly: self, pos: 0 }
    }

    /// Remove all rings from the polygon.
    pub fn clear(&mut self) {
        self.coords.to_mut().clear();
        self.hole_indices.to_mut().clear();
    }

    /// Adds an exterior or interior ring to the polygon.
    pub fn add_ring<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        if !self.coords.is_empty() {
            self.hole_indices.to_mut().push((self.coords.len()) as u32);
        }
        let head = self.coords.len();
        self.coords.to_mut().extend(iter);

        // remove closing point if exists
        let tail = self.coords.len();
        if tail > head + 2 && self.coords[head..head + 1] == self.coords[tail - 1..] {
            self.coords.to_mut().truncate(tail - 1);
        }
    }

    /// Create a new Polygon by applying the given transformation to all coordinates.
    pub fn transform<T2: Coord>(self, f: impl Fn(&T) -> T2) -> Polygon<'a, T2> {
        Polygon {
            coords: self.coords.iter().map(f).collect(),
            hole_indices: self.hole_indices.clone(),
        }
    }

    /// Applies the given transformation to all coordinates in the Polygon.
    pub fn transform_inplace(&mut self, mut f: impl FnMut(&T) -> T) {
        self.coords.to_mut().iter_mut().for_each(|c| {
            *c = f(c);
        });
    }
}

// 2-dimensional only
impl<'a, T: Coord2d> Polygon<'a, T> {
    pub fn area(&self) -> f64 {
        let mut area = 0.0;
        area += self.exterior().ring_area();
        for interior in self.interiors() {
            area -= interior.ring_area();
        }
        area
    }
}

pub struct Iter<'a, T: Coord> {
    poly: &'a Polygon<'a, T>,
    pos: usize,
}

impl<'a, T: Coord> Iterator for Iter<'a, T> {
    type Item = LineString<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.poly.hole_indices.len() + 1 {
            let start = if self.pos == 0 {
                0
            } else {
                self.poly.hole_indices[self.pos - 1] as usize
            };

            let end = if self.pos == self.poly.hole_indices.len() {
                self.poly.coords.len()
            } else {
                self.poly.hole_indices[self.pos] as usize
            };

            let line = LineString::from_raw(self.poly.coords[start..end].into());
            self.pos += 1;
            Some(line)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_empty() {
        let polygon = Polygon2::<f64>::new();
        assert_eq!(polygon.exterior().len(), 0);
        assert_eq!(polygon.interiors().count(), 0);
    }

    #[test]
    fn test_polygon_no_interior() {
        let coords: Vec<_> = (0..=10).map(|i| [i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![];
        let polygon: Polygon2<f64> =
            Polygon2::from_raw_unchecked(coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 11);
        assert_eq!(polygon.interiors().count(), 0);
    }

    #[test]
    fn test_polygon_one_interior() {
        let coords: Vec<_> = (0..=10).map(|i| [i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![4];
        let polygon: Polygon2<f64> =
            Polygon2::from_raw_unchecked(coords.into(), hole_indices.into());

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
        let coords: Vec<_> = (0..=10).map(|i| [i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![4, 7];
        let polygon: Polygon2<f64> =
            Polygon2::from_raw_unchecked(coords.into(), hole_indices.into());

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
        let coords: Vec<_> = (0..=10).map(|i| [i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![4, 7];
        let mut polygon: Polygon2<f64> =
            Polygon2::from_raw_unchecked(coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 4);
        assert_eq!(polygon.interiors().count(), 2);

        polygon.clear();
        assert_eq!(polygon.exterior().len(), 0);
        assert_eq!(polygon.interiors().count(), 0);
    }

    #[test]
    fn test_polygon_add_ring() {
        let mut polygon = Polygon2::new();
        assert_eq!(polygon.exterior().len(), 0);
        assert_eq!(polygon.interiors().count(), 0);
        polygon.add_ring([[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(polygon.exterior().len(), 3);
        assert_eq!(polygon.interiors().count(), 0);
        polygon.add_ring([[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(polygon.exterior().len(), 3);
        assert_eq!(polygon.interiors().count(), 1);

        let mut polygon = Polygon2::new();
        polygon.add_ring([[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.0, 0.0]]);
        assert_eq!(polygon.exterior().len(), 3);
        assert_eq!(polygon.interiors().count(), 0);
    }

    #[test]
    fn test_transform() {
        {
            let mut poly: Polygon2 = Polygon2::new();
            poly.add_ring([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            let new_poly = poly.transform(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                new_poly.exterior().raw_coords(),
                [[2., 1.], [7., 1.], [7., 6.], [2., 6.]]
            );
        }

        {
            let mut poly = Polygon2::new();
            poly.add_ring([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            poly.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                poly.exterior().raw_coords(),
                [[2., 1.], [7., 1.], [7., 6.], [2., 6.]]
            );
        }
    }

    /// Currently, it does not check whether the exterior is valid (have at least three vertices) or not
    #[test]
    fn test_polygon_invalid_exterior() {
        let coords: Vec<_> = (0..1).map(|i| [i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![];
        let polygon: Polygon2<f64> = Polygon2::from_raw(coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 1); // only one vertex
        assert_eq!(polygon.interiors().count(), 0);
    }

    /// Currently, it does not check whether the interior is valid (have at least three vertices) or not
    /// (It could be a "Steiner point", as we see in earcut)
    #[test]
    fn test_polygon_invalid_interior() {
        let all_coords: Vec<_> = (0..=10).map(|i| [i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![10]; // only one vertex
        let polygon: Polygon2<f64> = Polygon2::from_raw(all_coords.into(), hole_indices.into());

        assert_eq!(polygon.exterior().len(), 10);
        assert_eq!(polygon.interiors().count(), 1);
        for (i, interior) in polygon.interiors().enumerate() {
            match i {
                0 => assert_eq!(interior.len(), 1), // only one vertex
                _ => unreachable!(),
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_polygon_invalid_hole_indices_1() {
        let coords: Vec<_> = (0..=2).map(|i| [i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![3]; // out of `all_coords` range
        let _polygon: Polygon2<f64> = Polygon2::from_raw(coords.into(), hole_indices.into());
    }

    #[test]
    #[should_panic]
    fn test_polygon_invalid_hole_indices_2() {
        let coords: Vec<_> = (0..15).map(|i| [i as f64, i as f64]).collect();
        let hole_indices: Vec<u32> = vec![6, 3]; // not monotonically increasing
        let _polygon: Polygon2<f64> = Polygon2::from_raw(coords.into(), hole_indices.into());
    }

    #[test]
    fn test_area() {
        let mut polygon = Polygon2::new();
        assert_eq!(polygon.area(), 0.0);
        polygon.add_ring([[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0]]);
        assert_eq!(polygon.area(), 9.0);
        polygon.add_ring([[1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]]);
        assert_eq!(polygon.area(), 8.0);

        // winding order should not matter
        let mut polygon = Polygon2::new();
        polygon.add_ring([[0.0, 0.0], [0.0, 3.0], [3.0, 3.0], [3.0, 0.0]]);
        assert_eq!(polygon.area(), 9.0);
        polygon.add_ring([[1.0, 1.0], [2.0, 1.0], [2.0, 2.0], [1.0, 2.0]]);
        assert_eq!(polygon.area(), 8.0);
    }
}
