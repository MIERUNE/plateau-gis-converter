use std::ops::BitOrAssign;

use crate::transformer::Transform;

use nusamai_citygml::object::{Entity, ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;

#[derive(Default, Clone)]
pub struct FilterLodTransform {}

impl Transform for FilterLodTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        let lodmask = find_lods(&entity.root);
        let target_lod = lodmask.highest_lod();
        // Use the highest LOD as the target LOD for now.
        if let Some(target_lod) = target_lod {
            edit_tree(&mut entity.root, target_lod);
            out.push(entity);
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

#[derive(Default, Clone)]
pub struct LodMask(
    u8, // lods bit mask
    u8, // lods_has_texture bit mask (TODO)
);

impl LodMask {
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
        self.1 |= rhs.1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lod_mask() {
        let mut mask = super::LodMask::default();
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
    }
}
