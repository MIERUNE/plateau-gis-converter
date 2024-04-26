use crate::{Coord, Coord2d};
use std::{borrow::Cow, hash::Hash};

/// Computer-friendly LineString
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct LineString<'a, T: Coord> {
    /// Coordinates of all points
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    coords: Cow<'a, [T]>,
}

pub type LineString2<'a, C = f64> = LineString<'a, [C; 2]>;
pub type LineString3<'a, C = f64> = LineString<'a, [C; 3]>;

impl PartialEq for LineString2<'_, f64> {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl PartialEq for LineString3<'_, f64> {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl Hash for LineString2<'_, f64> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coords
            .iter()
            .for_each(|c| c.iter().for_each(|a| a.to_bits().hash(state)));
    }
}

impl Hash for LineString3<'_, f64> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coords
            .iter()
            .for_each(|c| c.iter().for_each(|a| a.to_bits().hash(state)));
    }
}

impl<'a, T: Coord> LineString<'a, T> {
    /// Creates an empty LineString.
    pub fn new() -> Self {
        Self {
            coords: Cow::Borrowed(&[]),
        }
    }

    pub fn from_raw(coords: Cow<'a, [T]>) -> Self {
        Self { coords }
    }

    pub fn raw_coords(&self) -> &[T] {
        self.as_ref()
    }

    /// Returns iterator over the all points in the LineString.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            close: false,
        }
    }

    /// Returns iterator over the all points with the start point repeated.
    pub fn iter_closed(&self) -> Iter<T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            close: true,
        }
    }

    /// Returns the number of points in the LineString.
    pub fn len(&self) -> usize {
        self.coords.len()
    }

    /// Returns `true` if the LineString is empty.
    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    /// Appends a point to the LineString.
    pub fn push(&mut self, coord: T) {
        self.coords.to_mut().push(coord);
    }

    /// Removes all points from the LineString.
    pub fn clear(&mut self) {
        self.coords.to_mut().clear();
    }

    /// Create a new LineString by applying the given transformation to all coordinates.
    pub fn transform<T2: Coord>(&self, f: impl Fn(&T) -> T2) -> LineString<T2> {
        LineString {
            coords: self.coords.iter().map(f).collect(),
        }
    }

    /// Applies the given transformation to all coordinates in the LineString.
    pub fn transform_inplace(&mut self, mut f: impl FnMut(&T) -> T) {
        self.coords.to_mut().iter_mut().for_each(|c| {
            *c = f(c);
        });
    }

    /// Reverses the coordinates in the LineString.
    pub fn reverse_inplace(&mut self) {
        let len = self.coords.len();
        if len > 0 {
            let data = self.coords.to_mut();
            for i in 0..data.len() / 2 {
                data.swap(i, len - (i + 1));
            }
        }
    }

    /// Reverses the winding order of the coordinates in the ring, preserving the first coordinate.
    pub fn reverse_ring_inplace(&mut self) {
        let len = self.coords.len();
        if len > 1 {
            let data = self.coords.to_mut();
            for i in 1..(data.len() + 1) / 2 {
                data.swap(i, len - i);
            }
        }
    }
}

// 2-dimensional only
impl<'a, T: Coord2d> LineString<'a, T> {
    /// Returns true if the ring is counter-clockwise.
    pub fn is_ccw(&self) -> bool {
        self.signed_ring_area() > 0.0
    }

    /// Returns true if the ring is clockwise.
    pub fn is_cw(&self) -> bool {
        self.signed_ring_area() < 0.0
    }

    /// Calculates the area of this LineString as a ring.
    pub fn ring_area(&self) -> f64 {
        self.signed_ring_area().abs()
    }

    /// Calculates the signed area of this LineString as a ring.
    pub fn signed_ring_area(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        let mut area = 0.0;
        let mut ring_iter = self.iter_closed();
        let mut prev = ring_iter.next().unwrap().xy();
        // shoelace formula
        for coord in ring_iter {
            let xy = coord.xy();
            area += (prev.0 * xy.1) - (prev.1 * xy.0);
            prev = xy;
        }
        area / 2.0
    }
}

impl<T: Coord> AsRef<[T]> for LineString<'_, T> {
    fn as_ref(&self) -> &[T] {
        self.coords.as_ref()
    }
}

impl<'a, T: Coord> IntoIterator for &'a LineString<'_, T> {
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T: Coord> {
    slice: &'a [T],
    pos: usize,
    close: bool,
}

