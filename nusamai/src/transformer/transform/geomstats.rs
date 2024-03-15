use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::{FeatureTypeDef, Schema, TypeDef, TypeRef},
};
use nusamai_plateau::Entity;

use crate::{pipeline::Feedback, transformer::Transform};

/// Jsonify all objects and arrays in the entity.
#[derive(Clone, Default)]
pub struct GeometryStatsTransform {}

impl Transform for GeometryStatsTransform {
    fn transform(&mut self, _feedback: &Feedback, mut entity: Entity, out: &mut Vec<Entity>) {
        // ...
        let Value::Object(obj) = &mut entity.root else {
            out.push(entity);
            return;
        };
        let ObjectStereotype::Feature { .. } = &obj.stereotype else {
            out.push(entity);
            return;
        };

        let mut max_h = f64::MIN;
        let mut min_h = f64::MAX;
        {
            let geom_store = entity.geometry_store.read().unwrap();
            geom_store.vertices.iter().for_each(|&v| {
                let [_lng, _lat, h] = v;
                max_h = max_h.max(h);
                min_h = min_h.min(h);
            });
        }

        if max_h != f64::MIN {
            obj.attributes
                .insert("maxHeight".to_string(), Value::Double(max_h));
        }
        if min_h != f64::MAX {
            obj.attributes
                .insert("minHeight".to_string(), Value::Double(min_h));
        }

        out.push(entity);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for ty in schema.types.values_mut() {
            match ty {
                TypeDef::Feature(FeatureTypeDef { attributes, .. }) => {
                    attributes.insert(
                        "maxHeight".to_string(),
                        nusamai_citygml::schema::Attribute::new(TypeRef::Double),
                    );
                    attributes.insert(
                        "minHeight".to_string(),
                        nusamai_citygml::schema::Attribute::new(TypeRef::Double),
                    );
                }
                TypeDef::Data(_) | TypeDef::Property(_) => {}
            }
        }
    }
}
