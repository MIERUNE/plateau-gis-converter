use std::borrow::Cow;

use super::CoordNum;

/// Computer-friendly MultiPoint
#[derive(Debug, Clone, Default)]
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

    pub fn iter(&self) -> Iter<'_, D, T> {
        Iter {
            slice: &self.coords,
            pos: 0,
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
}

impl<const D: usize, T: CoordNum> AsRef<[T]> for MultiPoint<'_, D, T> {
    fn as_ref(&self) -> &[T] {
        self.coords.as_ref()
    }
}

impl<'a, const D: usize, T: CoordNum> IntoIterator for &'a MultiPoint<'_, D, T> {
    type Item = &'a [T];
    type IntoIter = Iter<'a, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const D: usize, T: CoordNum> {
    slice: &'a [T],
    pos: usize,
}

impl<'a, const D: usize, T: CoordNum> Iterator for Iter<'a, D, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += D;
        if self.pos <= self.slice.len() {
            Some(&self.slice[self.pos - D..self.pos])
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
        assert_eq!(mpoints.get(0), &[0.0, 0.0]);
        assert_eq!(mpoints.get(1), &[1.0, 1.0]);
        assert_eq!(mpoints.len(), 2);
        assert_eq!(mpoints.iter().count(), 2);
        assert_eq!((&mpoints).into_iter().count(), 2);

        for (i, point) in mpoints.iter().enumerate() {
            match i {
                0 => assert_eq!(point, &[0.0, 0.0]),
                1 => assert_eq!(point, &[1.0, 1.0]),
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
}
