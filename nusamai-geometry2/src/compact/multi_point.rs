use crate::Coord;
use std::borrow::Cow;
use std::ops::Range;

/// Computer-friendly MultiPoint
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiPoint<'a, T: Coord> {
    /// Coordinates of all points
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    coords: Cow<'a, [T]>,
}

pub type MultiPoint2<'a, C = f64> = MultiPoint<'a, [C; 2]>;
pub type MultiPoint3<'a, C = f64> = MultiPoint<'a, [C; 3]>;

impl<'a, T: Coord> MultiPoint<'a, T> {
    /// Creates an empty MultiPoint.
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

    /// Returns iterator over the all points.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            end: self.coords.len(),
        }
    }

    /// Returns iterator over the points in the given range.
    pub fn iter_range(&self, range: Range<usize>) -> Iter<T> {
        Iter {
            slice: &self.coords,
            pos: range.start,
            end: range.end,
        }
    }

    /// Returns the point at the given index.
    pub fn get(&self, index: usize) -> Option<T> {
        self.coords.get(index).cloned()
    }

    /// Returns the number of points in the LineString.
    pub fn len(&self) -> usize {
        self.coords.len()
    }

    /// Returns `true` if the MultiPoint is empty.
    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    /// Appends a coordinate to the MultiPoint.
    pub fn push(&mut self, coord: T) {
        self.coords.to_mut().push(coord);
    }

    /// Removes all points from the LineString.
    pub fn clear(&mut self) {
        self.coords.to_mut().clear();
    }

    /// Create a new MultiPoint by applying the given transformation to all coordinates.
    pub fn transform<T2: Coord>(&self, f: impl Fn(&T) -> T2) -> MultiPoint<T2> {
        MultiPoint {
            coords: self.coords.iter().map(f).collect(),
        }
    }

    /// Applies the given transformation to all coordinates in the MultiPoint.
    pub fn transform_inplace(&mut self, mut f: impl FnMut(&T) -> T) {
        self.coords.to_mut().iter_mut().for_each(|c| {
            *c = f(c);
        });
    }
}

impl<T: Coord> AsRef<[T]> for MultiPoint<'_, T> {
    fn as_ref(&self) -> &[T] {
        self.coords.as_ref()
    }
}

impl<'a, T: Coord> IntoIterator for &'a MultiPoint<'_, T> {
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T: Coord> {
    slice: &'a [T],
    pos: usize,
    end: usize,
}

impl<'a, T: Coord> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let v = if self.pos < self.end {
            Some(self.slice[self.pos].clone())
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
    fn test_mpoints_basic() {
        let mut mpoints = MultiPoint2::new();
        assert!(mpoints.is_empty());
        assert_eq!(mpoints.len(), 0);

        mpoints.push([0.0, 0.0]);
        assert!(!mpoints.is_empty());
        assert_eq!(mpoints.len(), 1);

        mpoints.push([1.0, 1.0]);
        mpoints.push([2.0, 2.0]);
        mpoints.push([3.0, 3.0]);
        assert_eq!(mpoints.get(0), Some([0.0, 0.0]));
        assert_eq!(mpoints.get(1), Some([1.0, 1.0]));
        assert_eq!(mpoints.get(2), Some([2.0, 2.0]));
        assert_eq!(mpoints.get(3), Some([3.0, 3.0]));
        assert_eq!(mpoints.len(), 4);
        assert_eq!(mpoints.iter().count(), 4);
        assert_eq!((&mpoints).into_iter().count(), 4);

        for (i, point) in mpoints.iter().enumerate() {
            match i {
                0 => assert_eq!(point, [0.0, 0.0]),
                1 => assert_eq!(point, [1.0, 1.0]),
                2 => assert_eq!(point, [2.0, 2.0]),
                3 => assert_eq!(point, [3.0, 3.0]),
                _ => unreachable!(),
            }
        }

        for (i, point) in mpoints.iter_range(1..3).enumerate() {
            match i {
                0 => assert_eq!(point, [1.0, 1.0]),
                1 => assert_eq!(point, [2.0, 2.0]),
                _ => unreachable!(),
            }
        }

        mpoints.clear();
        assert!(mpoints.is_empty());
        assert_eq!(mpoints.len(), 0);
        assert_eq!(mpoints.iter().count(), 0);
    }

    #[test]
    fn test_mpoints_from_raw() {
        let mpoints = MultiPoint2::from_raw([[1.2, 2.1], [3.4, 4.3]][..].into());
        assert_eq!(mpoints.as_ref(), [[1.2, 2.1], [3.4, 4.3]]);
        assert_eq!(mpoints.raw_coords(), [[1.2, 2.1], [3.4, 4.3]]);
    }

    #[test]
    fn test_transform() {
        {
            let mpoints =
                MultiPoint2::from_raw([[0., 0.], [5., 0.], [5., 5.], [0., 5.]][..].into());
            let new_mpoints = mpoints.transform(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                new_mpoints.raw_coords(),
                [[2., 1.], [7., 1.], [7., 6.], [2., 6.]]
            );
        }

        {
            let mut mpoints =
                MultiPoint2::from_raw([[0., 0.], [5., 0.], [5., 5.], [0., 5.]][..].into());
            mpoints.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                mpoints.raw_coords(),
                [[2., 1.], [7., 1.], [7., 6.], [2., 6.]]
            );
        }
    }
}
