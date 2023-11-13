use std::borrow::Cow;

use super::polygon::Polygon;
use num_traits::Float;

/// Computer-friendly MultiPolygon
#[derive(Debug, Clone, Default)]
pub struct MultiPolygon<'a, const D: usize, T: Float = f64> {
    /// すべての Polygon の座標データを連結したもの
    ///
    /// e.g. `[x0, y0, z0, x1, y1, z1, ...]`
    all_coords: Cow<'a, [T]>,

    /// 各ポリゴンでの各 hole の開始頂点インデクスの列を全ポリゴンについて連結したもの
    all_hole_indices: Cow<'a, [u32]>,

    /// all_coords における各ポリゴンの開始頂点番号 (1つ目のポリゴンは必ず0なので省略)
    coords_spans: Cow<'a, [u32]>,

    /// all_hole_indices における各ポリゴンの開始位置 (1つ目のポリゴンは必ず0なので省略)
    holes_spans: Cow<'a, [u32]>,
}

pub type MultiPolygon2<'a, T = f64> = MultiPolygon<'a, 2, T>;
pub type MultiPolygon3<'a, T = f64> = MultiPolygon<'a, 3, T>;

impl<'a, const D: usize, T: Float> MultiPolygon<'a, D, T> {
    pub fn new(
        all_coords: Cow<'a, [T]>,
        all_hole_indices: Cow<'a, [u32]>,
        coords_spans: Cow<'a, [u32]>,
        holes_spans: Cow<'a, [u32]>,
    ) -> Self {
        Self {
            all_coords,
            coords_spans,
            holes_spans,
            all_hole_indices,
        }
    }

    /// この MultiPolygon に含まれる Polygon の数を返す
    #[inline]
    pub fn len(&self) -> usize {
        match self.coords_spans.len() {
            0 => match self.all_coords.len() {
                0 => 0,
                _ => 1,
            },
            len => len + 1,
        }
    }

    /// この MultiPolygon が空かどうかを返す
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.all_coords.is_empty()
    }

    /// この MultiPolygon に含まれる Polygon を返すイテレータを得る
    #[inline]
    pub fn iter(&self) -> Iter<'_, D, T> {
        Iter {
            mpoly: self,
            pos: 0,
        }
    }

    /// index 番目の Polygon を取得する
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
        Polygon::new(
            (&self.all_coords[c_start..c_end]).into(),
            (&self.all_hole_indices[h_start..h_end]).into(),
        )
    }

    /// この MultiPolygon を空にする
    #[inline]
    pub fn clear(&mut self) {
        self.all_coords.to_mut().clear();
        self.all_hole_indices.to_mut().clear();
        self.coords_spans.to_mut().clear();
        self.holes_spans.to_mut().clear();
    }

    /// exterior ring を追加する
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

    /// interior ring を追加する
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
}

impl<'a, const D: usize, T: Float> IntoIterator for &'a MultiPolygon<'_, D, T> {
    type Item = Polygon<'a, D, T>;
    type IntoIter = Iter<'a, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const D: usize, T: Float> {
    mpoly: &'a MultiPolygon<'a, D, T>,
    pos: usize,
}

impl<'a, const D: usize, T: Float> Iterator for Iter<'a, D, T> {
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
        let mut mpoly: MultiPolygon2 = Default::default();
        assert_eq!(mpoly.len(), 0);
        assert!(mpoly.is_empty());
        for _ in &mpoly {
            unreachable!();
        }

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

        for (i, poly) in mpoly.iter().enumerate() {
            match i {
                0 => assert_eq!(poly.interiors().count(), 2),
                1 => assert_eq!(poly.interiors().count(), 1),
                _ => unreachable!(),
            }
        }

        mpoly.clear();
        assert_eq!(mpoly.len(), 0);
        assert!(mpoly.is_empty());
    }
}
