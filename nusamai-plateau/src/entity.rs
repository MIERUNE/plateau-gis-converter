use std::sync::{Arc, RwLock};

use hashbrown::HashSet;
use nusamai_citygml::{geometry::GeometryStore, object::Value, GeometryRefs};
use nusamai_citygml::{
    object::{Object, ObjectStereotype},
    GeometryRef,
};

use crate::appearance::AppearanceStore;

/// City objects, features, objects or data
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Entity {
    /// GML id
    pub id: String,
    /// Description
    pub description: Option<String>,
    /// City object type
    pub name: String,
    /// Attribute tree
    pub root: Value,
    /// Base url of the entity
    pub base_url: url::Url,
    /// All geometries referenced by the attribute tree
    pub geometry_store: Arc<RwLock<GeometryStore>>,
    /// All appearances used in this city object
    pub appearance_store: Arc<RwLock<AppearanceStore>>,
    /// Bounded by
    pub bounded_by: Vec<BoundedBy>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct BoundedBy {
    pub id: String,
    pub geometry_refs: GeometryRefs,
}

#[derive(Default, Clone)]
pub struct GeometricMergedownTransform {
    geoms_buf: HashSet<GeometryRef>,
}

impl GeometricMergedownTransform {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn transform(&mut self, entity: &mut Entity) {
        if let Value::Object(obj) = &mut entity.root {
            self.collect_all_geoms(obj);
            if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
                *geometries = self.geoms_buf.drain().collect();
            }
        }
    }

    fn collect_all_geoms(&mut self, obj: &mut Object) -> bool {
        let mut is_feature = false;
        if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
            is_feature = true;
            self.geoms_buf.extend(geometries.drain(..));
        }

        obj.attributes.retain(|_key, value| match value {
            Value::Object(obj) => !self.collect_all_geoms(obj),
            Value::Array(arr) => {
                arr.retain_mut(|value| {
                    if let Value::Object(obj) = value {
                        !self.collect_all_geoms(obj)
                    } else {
                        true
                    }
                });
                !arr.is_empty()
            }
            _ => true,
        });
        is_feature
    }
}
