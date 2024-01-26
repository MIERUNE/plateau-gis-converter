use crate::transformer::Transform;

use indexmap::map::MutableKeys;
use nusamai_citygml::object::{Entity, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::schema::TypeDef;

/// Transform to remove the namespace prefix from the attribute name.
///
/// e.g) bldg:measuredHeight -> measuredHeight
#[derive(Default, Clone)]
pub struct RemoveNamespaceTransform {}

impl Transform for RemoveNamespaceTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        edit_tree(&mut entity.root);
        out.push(entity);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for ty in schema.types.values_mut() {
            let atrs = match ty {
                TypeDef::Data(data) => &mut data.attributes,
                TypeDef::Feature(feat) => &mut feat.attributes,
                TypeDef::Property(_) => continue,
            };
            atrs.retain2(|key, _| {
                if let Some(pos) = key.find(':') {
                    *key = key[pos + 1..].to_string();
                }
                true
            });
        }
    }
}

fn edit_tree(value: &mut Value) {
    match value {
        Value::Object(obj) => {
            obj.attributes.retain2(|key, value| {
                edit_tree(value);
                if let Some(pos) = key.find(':') {
                    *key = key[pos + 1..].to_string();
                }
                true
            });
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                edit_tree(v);
            }
        }
        _ => {}
    }
}
