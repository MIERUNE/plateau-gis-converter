use std::borrow::Cow;

use super::CoordNum;

/// Computer-friendly LineString
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
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
    pub fn iter(&self) -> Iter<D, T> {
        Iter {
            slice: &self.coords,
            pos: 0,
            close: false,
        }
    }

    /// 始点と終点を閉じた座標列のイテレータを得る
    pub fn iter_closed(&self) -> Iter<D, T> {
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

    /// Create a new LineString by applying the given transformation to all coordinates.
    pub fn transform(&self, f: impl Fn(&[T; D]) -> [T; D]) -> Self {
        Self {
            coords: self
                .coords
                .chunks_exact(D)
                .flat_map(|v| f(&v.try_into().unwrap()))
                .collect(),
        }
    }

    /// Applies the given transformation to all coordinates in the LineString.
    pub fn transform_inplace(&mut self, f: impl Fn(&[T; D]) -> [T; D]) {
        self.coords.to_mut().chunks_exact_mut(D).for_each(|c| {
            let transformed = f(&c.try_into().unwrap());
            c.copy_from_slice(&transformed);
        });
    }

    /// Reverses the coordinates in the LineString.
    pub fn reverse_inplace(&mut self) {
        let len = self.coords.len();
        if len > 0 {
            let data = self.coords.to_mut();
            for i in 0..data.len() / D / 2 {
                for j in 0..D {
                    data.swap(i * D + j, len - (i + 1) * D + j);
                }
            }
        }
    }

    /// Reverses the winding order of the coordinates in the ring, preserving the first coordinate.
    pub fn reverse_ring_inplace(&mut self) {
        let len = self.coords.len();
        if len > D {
            let data = self.coords.to_mut();
            for i in 1..(data.len() / D + 1) / 2 {
                for j in 0..D {
                    data.swap(i * D + j, len - i * D + j);
                }
            }
        }
    }
}

// 2-dimensional only
impl<'a, T: CoordNum> LineString<'a, 2, T> {
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
        let mut prev = ring_iter.next().unwrap();
        for coord in ring_iter {
            area += (prev[0].to_f64().unwrap() * coord[1].to_f64().unwrap())
                - (prev[1].to_f64().unwrap() * coord[0].to_f64().unwrap());
            prev = coord;
        }
        area / 2.0
    }
}

impl<const D: usize, T: CoordNum> AsRef<[T]> for LineString<'_, D, T> {
    fn as_ref(&self) -> &[T] {
        self.coords.as_ref()
    }
}

impl<'a, const D: usize, T: CoordNum> IntoIterator for &'a LineString<'_, D, T> {
    type Item = [T; D];
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
    type Item = [T; D];

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += D;
        if self.pos <= self.slice.len() {
            Some(self.slice[self.pos - D..self.pos].try_into().unwrap())
        } else if self.close && self.slice.len() >= D && self.pos == self.slice.len() + D {
            Some(self.slice[..D].try_into().unwrap())
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
                0 => assert_eq!(coord, [0., 1.]),
                1 => assert_eq!(coord, [2., 3.]),
                2 => assert_eq!(coord, [4., 5.]),
                3 => assert_eq!(coord, [6., 7.]),
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
                0 => assert_eq!(coord, [0., 1.]),
                1 => assert_eq!(coord, [2., 3.]),
                2 => assert_eq!(coord, [4., 5.]),
                3 => assert_eq!(coord, [0., 1.]),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_transform() {
        {
            let line = LineString2::from_raw([0., 0., 5., 0., 5., 5., 0., 5.][..].into());
            let new_line = line.transform(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(new_line.coords(), [2., 1., 7., 1., 7., 6., 2., 6.]);
        }

        {
            let mut line = LineString2::from_raw([0., 0., 5., 0., 5., 5., 0., 5.][..].into());
            line.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(line.coords(), [2., 1., 7., 1., 7., 6., 2., 6.]);
        }
    }

    #[test]
    fn test_winding_order() {
        let line = LineString2::from_raw(vec![0.0, 0.0, 3.0, 0.0, 3.0, 3.0, 0.0, 3.0].into());
        assert!(line.is_ccw());
        assert!(!line.is_cw());

        let line = LineString2::from_raw(vec![0.0, 0.0, 0.0, 3.0, 3.0, 3.0, 3.0, 0.0].into());
        assert!(!line.is_ccw());
        assert!(line.is_cw());

        let line = LineString2::from_raw(vec![0.0, 0.0, 0.0, 0.0].into());
        assert!(!line.is_ccw());
        assert!(!line.is_cw());
    }

    #[test]
    fn test_reverse() {
        let mut line = LineString2::from_raw(
            vec![0.0, 0.0, 3.0, 0.0, 3.0, 1.0, 3.0, 3.0, 1.0, 3.0, 0.0, 3.0].into(),
        );
        line.reverse_inplace();
        assert_eq!(
            line.coords(),
            vec![0.0, 3.0, 1.0, 3.0, 3.0, 3.0, 3.0, 1.0, 3.0, 0.0, 0.0, 0.0]
        );

        let mut line =
            LineString2::from_raw(vec![0.0, 0.0, 3.0, 0.0, 6.0, 0.0, 6.0, 3.0, 3.0, 3.0].into());
        line.reverse_inplace();
        assert_eq!(
            line.coords(),
            vec![3.0, 3.0, 6.0, 3.0, 6.0, 0.0, 3.0, 0.0, 0.0, 0.0]
        );
    }

    #[test]
    fn test_ring_reverse() {
        let mut line = LineString2::from_raw(
            vec![0.0, 0.0, 3.0, 0.0, 3.0, 1.0, 3.0, 3.0, 1.0, 3.0, 0.0, 3.0].into(),
        );
        line.reverse_ring_inplace();
        assert_eq!(
            line.coords(),
            vec![0.0, 0.0, 0.0, 3.0, 1.0, 3.0, 3.0, 3.0, 3.0, 1.0, 3.0, 0.0]
        );

        let mut line =
            LineString2::from_raw(vec![0.0, 0.0, 3.0, 0.0, 6.0, 0.0, 6.0, 3.0, 3.0, 3.0].into());
        line.reverse_ring_inplace();
        assert_eq!(
            line.coords(),
            vec![0.0, 0.0, 3.0, 3.0, 6.0, 3.0, 6.0, 0.0, 3.0, 0.0]
        );
    }
}
