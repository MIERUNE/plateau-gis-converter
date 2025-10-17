use std::sync::{Arc, RwLock};

use hashbrown::HashSet;
use nusamai_citygml::object::Map;
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
    pub id: Option<String>,
    /// City object type
    pub typename: Option<String>,
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

pub struct FlattenTreeTransform;

enum Parent {
    Feature { id: String, typename: String },
    Data { typename: String }, // Data stereotype does not have an id
    Object { id: String, typename: String },
}

impl FlattenTreeTransform {
    pub fn transform(entity: Entity) -> Vec<Entity> {
        let geom_store = entity.geometry_store;
        let appearance_store = entity.appearance_store;
        let mut out = Vec::new();
        Self::flatten_entity(
            entity.root,
            &geom_store,
            &appearance_store,
            &entity.bounded_by,
            &mut out,
            &None,
        );
        out
    }

    fn flatten_entity(
        value: Value,
        geom_store: &Arc<RwLock<GeometryStore>>,
        appearance_store: &Arc<RwLock<AppearanceStore>>,
        bounded_by: &Vec<BoundedBy>,
        out: &mut Vec<Entity>,
        parent: &Option<Parent>,
    ) -> Option<Value> {
        let id = value.id().map(|v| v.to_string());
        let typename = value.typename().map(|v| v.to_string());
        match value {
            Value::Object(mut obj) => {
                let new_parent = match &obj.stereotype {
                    ObjectStereotype::Feature { id, .. } => Some(Parent::Feature {
                        id: id.to_string(),
                        typename: obj.typename.to_string(),
                    }),
                    ObjectStereotype::Data => Some(Parent::Data {
                        typename: obj.typename.to_string(),
                    }),
                    ObjectStereotype::Object { id, .. } => Some(Parent::Object {
                        id: id.to_string(),
                        typename: obj.typename.to_string(),
                    }),
                };
                // Attributes
                let mut new_attribs = Map::default();
                for (key, value) in obj.attributes.drain(..) {
                    if let Some(v) = Self::flatten_entity(
                        value,
                        geom_store,
                        appearance_store,
                        bounded_by,
                        out,
                        &new_parent,
                    ) {
                        new_attribs.insert(key, v);
                    }
                }
                obj.attributes = new_attribs;

                if Self::is_flatten_target(&obj) {
                    // set parent id and type to attributes
                    if let Some(parent) = parent {
                        match parent {
                            Parent::Feature { id, typename } => {
                                obj.attributes
                                    .insert("parentId".to_string(), Value::String(id.to_string()));
                                obj.attributes.insert(
                                    "parentType".to_string(),
                                    Value::String(typename.to_string()),
                                );
                            }
                            Parent::Data { typename } => {
                                obj.attributes.insert(
                                    "parentType".to_string(),
                                    Value::String(typename.to_string()),
                                );
                            }
                            Parent::Object { id, typename } => {
                                obj.attributes
                                    .insert("parentId".to_string(), Value::String(id.to_string()));
                                obj.attributes.insert(
                                    "parentType".to_string(),
                                    Value::String(typename.to_string()),
                                );
                            }
                        }
                    }

                    out.push(Entity {
                        id,
                        typename, // OLD: Use typename from value.typename()
                        root: Value::Object(obj),
                        base_url: url::Url::parse("file:///dummy").expect("should be valid"),
                        geometry_store: geom_store.clone(),
                        appearance_store: appearance_store.clone(),
                        bounded_by: bounded_by.clone(),
                    });
                    return None;
                }

                Some(Value::Object(obj))
            }
            Value::Array(mut arr) => {
                let mut new_arr = Vec::with_capacity(arr.len());
                for value in arr.drain(..) {
                    if let Some(v) = Self::flatten_entity(
                        value,
                        geom_store,
                        appearance_store,
                        bounded_by,
                        out,
                        parent,
                    ) {
                        new_arr.push(v)
                    }
                }
                if new_arr.is_empty() {
                    None
                } else {
                    Some(Value::Array(new_arr))
                }
            }
            _ => Some(value),
        }
    }

    fn is_flatten_target(obj: &Object) -> bool {
        obj.typename != "gen:genericAttribute"
    }
}
