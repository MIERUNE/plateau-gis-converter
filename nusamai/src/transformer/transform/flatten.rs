use std::sync::{Arc, RwLock};

use crate::transformer::Transform;

use nusamai_citygml::object::{Entity, Map, Object, ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryStore;

pub struct FlattenFeatureTransform {
    split_thematic_surfaces: bool,
}

impl Transform for FlattenFeatureTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        let geom_store = entity.geometry_store;
        self.flatten_feature(entity.root, &geom_store, out);
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

impl FlattenFeatureTransform {
    pub fn new(split_thematic_surfaces: bool) -> Self {
        Self {
            split_thematic_surfaces,
        }
    }

    fn flatten_feature(
        &self,
        value: Value,
        geom_store: &Arc<RwLock<GeometryStore>>,
        out: &mut Vec<Entity>,
    ) -> Option<Value> {
        match value {
            Value::Object(mut obj) => {
                let mut new_attribs = Map::default();
                for (key, value) in obj.attributes.drain(..) {
                    if let Some(v) = self.flatten_feature(value, geom_store, out) {
                        new_attribs.insert(key, v);
                    }
                }
                obj.attributes = new_attribs;

                // if feature
                if let ObjectStereotype::Feature { .. } = &obj.stereotype {
                    if self.is_split_target(&obj) {
                        out.push(Entity {
                            root: Value::Object(obj),
                            geometry_store: geom_store.clone(),
                        });
                        return None;
                    }
                }

                Some(Value::Object(obj))
            }
            Value::Array(mut arr) => {
                let mut new_arr = Vec::new();
                for value in arr.drain(..) {
                    if let Some(v) = self.flatten_feature(value, geom_store, out) {
                        new_arr.push(v)
                    }
                }
                if arr.is_empty() {
                    None
                } else {
                    Some(Value::Array(arr))
                }
            }
            _ => Some(value),
        }
    }

    fn is_split_target(&self, obj: &Object) -> bool {
        if let ObjectStereotype::Feature { .. } = &obj.stereotype {
            // TODO: more robust approach to deterine if the feature is a thematic surface
            if self.split_thematic_surfaces {
                true
            } else {
                !obj.typename.ends_with("Surface")
                    && !obj.typename.ends_with(":Window")
                    && !obj.typename.ends_with(":Door")
                    && !obj.typename.ends_with("TrafficArea")
            }
        } else {
            false
        }
    }
}
