mod appearance;
mod attrname;
mod dots;
mod flatten;
mod geommerge;
mod jsonify;
mod lods;
mod projection;

pub use appearance::*;
pub use attrname::*;
pub use dots::*;
pub use flatten::*;
pub use geommerge::*;
pub use jsonify::*;
pub use lods::*;
pub use projection::*;

use super::Transform;
use nusamai_citygml::schema::Schema;
use nusamai_plateau::Entity;

/// Perform transforms in sequence
#[derive(Default)]
pub struct SerialTransform {
    transforms: Vec<Box<dyn Transform>>,
    buffer1: Vec<Entity>,
    buffer2: Vec<Entity>,
}

impl Transform for SerialTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        let entities = &mut self.buffer1;
        let temp_entities = &mut self.buffer2;
        entities.clear();
        temp_entities.clear();

        entities.push(entity);
        for transform in self.transforms.iter_mut() {
            for entity in entities.drain(..) {
                transform.transform(entity, temp_entities);
            }
            std::mem::swap(entities, temp_entities);
        }
        out.append(entities);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for transform in &self.transforms {
            transform.transform_schema(schema)
        }
    }
}

impl SerialTransform {
    pub fn push(&mut self, transform: Box<dyn Transform>) {
        self.transforms.push(transform);
    }
}

/// No-op transform
#[derive(Clone)]
pub struct IdentityTransform {}

impl Transform for IdentityTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        out.push(entity);
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use nusamai_citygml::object::{Object, Value};
    use nusamai_citygml::GeometryStore;

    use super::*;
    #[test]
    fn test_serial_transform() {
        let mut transform = SerialTransform::default();
        transform.push(Box::new(IdentityTransform {}));
        transform.push(Box::new(IdentityTransform {}));
        transform.push(Box::new(IdentityTransform {}));
        let mut entities = Vec::new();
        transform.transform(
            Entity {
                root: Value::Object(Object {
                    typename: "test".into(),
                    attributes: Default::default(),
                    stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                        id: "foobar".into(),
                        geometries: Default::default(),
                    },
                }),
                geometry_store: RwLock::new(GeometryStore::default()).into(),
                appearance_store: Default::default(),
            },
            &mut entities,
        );
        assert_eq!(entities.len(), 1);
    }
}
