use std::ops::{BitAnd, BitAndAssign, BitOrAssign};

use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
};
use nusamai_plateau::Entity;

use crate::{pipeline::Feedback, transformer::Transform};

#[derive(Clone, Copy)]
pub enum LodFilterMode {
    Highest,
    Lowest,
    TexturedMaxLod,

    // NOTE: Debug
    Lod1,
    Lod2,
    Lod3,
    Lod4,
    All,
}

#[derive()]
pub struct FilterLodTransform {
    mask: LodMask,
    mode: LodFilterMode,
}

impl FilterLodTransform {
    pub fn new(mask: LodMask, mode: LodFilterMode) -> Self {
        Self { mask, mode }
    }
}

/// Transform to filter and split the LODs
impl Transform for FilterLodTransform {
    fn transform(&mut self, _feedback: &Feedback, mut entity: Entity, out: &mut Vec<Entity>) {
        match self.mode {
            LodFilterMode::TexturedMaxLod => {
                let original_lods = find_lods(&entity.root) & self.mask;
                let mut current_lods = original_lods;
                let mut highest_lod_entity = None;

                while current_lods.0 != 0 {
                    let target_lod = current_lods.highest_lod();

                    if let Some(lod) = target_lod {
                        // Create a copy of the entity
                        let mut entity_copy = entity.clone();
                        edit_tree(&mut entity_copy.root, lod);

                        let has_textures = {
                            let appearance = entity_copy.appearance_store.read().unwrap();
                            !appearance.textures.is_empty()
                        };

                        if has_textures {
                            out.push(entity_copy);
                            return;
                        } else {
                            // Save the highest LOD entity
                            if highest_lod_entity.is_none() {
                                highest_lod_entity = Some(entity_copy);
                            }
                            // Exclude the current LOD and try the next LOD
                            current_lods.0 &= !(1 << lod);
                        }
                    } else {
                        println!("No valid LOD found");
                        break;
                    }
                }

                // If no texture is found, push entity with highest LOD
                if let Some(highest_entity) = highest_lod_entity {
                    out.push(highest_entity);
                }
            }
            LodFilterMode::Highest => {
                let lods = find_lods(&entity.root) & self.mask;
                let target_lod = lods.highest_lod();

                if let Some(target_lod) = target_lod {
                    edit_tree(&mut entity.root, target_lod);
                    out.push(entity);
                }
            }
            LodFilterMode::Lowest => {
                let lods = find_lods(&entity.root) & self.mask;
                let target_lod = lods.lowest_lod();

                if let Some(target_lod) = target_lod {
                    edit_tree(&mut entity.root, target_lod);
                    out.push(entity);
                }
            }
            _ => {}
        }
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

fn edit_tree(value: &mut Value, target_lod: u8) -> bool {
    match value {
        Value::Object(obj) => {
            let mut retain = false;
            if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
                geometries.retain(|geom| geom.lod == target_lod);
                retain |= !geometries.is_empty();
            } else {
                // Data or Object Stereotype
                retain = true;
            }
            obj.attributes.retain(|_, value| {
                let retain_child = edit_tree(value, target_lod);
                retain |= retain_child;
                retain_child
            });
            retain
        }
        Value::Array(arr) => {
            arr.retain_mut(|value| edit_tree(value, target_lod));
            !arr.is_empty()
        }
        _ => true,
    }
}

fn find_lods(value: &Value) -> LodMask {
    let mut mask = LodMask::default();
    match value {
        Value::Object(obj) => {
            if let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype {
                geometries.iter().for_each(|geom| mask.add_lod(geom.lod));
            }
            for value in obj.attributes.values() {
                mask |= find_lods(value);
            }
        }
        Value::Array(arr) => {
            arr.iter().for_each(|value| mask |= find_lods(value));
        }
        _ => {}
    }
    mask
}

#[derive(Default, Clone, Copy, Debug)]
pub struct LodMask(
    u8, // lods bit mask
);

impl LodMask {
    pub fn all() -> Self {
        Self(0b11111)
    }

    pub fn add_lod(&mut self, lod_no: u8) {
        self.0 |= 1 << lod_no;
    }

    pub fn remove_lod(&mut self, lod_no: u8) {
        self.0 |= 1 << lod_no;
    }

    pub fn has_lod(&self, lod_no: u8) -> bool {
        self.0 & (1 << lod_no) != 0
    }

    /// Returns the highest LOD number.
    ///
    /// It returns `None` if none of the LODs are set.
    pub fn highest_lod(&self) -> Option<u8> {
        match self.0 {
            0 => None,
            _ => Some(7 - self.0.leading_zeros() as u8),
        }
    }

    /// Returns the lowest LOD number.
    ///
    /// It returns `None` if none of the LODs are set.
    pub fn lowest_lod(&self) -> Option<u8> {
        match self.0 {
            0 => None,
            _ => Some(self.0.trailing_zeros() as u8),
        }
    }
}

impl BitOrAssign for LodMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for LodMask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAnd for LodMask {
    type Output = LodMask;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lod_mask() {
        let mut mask = LodMask::default();
        assert_eq!(mask.lowest_lod(), None);
        assert_eq!(mask.highest_lod(), None);

        mask.add_lod(1);
        assert_eq!(mask.lowest_lod(), Some(1));
        assert_eq!(mask.highest_lod(), Some(1));
        assert!(!mask.has_lod(0));

        mask.add_lod(2);
        assert_eq!(mask.lowest_lod(), Some(1));
        assert_eq!(mask.highest_lod(), Some(2));
        assert!(!mask.has_lod(3));

        mask.add_lod(3);
        assert_eq!(mask.lowest_lod(), Some(1));
        assert_eq!(mask.highest_lod(), Some(3));
        assert!(mask.has_lod(3));

        // bitand
        let mut mask2 = LodMask::default();
        mask2.add_lod(3);
        assert!((mask & mask2).has_lod(3));
        assert!(!(mask & mask2).has_lod(1));
    }
}
