use std::borrow::Cow;

use super::linestring::LineString;
use super::CoordNum;

/// Computer-friendly MultiString
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiLineString<'a, const D: usize, T: CoordNum = f64> {
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

pub type MultiLineString2<'a, T = f64> = MultiLineString<'a, 2, T>;
pub type MultiLineString3<'a, T = f64> = MultiLineString<'a, 3, T>;

impl<'a, const D: usize, T: CoordNum> MultiLineString<'a, D, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_raw(all_coords: Cow<'a, [T]>, coords_spans: Cow<'a, [u32]>) -> Self {
        if all_coords.len() % D != 0 {
            panic!("all_coords.len() must be a multiple of D")
        }
        // Check if all span values are within range and monotonically increasing
        if let Some(&last) = coords_spans.last() {
            let len = (all_coords.len() / D) as u32;
            if last >= len || coords_spans.windows(2).any(|a| a[0] >= a[1]) {
                panic!("invalid coords_spans")
            }
        }
        Self {
            all_coords,
            coords_spans,
        }
    }

    pub fn from_raw_unchecked(all_coords: Cow<'a, [T]>, coords_spans: Cow<'a, [u32]>) -> Self {
        Self {
            all_coords,
            coords_spans,
        }
    }

    pub fn iter(&self) -> Iter<D, T> {
        Iter {
            all_coords: &self.all_coords,
            coords_spans: &self.coords_spans,
            start: 0,
            index_pos: 0,
        }
    }

    pub fn len(&self) -> usize {
        if self.all_coords.is_empty() {
            0
        } else {
            self.coords_spans.len() + 1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.all_coords.is_empty()
    }

    pub fn clear(&mut self) {
        self.all_coords.to_mut().clear();
        self.coords_spans.to_mut().clear();
    }

    pub fn add_linestring<I: IntoIterator<Item = [T; D]>>(&mut self, iter: I) {
        if !self.all_coords.is_empty() {
            self.coords_spans
                .to_mut()
                .push((self.all_coords.len() / D) as u32);
        }
        self.all_coords.to_mut().extend(iter.into_iter().flatten());
    }

    /// Create a new MultiLineString by applying the given transformation to all coordinates.
    pub fn transform(&self, f: impl Fn(&[T; D]) -> [T; D]) -> Self {
        Self {
            all_coords: self
                .all_coords
                .chunks_exact(D)
                .flat_map(|v| f(&v.try_into().unwrap()))
                .collect(),
            ..self.clone()
        }
    }

    /// Applies the given transformation to all coordinates in the MultiLineString.
    pub fn transform_inplace(&mut self, f: impl Fn(&[T; D]) -> [T; D]) {
        self.all_coords.to_mut().chunks_exact_mut(D).for_each(|c| {
            let transformed = f(&c.try_into().unwrap());
            c.copy_from_slice(&transformed);
        });
    }
}

impl<'a, const D: usize, T: CoordNum> IntoIterator for &'a MultiLineString<'_, D, T> {
    type Item = LineString<'a, D, T>;
    type IntoIter = Iter<'a, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const D: usize, T: CoordNum> {
    all_coords: &'a [T],
    coords_spans: &'a [u32],
    start: usize,
    index_pos: usize,
}

impl<'a, const D: usize, T: CoordNum> Iterator for Iter<'a, D, T> {
    type Item = LineString<'a, D, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_pos < self.coords_spans.len() {
            let end = self.coords_spans[self.index_pos] as usize * D;
            self.index_pos += 1;
            let coords = &self.all_coords[self.start..end];
            self.start = end;
            Some(LineString::from_raw(coords.into()))
        } else if self.start < self.all_coords.len() {
            let coords = &self.all_coords[self.start..];
            self.start = self.all_coords.len();
            Some(LineString::from_raw(coords.into()))
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
        let mut mline = MultiLineString2::from_raw(
            (0..14).map(|v| v as f64).collect::<Vec<f64>>().into(),
            vec![3, 5].into(),
        );
        assert_eq!(mline.len(), 3);
        assert_eq!(mline.iter().count(), 3);
        for (i, line) in mline.iter().enumerate() {
            match i {
                0 => assert_eq!(line.coords(), &[0., 1., 2., 3., 4., 5.]),
                1 => assert_eq!(line.coords(), &[6., 7., 8., 9.]),
                2 => assert_eq!(line.coords(), &[10., 11., 12., 13.]),
                _ => unreachable!(),
            }
        }
        mline.clear();
        assert_eq!(mline.len(), 0);
    }

    #[test]
    fn test_mline_one_linestring() {
        let mline = MultiLineString2::from_raw_unchecked(
            Cow::Borrowed(&[1., 2., 3., 4.]),
            Cow::Borrowed(&[]),
        );
        assert_eq!(mline.len(), 1);
        assert!(!mline.is_empty());
        for (i, _line) in mline.iter().enumerate() {
            match i {
                0 => assert_eq!(_line.coords(), &[1., 2., 3., 4.]),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_mline_empty() {
        let mline = MultiLineString2::<f64>::new();
        assert_eq!(mline.len(), 0);
        assert!(mline.is_empty());
        assert_eq!(mline.iter().count(), 0);
    }

    #[test]
    fn test_mline_add_linestring() {
        let mut mline = MultiLineString2::new();
        assert_eq!(mline.len(), 0);

        mline.add_linestring(vec![[0., 0.], [1., 1.], [2., 2.]]);
        assert_eq!(mline.len(), 1);

        mline.add_linestring(vec![[3., 3.], [4., 4.], [5., 5.]]);
        assert_eq!(mline.len(), 2);

        mline.add_linestring(vec![[6., 6.], [7., 7.], [8., 8.], [9., 9.]]);
        assert_eq!(mline.len(), 3);

        for (i, line) in mline.iter().enumerate() {
            match i {
                0 => assert_eq!(line.coords(), &[0., 0., 1., 1., 2., 2.]),
                1 => assert_eq!(line.coords(), &[3., 3., 4., 4., 5., 5.]),
                2 => assert_eq!(line.coords(), &[6., 6., 7., 7., 8., 8., 9., 9.]),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_transform() {
        {
            let mut mlines: MultiLineString<2> = MultiLineString2::new();
            mlines.add_linestring([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            let new_mlines = mlines.transform(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                new_mlines.iter().next().unwrap().coords(),
                [2., 1., 7., 1., 7., 6., 2., 6.]
            );
        }

        {
            let mut mlines = MultiLineString2::new();
            mlines.add_linestring([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            mlines.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                mlines.iter().next().unwrap().coords(),
                [2., 1., 7., 1., 7., 6., 2., 6.]
            );
        }
    }

    #[test]
    #[should_panic(expected = "invalid coords_spans")]
    fn test_mline_invalid_coords_spans_1() {
        let all_coords: Vec<f64> = (0..=2).flat_map(|i| vec![i as f64, i as f64]).collect();
        let coords_spans: Vec<u32> = vec![99]; // out of `all_coords` range
        let _polygon: MultiLineString2<f64> =
            MultiLineString2::from_raw(all_coords.into(), coords_spans.into());
    }

    #[test]
    #[should_panic(expected = "invalid coords_spans")]
    fn test_mline_invalid_coords_spans_2() {
        let all_coords: Vec<f64> = vec![];
        let coords_spans: Vec<u32> = vec![0]; // out of `all_coords` range
        let _polygon: MultiLineString2<f64> =
            MultiLineString2::from_raw(all_coords.into(), coords_spans.into());
    }
}
