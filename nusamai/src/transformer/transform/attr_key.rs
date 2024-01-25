use crate::transformer::Transform;

use indexmap::map::MutableKeys;
use nusamai_citygml::object::Entity;
use nusamai_citygml::schema::Schema;
use nusamai_citygml::schema::TypeDef;

/// Transform to remove the namespace prefix from the attribute name.
///
/// e.g) bldg:measuredHeight -> measuredHeight
#[derive(Default)]
pub struct NamespaceRemovalTransform {}

impl Transform for NamespaceRemovalTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        entity.root.traverse_object_mut(|obj| {
            obj.attributes.retain2(|key, _| {
                if let Some(pos) = key.find(':') {
                    *key = key[pos + 1..].to_string();
                }
                true
            });
        });
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
