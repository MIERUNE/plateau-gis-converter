use std::borrow::Cow;

use super::polygon::Polygon;
use super::CoordNum;

/// Computer-friendly MultiPolygon
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct MultiPolygon<'a, const D: usize, T: CoordNum = f64> {
    /// すべての Polygon の座標データを連結したもの
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    all_coords: Cow<'a, [T]>,

    /// all_coords における各ポリゴンの開始頂点番号 (1つ目のポリゴンは必ず0なので省略)
    coords_spans: Cow<'a, [u32]>,

    /// 各ポリゴンでの各 hole の開始頂点インデクスの列を全ポリゴンについて連結したもの
    all_hole_indices: Cow<'a, [u32]>,

    /// all_hole_indices における各ポリゴンの開始位置 (1つ目のポリゴンは必ず0なので省略)
    holes_spans: Cow<'a, [u32]>,
}

pub type MultiPolygon2<'a, T = f64> = MultiPolygon<'a, 2, T>;
pub type MultiPolygon3<'a, T = f64> = MultiPolygon<'a, 3, T>;

impl<'a, const D: usize, T: CoordNum> MultiPolygon<'a, D, T> {
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new polygon from the given raw data, checking for validity.
    pub fn from_raw(
        all_coords: Cow<'a, [T]>,
        coords_spans: Cow<'a, [u32]>,
        all_hole_indices: Cow<'a, [u32]>,
        holes_spans: Cow<'a, [u32]>,
    ) -> Self {
        if all_coords.len() % D != 0 {
            panic!("all_coords.len() must be a multiple of D")
        }
        if coords_spans.len() != holes_spans.len() {
            panic!("coords_spans and holes_spans must have the same length");
        }

        // check multipolygon
        let mut cs_start = 0;
        let mut hs_start = 0;
        coords_spans
            .iter()
            .zip(holes_spans.iter())
            .chain(Some((
                &((all_coords.len() / D) as u32),
                &((all_hole_indices.len()) as u32),
            )))
            .for_each(|(&cs_end, &hs_end)| {
                if cs_start > cs_end {
                    panic!("invalid coords_spans");
                }
                if hs_start > hs_end {
                    panic!("invalid holes_spans");
                }
                // check polygon
                let coords = &all_coords[cs_start as usize * D..cs_end as usize * D];
                let hole_indices = &all_hole_indices[hs_start as usize..hs_end as usize];
                if let Some(&last) = hole_indices.last() {
                    let len = (coords.len() / D) as u32;
                    if last >= len || hole_indices.windows(2).any(|a| a[0] >= a[1]) {
                        panic!("invalid hole_indices")
                    }
                }
                (cs_start, hs_start) = (cs_end, hs_end);
            });

        Self {
            all_coords,
            coords_spans,
            holes_spans,
            all_hole_indices,
        }
    }

    /// Creates a new multipolygon from the given raw data, without validity check.
    pub fn from_raw_unchecked(
        all_coords: Cow<'a, [T]>,
        coords_spans: Cow<'a, [u32]>,
        holes_spans: Cow<'a, [u32]>,
        all_hole_indices: Cow<'a, [u32]>,
    ) -> Self {
        Self {
            all_coords,
            coords_spans,
            holes_spans,
            all_hole_indices,
        }
    }

    /// Returns the number of polygons in the multipolygon.
    pub fn len(&self) -> usize {
        match self.coords_spans.len() {
            0 => match self.all_coords.len() {
                0 => 0,
                _ => 1,
            },
            len => len + 1,
        }
    }

    /// Returns `true` if the multipolygon contains no polygons.
    pub fn is_empty(&self) -> bool {
        self.all_coords.is_empty()
    }

