use std::borrow::Cow;

use super::CoordNum;

/// Computer-friendly LineString
#[derive(Debug, Clone, Default)]
pub struct LineString<'a, const D: usize, T: CoordNum = f64> {
    /// 座標データ
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    coords: Cow<'a, [T]>,
}

pub type LineString2<'a, T = f64> = LineString<'a, 2, T>;
pub type LineString3<'a, T = f64> = LineString<'a, 3, T>;

impl<'a, const D: usize, T: CoordNum> LineString<'a, D, T> {
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

    /// この LineString の座標列のイテレータを得る
    pub fn iter(&self) -> Iter<'_, D, T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            close: false,
        }
    }

    /// 始点と終点を閉じた座標列のイテレータを得る
    pub fn iter_closed(&self) -> Iter<'_, D, T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            close: true,
        }
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

impl<const D: usize, T: CoordNum> AsRef<[T]> for LineString<'_, D, T> {
    fn as_ref(&self) -> &[T] {
        self.coords.as_ref()
    }
}

impl<'a, const D: usize, T: CoordNum> IntoIterator for &'a LineString<'_, D, T> {
    type Item = &'a [T];
    type IntoIter = Iter<'a, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const D: usize, T: CoordNum> {
    slice: &'a [T],
    pos: usize,
    close: bool,
}

impl<'a, const D: usize, T: CoordNum> Iterator for Iter<'a, D, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += D;
        if self.pos <= self.slice.len() {
            Some(&self.slice[self.pos - D..self.pos])
        } else if self.close && self.slice.len() >= D && self.pos == self.slice.len() + D {
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
        let mut line: LineString2 =
            LineString2::from_raw((0..8).map(|v| v as f64).collect::<Vec<f64>>().into());
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

        line.clear();
        assert_eq!(line.len(), 0);
        assert!(line.is_empty());

        line.push(&[0., 1.]);
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
        let line: LineString2 =
            LineString2::from_raw((0..6).map(|v| v as f64).collect::<Vec<f64>>().into());
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