impl<'a, T: Coord> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let v = if self.pos < self.slice.len() {
            Some(self.slice[self.pos].clone())
        } else if self.close && !self.slice.is_empty() && self.pos == self.slice.len() {
            Some(self.slice[0].clone())
        } else {
            None
        };
        self.pos += 1;
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_basic() {
        let mut line: LineString2 = LineString2::from_raw(
            (0..4)
                .map(|v| [v as f64, v as f64])
                .collect::<Vec<[f64; 2]>>()
                .into(),
        );
        assert_eq!(line.len(), 4);
        assert!(!line.is_empty());
        for (i, coord) in line.iter().enumerate() {
            match i {
                0 => assert_eq!(coord, [0., 0.]),
                1 => assert_eq!(coord, [1., 1.]),
                2 => assert_eq!(coord, [2., 2.]),
                3 => assert_eq!(coord, [3., 3.]),
                _ => unreachable!(),
            }
        }

        line.clear();
        assert_eq!(line.len(), 0);
        assert!(line.is_empty());

        line.push([0., 1.]);
        assert_eq!(line.len(), 1);
        for _c in &line {}
    }

    #[test]
    fn test_line_empty() {
        let line: LineString2 = LineString2::new();
        assert_eq!(line.len(), 0);
        assert!(line.is_empty());
        assert_eq!(line.iter().count(), 0);
    }

    #[test]
    fn test_line_empty_iter_closed() {
        let line: LineString2 = LineString2::new();
        assert_eq!(line.iter_closed().count(), 0);
    }

    #[test]
    fn test_line_close() {
        let line: LineString2 = LineString2::from_raw(
            (0..3)
                .map(|v| [v as f64, v as f64])
                .collect::<Vec<[f64; 2]>>()
                .into(),
        );
        assert_eq!(line.len(), 3);
        assert!(!line.is_empty());
        for (i, coord) in line.iter_closed().enumerate() {
            match i {
                0 => assert_eq!(coord, [0., 0.]),
                1 => assert_eq!(coord, [1., 1.]),
                2 => assert_eq!(coord, [2., 2.]),
                3 => assert_eq!(coord, [0., 0.]),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_transform() {
        {
            let line = LineString2::from_raw(vec![[0., 0.], [5., 0.], [5., 5.], [0., 5.]].into());
            let new_line = line.transform(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                new_line.raw_coords(),
                [[2., 1.], [7., 1.], [7., 6.], [2., 6.]]
            );
        }

        {
            let mut line =
                LineString2::from_raw([[0., 0.], [5., 0.], [5., 5.], [0., 5.]][..].into());
            line.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(line.raw_coords(), [[2., 1.], [7., 1.], [7., 6.], [2., 6.]]);
        }
    }

    #[test]
    fn test_winding_order() {
        let line =
            LineString2::from_raw(vec![[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0]].into());
        assert!(line.is_ccw());
        assert!(!line.is_cw());

        let line =
            LineString2::from_raw(vec![[0.0, 0.0], [0.0, 3.0], [3.0, 3.0], [3.0, 0.0]].into());
        assert!(!line.is_ccw());
        assert!(line.is_cw());

        let line = LineString2::from_raw(vec![[0.0, 0.0], [0.0, 0.0]].into());
        assert!(!line.is_ccw());
        assert!(!line.is_cw());
    }

    #[test]
    fn test_reverse() {
        let mut line = LineString2::from_raw(
            vec![
                [0.0, 0.0],
                [3.0, 0.0],
                [3.0, 1.0],
                [3.0, 3.0],
                [1.0, 3.0],
                [0.0, 3.0],
            ]
            .into(),
        );
        line.reverse_inplace();
        assert_eq!(
            line.raw_coords(),
            vec![
                [0.0, 3.0],
                [1.0, 3.0],
                [3.0, 3.0],
                [3.0, 1.0],
                [3.0, 0.0],
                [0.0, 0.0]
            ]
        );

        let mut line = LineString2::from_raw(
            vec![[0.0, 0.0], [3.0, 0.0], [6.0, 0.0], [6.0, 3.0], [3.0, 3.0]].into(),
        );
        line.reverse_inplace();
        assert_eq!(
            line.raw_coords(),
            vec![[3.0, 3.0], [6.0, 3.0], [6.0, 0.0], [3.0, 0.0], [0.0, 0.0]]
        );
    }

    #[test]
    fn test_ring_reverse() {
        let mut line = LineString2::from_raw(
            vec![
                [0.0, 0.0],
                [3.0, 0.0],
                [3.0, 1.0],
                [3.0, 3.0],
                [1.0, 3.0],
                [0.0, 3.0],
            ]
            .into(),
        );
        line.reverse_ring_inplace();
        assert_eq!(
            line.raw_coords(),
            vec![
                [0.0, 0.0],
                [0.0, 3.0],
                [1.0, 3.0],
                [3.0, 3.0],
                [3.0, 1.0],
                [3.0, 0.0]
            ]
        );

        let mut line = LineString2::from_raw(
            vec![[0.0, 0.0], [3.0, 0.0], [6.0, 0.0], [6.0, 3.0], [3.0, 3.0]].into(),
        );
        line.reverse_ring_inplace();
        assert_eq!(
            line.raw_coords(),
            vec![[0.0, 0.0], [3.0, 3.0], [6.0, 3.0], [6.0, 0.0], [3.0, 0.0]]
        );
    }
}