    /// Returns an iterator over the polygons
    pub fn iter(&self) -> Iter<'_, D, T> {
        Iter {
            mpoly: self,
            pos: 0,
        }
    }

    /// Returns the polygon at the given index.
    pub fn get(&self, index: usize) -> Polygon<D, T> {
        let len = self.len();
        let (c_start, c_end, h_start, h_end) = match index {
            index if index >= len => {
                panic!(
                    "index out of bounds: {} polygon(s) but index is {}",
                    len, index
                );
            }
            0 => (
                0,
                self.coords_spans
                    .first()
                    .map_or(self.all_coords.len(), |&i| i as usize * D),
                0,
                self.holes_spans
                    .first()
                    .map_or(self.all_hole_indices.len(), |&i| i as usize),
            ),
            index if index == len - 1 => (
                self.coords_spans[index - 1] as usize * D,
                self.all_coords.len(),
                self.holes_spans[index - 1] as usize,
                self.all_hole_indices.len(),
            ),
            _ => (
                self.coords_spans[index - 1] as usize * D,
                self.coords_spans[index] as usize * D,
                self.holes_spans[index - 1] as usize,
                self.holes_spans[index] as usize,
            ),
        };
        Polygon::from_raw_unchecked(
            (&self.all_coords[c_start..c_end]).into(),
            (&self.all_hole_indices[h_start..h_end]).into(),
        )
    }

    /// Clears the multipolygon, removing all polygon.
    pub fn clear(&mut self) {
        self.all_coords.to_mut().clear();
        self.all_hole_indices.to_mut().clear();
        self.coords_spans.to_mut().clear();
        self.holes_spans.to_mut().clear();
    }

    /// Adds a polygon with the given exterior ring.
    pub fn add_exterior<I: IntoIterator<Item = [T; D]>>(&mut self, iter: I) {
        if !self.all_coords.is_empty() {
            self.coords_spans
                .to_mut()
                .push((self.all_coords.len() / D) as u32);
            self.holes_spans
                .to_mut()
                .push(self.all_hole_indices.len() as u32);
        }

        let head = self.all_coords.len();
        self.all_coords.to_mut().extend(iter.into_iter().flatten());

        // remove closing point if exists
        let tail = self.all_coords.len();
        if tail > head + 2 * D && self.all_coords[head..head + D] == self.all_coords[tail - D..] {
            self.all_coords.to_mut().truncate(tail - D);
        }
    }

    /// Adds an interior ring to the last polygon.
    pub fn add_interior<I: IntoIterator<Item = [T; D]>>(&mut self, iter: I) {
        self.all_hole_indices
            .to_mut()
            .push((self.all_coords.len() / D) as u32 - *self.coords_spans.last().unwrap_or(&0));

        let head = self.all_coords.len();
        self.all_coords.to_mut().extend(iter.into_iter().flatten());

        // remove closing point if exists
        let tail = self.all_coords.len();
        if tail > head + 2 * D && self.all_coords[head..head + D] == self.all_coords[tail - D..] {
            self.all_coords.to_mut().truncate(tail - D);
        }
    }

    /// Create a new multipolygon by applying the given transformation to all coordinates in the MultiPolygon.
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

    /// Applies the given transformation to all coordinates in the MultiPolygon.
    pub fn transform_inplace(&mut self, f: impl Fn(&[T; D]) -> [T; D]) {
        self.all_coords.to_mut().chunks_exact_mut(D).for_each(|c| {
            let transformed = f(&c.try_into().unwrap());
            c.copy_from_slice(&transformed);
        });
    }
}

impl<'a, const D: usize, T: CoordNum> IntoIterator for &'a MultiPolygon<'_, D, T> {
    type Item = Polygon<'a, D, T>;
    type IntoIter = Iter<'a, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const D: usize, T: CoordNum> {
    mpoly: &'a MultiPolygon<'a, D, T>,
    pos: usize,
}

