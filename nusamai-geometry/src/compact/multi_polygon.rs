use std::borrow::Cow;

use super::polygon::CompactPolygon;
use num_traits::Float;

/// Computer-friendly MultiPolygon
#[derive(Debug, Clone, Default)]
pub struct CompactMultiPolygon<'a, const D: usize, T: Float = f64> {
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

pub type CompactMultiPolygon2<'a, T = f64> = CompactMultiPolygon<'a, 2, T>;
pub type CompactMultiPolygon3<'a, T = f64> = CompactMultiPolygon<'a, 3, T>;

impl<'a, const D: usize, T: Float> CompactMultiPolygon<'a, D, T> {
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

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.all_coords.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = CompactPolygon<D, T>> {
        (0..self.len()).map(|index| self.get(index))
    }

    pub fn get(&self, index: usize) -> CompactPolygon<D, T> {
        let len = self.len();
        let (c_start, c_end, h_start, h_end) = match index {
            index if index >= len => {
                panic!(
                    "index out of bounds: {} polygon(s) but index is {}",
                    len, index
                );
            }
            index if index == len - 1 => (
                self.coords_spans[index - 1] as usize * D,
                self.all_coords.len(),
                self.holes_spans[index - 1] as usize,
                self.all_hole_indices.len(),
            ),
            0 => (
                0,
                self.coords_spans[0] as usize * D,
                0,
                self.holes_spans[0] as usize,
            ),
            _ => (
                self.coords_spans[index - 1] as usize * D,
                self.coords_spans[index] as usize * D,
                self.holes_spans[index - 1] as usize,
                self.holes_spans[index] as usize,
            ),
        };
        CompactPolygon::new(
            (&self.all_coords[c_start..c_end]).into(),
            (&self.all_hole_indices[h_start..h_end]).into(),
        )
    }

    #[inline]
    pub fn clear(&mut self) {
        self.all_coords.to_mut().clear();
        self.all_hole_indices.to_mut().clear();
        self.coords_spans.to_mut().clear();
        self.holes_spans.to_mut().clear();
    }

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
