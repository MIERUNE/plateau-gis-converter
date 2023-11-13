use std::borrow::Cow;

use num_traits::Float;

/// Computer-friendly LineString
#[derive(Debug, Clone, Default)]
pub struct CompactLineString<'a, const D: usize, T: Float = f64> {
    /// 座標データ
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    coords: Cow<'a, [T]>,
}

pub type CompactLineString2<'a, T = f64> = CompactLineString<'a, 2, T>;
pub type CompactLineString3<'a, T = f64> = CompactLineString<'a, 3, T>;

impl<'a, const D: usize, T: Float> CompactLineString<'a, D, T> {
    pub fn new(coords: Cow<'a, [T]>) -> Self {
        Self { coords }
    }

    #[inline]
    pub fn coords(&self) -> &[T] {
        self.as_ref()
    }

    /// この LineString の座標列のイテレータを得る
    #[inline]
    pub fn iter(&self) -> Iter<'_, D, T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            close: false,
        }
    }

    /// 始点と終点を閉じた座標列のイテレータを得る
    #[inline]
    pub fn iter_closed(&self) -> Iter<'_, D, T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            close: true,
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

impl<const D: usize, T: Float> AsRef<[T]> for CompactLineString<'_, D, T> {
    fn as_ref(&self) -> &[T] {
        self.coords.as_ref()
    }
}

impl<'a, const D: usize, T: Float> IntoIterator for &'a CompactLineString<'_, D, T> {
    type Item = &'a [T];
    type IntoIter = Iter<'a, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const D: usize, T: Float> {
    slice: &'a [T],
    pos: usize,
    close: bool,
}

impl<'a, const D: usize, T: Float> Iterator for Iter<'a, D, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += D;
        if self.pos <= self.slice.len() {
            Some(&self.slice[self.pos - D..self.pos])
        } else if self.pos == self.slice.len() + D && self.close {
            Some(&self.slice[..D])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_basic() {
        let line: CompactLineString2 =
            CompactLineString2::new((0..8).map(|v| v as f64).collect::<Vec<f64>>().into());
        assert_eq!(line.len(), 4);
        assert!(!line.is_empty());
        for (i, coord) in line.iter().enumerate() {
            match i {
                0 => assert_eq!(coord, &[0., 1.]),
                1 => assert_eq!(coord, &[2., 3.]),
                2 => assert_eq!(coord, &[4., 5.]),
                3 => assert_eq!(coord, &[6., 7.]),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_line_empty() {
        let line: CompactLineString2 = Default::default();
        assert_eq!(line.len(), 0);
        assert!(line.is_empty());
        for _coord in &line {
            unreachable!("should not be called");
        }
    }

    #[test]
    fn test_line_close() {
        let line: CompactLineString2 =
            CompactLineString2::new((0..6).map(|v| v as f64).collect::<Vec<f64>>().into());
        assert_eq!(line.len(), 3);
        assert!(!line.is_empty());
        for (i, coord) in line.iter_closed().enumerate() {
            match i {
                0 => assert_eq!(coord, &[0., 1.]),
                1 => assert_eq!(coord, &[2., 3.]),
                2 => assert_eq!(coord, &[4., 5.]),
                3 => assert_eq!(coord, &[0., 1.]),
                _ => unreachable!(),
            }
        }
    }
}
