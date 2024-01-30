use std::{borrow::Cow, ops::Range};

use super::CoordNum;

/// Computer-friendly MultiPoint
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiPoint<'a, const D: usize, T: CoordNum = f64> {
    /// すべての Point の座標データを連結したもの
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    coords: Cow<'a, [T]>,
}

pub type MultiPoint2<'a, T = f64> = MultiPoint<'a, 2, T>;
pub type MultiPoint3<'a, T = f64> = MultiPoint<'a, 3, T>;

impl<'a, const D: usize, T: CoordNum> MultiPoint<'a, D, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_raw(coords: Cow<'a, [T]>) -> Self {
        if coords.len() % D != 0 {
            panic!("coords.len() must be a multiple of D")
        }
        Self { coords }
    }

    pub fn coords(&self) -> &[T] {
        self.as_ref()
    }

    pub fn iter(&self) -> Iter<D, T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            end: self.coords.len(),
        }
    }

    pub fn iter_range(&self, range: Range<usize>) -> Iter<D, T> {
        Iter {
            slice: &self.coords,
            pos: range.start * D,
            end: range.end * D,
        }
    }

    pub fn get(&self, index: usize) -> &[T] {
        &self.coords[index * D..(index + 1) * D]
    }

    pub fn len(&self) -> usize {
        self.coords.len() / D
    }

    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    pub fn push(&mut self, coord: &[T; D]) {
        self.coords.to_mut().extend(coord);
    }

    pub fn clear(&mut self) {
        self.coords.to_mut().clear();
    }

    /// Create a new MultiPoint by applying the given transformation to all coordinates.
    pub fn transform<const D2: usize, T2: CoordNum>(
        &self,
        f: impl Fn(&[T; D]) -> [T2; D2],
    ) -> MultiPoint<D2, T2> {
        MultiPoint {
            coords: self
                .coords
                .chunks_exact(D)
                .flat_map(|v| f(&v.try_into().unwrap()))
                .collect(),
        }
    }

    /// Applies the given transformation to all coordinates in the MultiPoint.
    pub fn transform_inplace(&mut self, f: impl Fn(&[T; D]) -> [T; D]) {
        self.coords.to_mut().chunks_exact_mut(D).for_each(|c| {
            let transformed = f(&c.try_into().unwrap());
            c.copy_from_slice(&transformed);
        });
    }
}

impl<const D: usize, T: CoordNum> AsRef<[T]> for MultiPoint<'_, D, T> {
    fn as_ref(&self) -> &[T] {
        self.coords.as_ref()
    }
}

impl<'a, const D: usize, T: CoordNum> IntoIterator for &'a MultiPoint<'_, D, T> {
    type Item = [T; D];
    type IntoIter = Iter<'a, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const D: usize, T: CoordNum> {
    slice: &'a [T],
    pos: usize,
    end: usize,
}

impl<'a, const D: usize, T: CoordNum> Iterator for Iter<'a, D, T> {
    type Item = [T; D];

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += D;
        if self.pos <= self.end {
            Some(self.slice[self.pos - D..self.pos].try_into().unwrap())
        } else {
            None
        }
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

        mpoints.push(&[0.0, 0.0]);
        assert!(!mpoints.is_empty());
        assert_eq!(mpoints.len(), 1);

        mpoints.push(&[1.0, 1.0]);
        mpoints.push(&[2.0, 2.0]);
        mpoints.push(&[3.0, 3.0]);
        assert_eq!(mpoints.get(0), &[0.0, 0.0]);
        assert_eq!(mpoints.get(1), &[1.0, 1.0]);
        assert_eq!(mpoints.get(2), &[2.0, 2.0]);
        assert_eq!(mpoints.get(3), &[3.0, 3.0]);
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
        let mpoints = MultiPoint2::from_raw([1.2, 2.1, 3.4, 4.3][..].into());
        assert_eq!(mpoints.as_ref(), [1.2, 2.1, 3.4, 4.3]);
        assert_eq!(mpoints.coords(), [1.2, 2.1, 3.4, 4.3]);
    }

    #[test]
    fn test_transform() {
        {
            let mpoints = MultiPoint2::from_raw([0., 0., 5., 0., 5., 5., 0., 5.][..].into());
            let new_mpoints = mpoints.transform(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(new_mpoints.coords(), [2., 1., 7., 1., 7., 6., 2., 6.]);
        }

        {
            let mut mpoints = MultiPoint2::from_raw([0., 0., 5., 0., 5., 5., 0., 5.][..].into());
            mpoints.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(mpoints.coords(), [2., 1., 7., 1., 7., 6., 2., 6.]);
        }
    }
}
