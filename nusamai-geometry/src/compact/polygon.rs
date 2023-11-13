use std::borrow::Cow;

use super::linestring::CompactLineString;
use num_traits::Float;

/// Computer-friendly Polygon
#[derive(Debug, Clone, Default)]
pub struct CompactPolygon<'a, const D: usize, T: Float = f64> {
    coords: Cow<'a, [T]>,
    hole_indices: Cow<'a, [u32]>,
}

pub type CompactPolygon3<'a, T> = CompactPolygon<'a, 3, T>;
pub type CompactPolygon2<'a, T> = CompactPolygon<'a, 2, T>;

impl<'a, const D: usize, T: Float> CompactPolygon<'a, D, T> {
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
    pub fn exterior(&self) -> CompactLineString<D, T> {
        CompactLineString::new(if self.hole_indices.is_empty() {
            self.coords.as_ref().into()
        } else {
            self.coords[..self.hole_indices[0] as usize * D].into()
        })
    }

    pub fn interiors(&self) -> impl Iterator<Item = CompactLineString<D, T>> {
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
            .map(|(start, end)| CompactLineString::new(self.coords[start..end].into()))
    }

    #[inline]
    pub fn clear(&mut self) {
        self.coords.to_mut().clear();
        self.hole_indices.to_mut().clear();
    }
}
