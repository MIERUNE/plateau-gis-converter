use std::sync::{Arc, RwLock};

use crate::transformer::Transform;

use nusamai_citygml::object::{Entity, Map, ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryStore;

#[derive(Default, Clone)]
pub struct FlattenFeatureTransform {}

impl Transform for FlattenFeatureTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        let geom_store = entity.geometry_store;
        flatten_feature(entity.root, &geom_store, out);
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

fn flatten_feature(
    value: Value,
    geom_store: &Arc<RwLock<GeometryStore>>,
    out: &mut Vec<Entity>,
) -> Option<Value> {
    match value {
        Value::Object(mut obj) => {
            let mut new_attribs = Map::default();
            for (key, value) in obj.attributes.drain(..) {
                if let Some(v) = flatten_feature(value, geom_store, out) {
                    new_attribs.insert(key, v);
                }
            }
            obj.attributes = new_attribs;

            if let ObjectStereotype::Feature { .. } = &obj.stereotype {
                out.push(Entity {
                    root: Value::Object(obj),
                    geometry_store: geom_store.clone(),
                });
                None
            } else {
                // Data or Object Stereotype
                Some(Value::Object(obj))
            }
        }
        Value::Array(mut arr) => {
            let mut new_arr = Vec::new();
            for value in arr.drain(..) {
                if let Some(v) = flatten_feature(value, geom_store, out) {
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
