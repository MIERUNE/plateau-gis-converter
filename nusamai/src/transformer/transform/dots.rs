use nusamai_citygml::{
    object::{Map, Value},
    schema::{Schema, TypeDef},
};
use nusamai_plateau::Entity;

use crate::{pipeline::Feedback, transformer::Transform};

/// Collect all attributes in dot notation
#[derive(Default, Clone)]
pub struct DotNotationTransform {
    path_buf: String,
}

impl Transform for DotNotationTransform {
    fn transform(&mut self, _feedback: &Feedback, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(mut obj) = entity.root {
            let mut new_attrs = Default::default();
            let path = &mut self.path_buf;
            path.clear();

            collect_all_attrs(&mut new_attrs, path, obj.attributes);

            obj.attributes = new_attrs;
            entity.root = Value::Object(obj);
            out.push(entity);
        }
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for ty in schema.types.values_mut() {
            match ty {
                TypeDef::Data(data) => data.additional_attributes = true,
                TypeDef::Feature(feat) => feat.additional_attributes = true,
                TypeDef::Property(_) => continue,
            };
        }
    }
}

fn collect_all_attrs(new_attrs: &mut Map, path: &mut String, attributes: Map) {
    for (key, value) in attributes {
        let path_len = path.len();
        path.push_str(&key);
        match value {
            Value::Object(obj) => {
                path.push('.');
                collect_all_attrs(new_attrs, path, obj.attributes);
            }
            Value::Array(arr) => {
                path.push('.');
                for (i, value) in arr.into_iter().enumerate() {
                    let len = path.len();
                    path.push_str(&format!("{i}"));
                    match value {
                        Value::Object(obj) => {
                            path.push('.');
                            collect_all_attrs(new_attrs, path, obj.attributes);
                        }
                        _ => {
                            new_attrs.insert(path.clone(), value);
                        }
                    }
                    path.truncate(len);
                }
            }
            _ => {
                new_attrs.insert(path.to_string(), value);
            }
        }
        path.truncate(path_len);
    }
}