impl<'a, const D: usize, T: CoordNum> Iterator for Iter<'a, D, T> {
    type Item = Polygon<'a, D, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.mpoly.len() {
            let poly = self.mpoly.get(self.pos);
            self.pos += 1;
            Some(poly)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpoly_append() {
        let mut mpoly = MultiPolygon2::new();
        assert_eq!(mpoly.len(), 0);
        assert!(mpoly.is_empty());
        assert_eq!(mpoly.iter().count(), 0);

        // 1st polygon
        mpoly.add_exterior([[0., 0.], [5., 0.], [5., 5.], [0., 5.], [0., 0.]]);
        assert_eq!(mpoly.len(), 1);
        mpoly.add_interior([[1., 1.], [2., 1.], [2., 2.], [1., 2.], [1., 1.]]);
        assert_eq!(mpoly.len(), 1);
        mpoly.add_interior([[3., 3.], [4., 3.], [4., 4.], [3., 4.], [3., 3.]]);
        assert_eq!(mpoly.len(), 1);
        assert!(!mpoly.is_empty());
        for (i, poly) in mpoly.iter().enumerate() {
            match i {
                0 => assert_eq!(poly.exterior().len(), 4),
                _ => unreachable!(),
            }
        }

        // 2nd polygon
        mpoly.add_exterior([[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]);
        assert_eq!(mpoly.len(), 2);
        mpoly.add_interior([[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]);
        assert_eq!(mpoly.len(), 2);

        // 3rd polygon
        mpoly.add_exterior([[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]);
        assert_eq!(mpoly.len(), 3);
        mpoly.add_interior([[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]);
        assert_eq!(mpoly.len(), 3);

        for (i, poly) in mpoly.iter().enumerate() {
            match i {
                0 => assert_eq!(poly.interiors().count(), 2),
                1 => assert_eq!(poly.interiors().count(), 1),
                2 => assert_eq!(poly.interiors().count(), 1),
                _ => unreachable!(),
            }
        }

        mpoly.clear();
        assert_eq!(mpoly.len(), 0);
        assert!(mpoly.is_empty());
    }

    #[test]
    fn test_mpoly_empty_exterior() {
        let mut mpoly = MultiPolygon2::new();

        // Start the 1st polygon with an empty exterior
        mpoly.add_interior([[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]);
        assert_eq!(mpoly.get(0).exterior().len(), 0);
        assert_eq!(mpoly.len(), 1);
        // Start the 2nd polygon
        mpoly.add_exterior([[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]);
        assert_eq!(mpoly.len(), 2);
    }

    #[test]
    fn test_from_raw_valid() {
        // checked
        let _mpoly = MultiPolygon2::from_raw(
            [
                0.0, 0.0, 5.0, 0.0, 5.0, 5.0, 0.0, 5.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0, 2.0,
                3.0, 3.0, 4.0, 3.0, 4.0, 4.0, 3.0, 4.0, 4.0, 0.0, 7.0, 0.0, 7.0, 3.0, 4.0, 3.0,
                5.0, 1.0, 6.0, 1.0, 6.0, 2.0, 5.0, 2.0, 4.0, 0.0, 7.0, 0.0, 7.0, 3.0, 4.0, 3.0,
                5.0, 1.0, 6.0, 1.0, 6.0, 2.0, 5.0, 2.0,
            ][..]
                .into(),
            [12, 20][..].into(),
            [4, 8, 4, 4][..].into(),
            [2, 3][..].into(),
        );

        // unchecked
        let _mpoly = MultiPolygon2::from_raw_unchecked(
            [
                0.0, 0.0, 5.0, 0.0, 5.0, 5.0, 0.0, 5.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0, 2.0,
                3.0, 3.0, 4.0, 3.0, 4.0, 4.0, 3.0, 4.0, 4.0, 0.0, 7.0, 0.0, 7.0, 3.0, 4.0, 3.0,
                5.0, 1.0, 6.0, 1.0, 6.0, 2.0, 5.0, 2.0, 4.0, 0.0, 7.0, 0.0, 7.0, 3.0, 4.0, 3.0,
                5.0, 1.0, 6.0, 1.0, 6.0, 2.0, 5.0, 2.0,
            ][..]
                .into(),
            [12, 20][..].into(),
            [4, 8, 4, 4][..].into(),
            [2, 3][..].into(),
        );
    }

    #[test]
    fn test_transform() {
        {
            let mut mpoly = MultiPolygon2::new();
            mpoly.add_exterior([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            let new_mpoly = mpoly.transform(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(new_mpoly.get(0).coords(), [2., 1., 7., 1., 7., 6., 2., 6.]);
        }

        {
            let mut mpoly = MultiPolygon2::new();
            mpoly.add_exterior([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            mpoly.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(mpoly.get(0).coords(), [2., 1., 7., 1., 7., 6., 2., 6.]);
        }
    }

    #[test]
    #[should_panic]
    fn test_from_raw_invalid_1() {
        let _mpoly = MultiPolygon2::from_raw(
            [0.0, 0.0, 5.0, 0.0, 5.0, 5.0, 0.0, 1.0][..].into(),
            [2][..].into(),
            [1, 99][..].into(), // invalid
            [1][..].into(),
        );
    }

    #[test]
    #[should_panic]
    fn test_from_raw_invalid_2() {
        let _mpoly = MultiPolygon2::from_raw(
            [0.0, 0.0, 5.0, 0.0, 5.0, 5.0, 0.0, 1.0][..].into(),
            [99][..].into(), // invalid
            [1, 1][..].into(),
            [1][..].into(),
        );
    }

    #[test]
    #[should_panic]
    fn test_from_raw_invalid_3() {
        let _mpoly = MultiPolygon2::from_raw(
            [0.0, 0.0, 5.0, 0.0, 5.0, 5.0, 0.0, 1.0][..].into(),
            [2][..].into(),
            [1, 1][..].into(),
            [99][..].into(), // invalid
        );
    }

    #[test]
    #[should_panic]
    fn test_from_raw_invalid_4() {
        // checked
        let _mpoly = MultiPolygon2::from_raw(
            [0.0, 0.0, 5.0, 0.0, 5.0, 5.0, 0.0, 1.0][..].into(),
            [1, 2][..].into(),
            [][..].into(),
            [0][..].into(), // coords_spans.len() != holes_spans.len()
        );
    }

    #[test]
    #[should_panic]
    fn test_from_raw_invalid_5() {
        // checked
        let _mpoly = MultiPolygon2::from_raw(
            [0.0, 0.0, 5.0, 0.0, 5.0, 5.0, 0.0, 1.0][..].into(),
            [2, 1][..].into(), // not increasing
            [][..].into(),
            [0, 0][..].into(),
        );
    }

    #[test]
    #[should_panic]
    fn test_from_raw_invalid_6() {
        // checked
        let _mpoly = MultiPolygon2::from_raw(
            [0., 0., 1., 1., 2., 2., 3., 3., 4., 4., 5., 5.][..].into(),
            [2, 4][..].into(),
            [1, 1][..].into(),
            [1, 0][..].into(), // not increasing
        );
    }
}
