use std::borrow::Cow;

use super::linestring::LineString;
use num_traits::Float;

/// Computer-friendly Polygon
#[derive(Debug, Clone, Default)]
pub struct Polygon<'a, const D: usize, T: Float = f64> {
    /// 座標データ
    coords: Cow<'a, [T]>,
    /// 各 hole が何番目の頂点から始まるかの列
    hole_indices: Cow<'a, [u32]>,
}

pub type Polygon3<'a, T> = Polygon<'a, 3, T>;
pub type Polygon2<'a, T> = Polygon<'a, 2, T>;

impl<'a, const D: usize, T: Float> Polygon<'a, D, T> {
    pub fn new(all_coords: Cow<'a, [T]>, hole_indices: Cow<'a, [u32]>) -> Self {
        Self {
            coords: all_coords,
            hole_indices,
        }
    }

    #[inline]
    pub fn coords(&self) -> &[T] {
        self.coords.as_ref()
    }

    #[inline]
    pub fn hole_indices(&self) -> &[u32] {
        self.hole_indices.as_ref()
    }

    #[inline]
    pub fn exterior(&self) -> LineString<D, T> {
        LineString::new(if self.hole_indices.is_empty() {
            self.coords.as_ref().into()
        } else {
            self.coords[..self.hole_indices[0] as usize * D].into()
        })
    }

    pub fn interiors(&self) -> impl Iterator<Item = LineString<D, T>> {
        self.hole_indices
            .windows(2)
            .map(|a| (a[0] as usize * D, a[1] as usize * D))
            .chain(match self.hole_indices.is_empty() {
                true => None,
                false => Some((
                    self.hole_indices[self.hole_indices.len() - 1] as usize * D,
                    self.coords.len(),
                )),
            })
            .map(|(start, end)| LineString::new(self.coords[start..end].into()))
    }

    #[inline]
    pub fn clear(&mut self) {
        self.coords.to_mut().clear();
        self.hole_indices.to_mut().clear();
    }
}
