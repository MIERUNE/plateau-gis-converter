use std::{borrow::Cow, ops::Range};

use super::{linestring::LineString, Coord};

/// Computer-friendly MultiString
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiLineString<'a, T: Coord> {
    /// All coordinates of all LineStrings
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    all_coords: Cow<'a, [T]>,

    /// A sequence of indices of all_coords from which each linestring starts
    /// (the first linestring always starts from 0 so it is omitted)
    ///
    /// e.g. `[5, 12, 23]`
    coords_spans: Cow<'a, [u32]>,
}

pub type MultiLineString2<'a, C = f64> = MultiLineString<'a, [C; 2]>;
pub type MultiLineString3<'a, C = f64> = MultiLineString<'a, [C; 3]>;

impl<'a, T: Coord> MultiLineString<'a, T> {
    /// Creates an empty MultiLineString.
    pub fn new() -> Self {
        Self {
            all_coords: Cow::Borrowed(&[]),
            coords_spans: Cow::Borrowed(&[]),
        }
    }

    pub fn from_raw(all_coords: Cow<'a, [T]>, coords_spans: Cow<'a, [u32]>) -> Self {
        // Check if all span values are within range and monotonically increasing
        if let Some(&last) = coords_spans.last() {
            let len = (all_coords.len()) as u32;
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

    /// Returns iterator over the linestrings.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            ls: self,
            pos: 0,
            end: self.len(),
        }
    }

    /// Returns iterator over the linestrings in the given range.
    pub fn iter_range(&self, range: Range<usize>) -> Iter<T> {
        Iter {
            ls: self,
            pos: range.start,
            end: range.end,
        }
    }

    /// Returns the number of linestrings.
    pub fn len(&self) -> usize {
        if self.all_coords.is_empty() {
            0
        } else {
            self.coords_spans.len() + 1
        }
    }

    /// Returns `true` if the MultiLineString is empty.
    pub fn is_empty(&self) -> bool {
        self.all_coords.is_empty()
    }

    /// Removes all linestrings.
    pub fn clear(&mut self) {
        self.all_coords.to_mut().clear();
        self.coords_spans.to_mut().clear();
    }

    /// Add a linestring to the MultiLineString.
    pub fn add_linestring<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        if !self.all_coords.is_empty() {
            self.coords_spans
                .to_mut()
                .push((self.all_coords.len()) as u32);
        }
        self.all_coords.to_mut().extend(iter);
    }

    /// Create a new MultiLineString by applying the given transformation to all coordinates.
    pub fn transform<T2: Coord>(&self, f: impl Fn(&T) -> T2) -> MultiLineString<T2> {
        MultiLineString {
            all_coords: self.all_coords.iter().map(f).collect(),
            coords_spans: self.coords_spans.clone(),
        }
    }

    /// Applies the given transformation to all coordinates in the MultiLineString.
    pub fn transform_inplace(&mut self, mut f: impl FnMut(&T) -> T) {
        self.all_coords.to_mut().iter_mut().for_each(|c| {
            *c = f(c);
        });
    }
}

impl<'a, T: Coord> IntoIterator for &'a MultiLineString<'_, T> {
    type Item = LineString<'a, T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T: Coord> {
    ls: &'a MultiLineString<'a, T>,
    pos: usize,
    end: usize,
}

impl<'a, T: Coord> Iterator for Iter<'a, T> {
    type Item = LineString<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ls.is_empty() {
            return None;
        }
        if self.pos < self.end {
            let start = match self.pos {
                0 => 0,
                _ => self.ls.coords_spans[self.pos - 1] as usize,
            };
            let end = if self.pos < self.ls.coords_spans.len() {
                self.ls.coords_spans[self.pos] as usize
            } else {
                self.ls.all_coords.len()
            };
            let coords = &self.ls.all_coords[start..end];
            self.pos += 1;
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
            (0..7)
                .map(|v| [(v * 2) as f64, (v * 2 + 1) as f64])
                .collect::<Vec<_>>()
                .into(),
            vec![3, 5].into(),
        );
        assert_eq!(mline.len(), 3);
        assert_eq!(mline.iter().count(), 3);
        for (i, line) in mline.iter().enumerate() {
            match i {
                0 => assert_eq!(line.raw_coords(), &[[0., 1.], [2., 3.], [4., 5.]]),
                1 => assert_eq!(line.raw_coords(), &[[6., 7.], [8., 9.]]),
                2 => assert_eq!(line.raw_coords(), &[[10., 11.], [12., 13.]]),
                _ => unreachable!(),
            }
        }
        mline.clear();
        assert_eq!(mline.len(), 0);
    }

    #[test]
    fn test_mline_one_linestring() {
        let mline = MultiLineString2::from_raw_unchecked(
            Cow::Borrowed(&[[1., 2.], [3., 4.]]),
            Cow::Borrowed(&[]),
        );
        assert_eq!(mline.len(), 1);
        assert!(!mline.is_empty());
        for (i, _line) in mline.iter().enumerate() {
            match i {
                0 => assert_eq!(_line.raw_coords(), &[[1., 2.], [3., 4.]]),
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

        mline.add_linestring(vec![[1., 1.], [2., 2.], [3., 3.], [4., 4.]]);
        assert_eq!(mline.len(), 4);

        for (i, line) in mline.iter().enumerate() {
            match i {
                0 => assert_eq!(line.raw_coords(), &[[0., 0.], [1., 1.], [2., 2.]]),
                1 => assert_eq!(line.raw_coords(), &[[3., 3.], [4., 4.], [5., 5.]]),
                2 => assert_eq!(line.raw_coords(), &[[6., 6.], [7., 7.], [8., 8.], [9., 9.]]),
                3 => assert_eq!(line.raw_coords(), &[[1., 1.], [2., 2.], [3., 3.], [4., 4.]]),
                _ => unreachable!(),
            }
        }

        for (i, line) in mline.iter_range(1..3).enumerate() {
            match i {
                0 => assert_eq!(line.raw_coords(), &[[3., 3.], [4., 4.], [5., 5.]]),
                1 => assert_eq!(line.raw_coords(), &[[6., 6.], [7., 7.], [8., 8.], [9., 9.]]),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_transform() {
        {
            let mut mlines: MultiLineString2 = MultiLineString2::new();
            mlines.add_linestring([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            let new_mlines = mlines.transform(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                new_mlines.iter().next().unwrap().raw_coords(),
                [[2., 1.], [7., 1.], [7., 6.], [2., 6.]]
            );
        }

        {
            let mut mlines = MultiLineString2::new();
            mlines.add_linestring([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            mlines.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(
                mlines.iter().next().unwrap().raw_coords(),
                [[2., 1.], [7., 1.], [7., 6.], [2., 6.]]
            );
        }
    }

    #[test]
    #[should_panic(expected = "invalid coords_spans")]
    fn test_mline_invalid_coords_spans_1() {
        let all_coords: Vec<[f64; 2]> = (0..=2).map(|i| [i as f64, i as f64]).collect();
        let coords_spans: Vec<u32> = vec![99]; // out of `all_coords` range
        let _polygon: MultiLineString2<f64> =
            MultiLineString2::from_raw(all_coords.into(), coords_spans.into());
    }

    #[test]
    #[should_panic(expected = "invalid coords_spans")]
    fn test_mline_invalid_coords_spans_2() {
        let all_coords: Vec<[f64; 2]> = vec![];
        let coords_spans: Vec<u32> = vec![0]; // out of `all_coords` range
        let _polygon: MultiLineString2<f64> =
            MultiLineString2::from_raw(all_coords.into(), coords_spans.into());
    }
}
