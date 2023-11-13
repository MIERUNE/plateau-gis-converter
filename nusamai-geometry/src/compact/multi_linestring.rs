use std::borrow::Cow;

use super::linestring::CompactLineString;

use num_traits::Float;

/// Computer-friendly MultiString
#[derive(Debug, Clone, Default)]
pub struct CompactMultiLineString<'a, const D: usize, T: Float = f64> {
    /// すべての LineString の座標データを連結したもの
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    all_coords: Cow<'a, [T]>,

    /// all_coords における各 LineString の開始頂点番号
    ///
    /// ただし1つ目の LineString の開始位置は必ず `0` なので省略する。
    ///
    /// e.g. `[5, 12, 23]`
    coords_spans: Cow<'a, [u32]>,
}

pub type CompactMultiLineString2<'a, T = f64> = CompactMultiLineString<'a, 2, T>;
pub type CompactMultiLineString3<'a, T = f64> = CompactMultiLineString<'a, 3, T>;

impl<'a, const D: usize, T: Float> CompactMultiLineString<'a, D, T> {
    pub fn new(all_coords: Cow<'a, [T]>, coords_spans: Cow<'a, [u32]>) -> Self {
        Self {
            all_coords,
            coords_spans,
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, D, T> {
        Iter {
            all_coords: &self.all_coords,
            coords_spans: &self.coords_spans,
            start: 0,
            index_pos: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        if self.all_coords.is_empty() {
            0
        } else {
            self.coords_spans.len() + 1
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.all_coords.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.all_coords.to_mut().clear();
        self.coords_spans.to_mut().clear();
    }
}

impl<'a, const D: usize, T: Float> IntoIterator for &'a CompactMultiLineString<'_, D, T> {
    type Item = CompactLineString<'a, D, T>;
    type IntoIter = Iter<'a, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const D: usize, T: Float> {
    all_coords: &'a [T],
    coords_spans: &'a [u32],
    start: usize,
    index_pos: usize,
}

impl<'a, const D: usize, T: Float> Iterator for Iter<'a, D, T> {
    type Item = CompactLineString<'a, D, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_pos < self.coords_spans.len() {
            let end = self.coords_spans[self.index_pos] as usize * D;
            self.index_pos += 1;
            let coords = &self.all_coords[self.start..end];
            self.start = end;
            Some(CompactLineString::new(coords.into()))
        } else if self.start < self.all_coords.len() {
            let coords = &self.all_coords[self.start..];
            self.start = self.all_coords.len();
            Some(CompactLineString::new(coords.into()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mline_basic() {
        let mline = CompactMultiLineString2::new(
            (0..14).map(|v| v as f64).collect::<Vec<f64>>().into(),
            vec![3, 5].into(),
        );
        assert_eq!(mline.len(), 3);
        for (i, line) in mline.iter().enumerate() {
            match i {
                0 => assert_eq!(line.coords(), &[0., 1., 2., 3., 4., 5.]),
                1 => assert_eq!(line.coords(), &[6., 7., 8., 9.]),
                2 => assert_eq!(line.coords(), &[10., 11., 12., 13.]),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_mline_one_linestring() {
        let mline = CompactMultiLineString2::new(Cow::Borrowed(&[1., 2., 3.]), Cow::Borrowed(&[]));
        assert_eq!(mline.len(), 1);
        assert!(!mline.is_empty());
        for (i, _line) in mline.iter().enumerate() {
            match i {
                0 => assert_eq!(_line.coords(), &[1., 2., 3.]),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_mline_empty() {
        let mline: CompactMultiLineString2 = Default::default();
        assert_eq!(mline.len(), 0);
        assert!(mline.is_empty());
        for _line in &mline {
            unreachable!("should not be called");
        }
    }
}
