use std::borrow::Cow;

use num_traits::Float;

/// Computer-friendly MultiPoint
#[derive(Debug, Clone, Default)]
pub struct CompactMultiPoint<'a, const D: usize, T: Float = f64> {
    /// すべての Point の座標データを連結したもの
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    coords: Cow<'a, [T]>,
}

pub type CompactMultiPoint2<'a, T = f64> = CompactMultiPoint<'a, 2, T>;
pub type CompactMultiPoint3<'a, T = f64> = CompactMultiPoint<'a, 3, T>;

impl<'a, const D: usize, T: Float> CompactMultiPoint<'a, D, T> {
    pub fn new(coords: Cow<'a, [T]>) -> Self {
        Self { coords }
    }

    #[inline]
    pub fn coords(&self) -> &[T] {
        self.as_ref()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, D, T> {
        Iter {
            slice: &self.coords,
            pos: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.coords.len() / D
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    #[inline]
    pub fn push(&mut self, coord: &[T; D]) {
        self.coords.to_mut().extend(coord);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.coords.to_mut().clear();
    }
}

impl<const D: usize, T: Float> AsRef<[T]> for CompactMultiPoint<'_, D, T> {
    fn as_ref(&self) -> &[T] {
        self.coords.as_ref()
    }
}

pub struct Iter<'a, const D: usize, T: Float> {
    slice: &'a [T],
    pos: usize,
}

impl<'a, const D: usize, T: Float> Iterator for Iter<'a, D, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += D;
        if self.pos < self.slice.len() {
            Some(&self.slice[self.pos - D..self.pos])
        } else {
            None
        }
    }
}
