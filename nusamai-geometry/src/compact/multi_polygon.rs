use std::borrow::Cow;
use std::ops::Range;

use super::polygon::Polygon;
use super::CoordNum;

/// Computer-friendly MultiPolygon
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiPolygon<'a, const D: usize, T: CoordNum = f64> {
    /// All coordinates of all polygons
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    all_coords: Cow<'a, [T]>,

    /// A sequence of indices of all_coords from which each polygon starts
    /// (the first polygon always starts from 0 so it is omitted)
    coords_spans: Cow<'a, [u32]>,

    /// All hole_indices (See `Polygon`) of all polygons
    all_hole_indices: Cow<'a, [u32]>,

    /// A sequence of indices of all_hole_indices from which each polygon starts
    /// (the first polygon always starts from 0 so it is omitted)
    holes_spans: Cow<'a, [u32]>,
}

pub type MultiPolygon2<'a, T = f64> = MultiPolygon<'a, 2, T>;
pub type MultiPolygon3<'a, T = f64> = MultiPolygon<'a, 3, T>;

impl<'a, const D: usize, T: CoordNum> MultiPolygon<'a, D, T> {
    /// Creates an empty MultiPolygon.
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
    pub fn iter(&self) -> Iter<D, T> {
        Iter {
            mpoly: self,
            pos: 0,
            end: self.len(),
        }
    }

    /// Returns an iterator over the polygons in the given range.
    pub fn iter_range(&self, range: Range<usize>) -> Iter<D, T> {
        Iter {
            mpoly: self,
            pos: range.start,
            end: range.end,
        }
    }

    /// Returns the polygon at the given index.
    pub fn get(&'a self, index: usize) -> Polygon<'a, D, T> {
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

    /// Clears the multipolygon, removing all polygons.
    pub fn clear(&mut self) {
        self.all_coords.to_mut().clear();
        self.all_hole_indices.to_mut().clear();
        self.coords_spans.to_mut().clear();
        self.holes_spans.to_mut().clear();
    }

    /// Adds a polygon to the multipolygon.
    pub fn push(&mut self, poly: &Polygon<D, T>) {
        self.add_exterior(&poly.exterior());
        for hole in poly.interiors() {
            self.add_interior(&hole);
        }
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

    /// Create a new MultiPolygon by applying the given transformation to all coordinates.
    pub fn transform<const D2: usize, T2: CoordNum>(
        &self,
        f: impl Fn(&[T; D]) -> [T2; D2],
    ) -> MultiPolygon<D2, T2> {
        MultiPolygon {
            all_coords: self
                .all_coords
                .chunks_exact(D)
                .flat_map(|v| f(&v.try_into().unwrap()))
                .collect(),
            coords_spans: self.coords_spans.clone(),
            all_hole_indices: self.all_hole_indices.clone(),
            holes_spans: self.holes_spans.clone(),
        }
    }

    /// Applies the given transformation to all coordinates in the MultiPolygon.
    pub fn transform_inplace(&mut self, mut f: impl FnMut(&[T; D]) -> [T; D]) {
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
    end: usize,
}

impl<'a, const D: usize, T: CoordNum> Iterator for Iter<'a, D, T> {
    type Item = Polygon<'a, D, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.end {
            // TODO: optimize
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
    use super::super::polygon::Polygon2;
    use super::*;

    #[test]
    fn test_mpoly_add() {
        let mut mpoly = MultiPolygon2::new();
        assert_eq!(mpoly.len(), 0);
        assert!(mpoly.is_empty());
        assert_eq!(mpoly.iter().count(), 0);

        // 1st polygon
        let mut poly1 = Polygon2::new();
        poly1.add_ring([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]); // exterior
        poly1.add_ring([[1., 1.], [2., 1.], [2., 2.], [1., 2.]]); // interior
        poly1.add_ring([[3., 3.], [4., 3.], [4., 4.], [3., 4.]]); // interior
        mpoly.push(&poly1);
        assert!(!mpoly.is_empty());
        assert_eq!(mpoly.len(), 1);

        // 2nd polygon
        let mut poly2 = Polygon2::new();
        poly2.add_ring([[4., 0.], [7., 0.], [7., 3.], [4., 3.]]); // exterior
        poly2.add_ring([[5., 1.], [6., 1.], [6., 2.], [5., 2.]]); // interior
        mpoly.push(&poly2);
        assert_eq!(mpoly.len(), 2);

        // 3rd polygon
        let mut poly3 = Polygon2::new();
        poly3.add_ring([[4., 0.], [7., 0.], [7., 3.], [4., 3.]]); // exterior
        mpoly.push(&poly3);
        assert_eq!(mpoly.len(), 3);

        for (i, poly) in mpoly.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!(
                        poly.exterior().iter().collect::<Vec<_>>(),
                        [[0., 0.], [5., 0.], [5., 5.], [0., 5.]]
                    );
                    assert_eq!(poly.interiors().count(), 2);
                    assert_eq!(
                        poly.interiors()
                            .map(|r| r.iter().collect::<Vec<_>>())
                            .collect::<Vec<_>>(),
                        [
                            [[1., 1.], [2., 1.], [2., 2.], [1., 2.]],
                            [[3., 3.], [4., 3.], [4., 4.], [3., 4.]],
                        ]
                    );
                }
                1 => {
                    assert_eq!(
                        poly.exterior().iter().collect::<Vec<_>>(),
                        [[4., 0.], [7., 0.], [7., 3.], [4., 3.]]
                    );
                    assert_eq!(poly.interiors().count(), 1);
                    assert_eq!(
                        poly.interiors()
                            .map(|r| r.iter().collect::<Vec<_>>())
                            .collect::<Vec<_>>(),
                        [[[5., 1.], [6., 1.], [6., 2.], [5., 2.]]]
                    );
                }
                2 => {
                    assert_eq!(
                        poly.exterior().iter().collect::<Vec<_>>(),
                        [[4., 0.], [7., 0.], [7., 3.], [4., 3.]]
                    );
                    assert_eq!(poly.interiors().count(), 0);
                }
                _ => unreachable!(),
            }
        }
    }

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

        for (i, poly) in mpoly.iter().enumerate() {
            match i {
                0 => assert_eq!(poly.interiors().count(), 2),
                1 => assert_eq!(poly.interiors().count(), 1),
                2 => assert_eq!(poly.interiors().count(), 0),
                _ => unreachable!(),
            }
        }

        for (i, poly) in mpoly.iter_range(0..1).enumerate() {
            match i {
                0 => assert_eq!(poly.interiors().count(), 2),
                _ => unreachable!(),
            }
        }

        for (i, poly) in mpoly.iter_range(1..2).enumerate() {
            match i {
                0 => assert_eq!(poly.interiors().count(), 1),
                _ => unreachable!(),
            }
        }

        let mut found = false;
        for (i, poly) in mpoly.iter_range(2..3).enumerate() {
            match i {
                0 => {
                    assert_eq!(poly.interiors().count(), 0);
                    found = true;
                }
                _ => unreachable!(),
            }
        }
        assert!(found);

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
            assert_eq!(
                new_mpoly.get(0).raw_coords(),
                [2., 1., 7., 1., 7., 6., 2., 6.]
            );
        }

        {
            let mut mpoly = MultiPolygon2::new();
            mpoly.add_exterior([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
            mpoly.transform_inplace(|[x, y]| [x + 2., y + 1.]);
            assert_eq!(mpoly.get(0).raw_coords(), [2., 1., 7., 1., 7., 6., 2., 6.]);
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
