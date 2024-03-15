mod appearance;
mod attrname;
mod dots;
pub mod flatten;
mod geommerge;
mod geomstats;
mod jsonify;
mod lods;
mod projection;

pub use appearance::*;
pub use attrname::*;
pub use dots::*;
pub use flatten::*;
pub use geommerge::*;
pub use geomstats::*;
pub use jsonify::*;
pub use lods::*;
use nusamai_citygml::schema::Schema;
use nusamai_plateau::Entity;
pub use projection::*;

use super::Transform;
use crate::pipeline::Feedback;

/// Perform transforms in sequence
#[derive(Default)]
pub struct SerialTransform {
    transforms: Vec<Box<dyn Transform>>,
    buffer1: Vec<Entity>,
    buffer2: Vec<Entity>,
}

impl Transform for SerialTransform {
    fn transform(&mut self, feedback: &Feedback, entity: Entity, out: &mut Vec<Entity>) {
        let entities = &mut self.buffer1;
        let temp_entities = &mut self.buffer2;
        entities.clear();
        temp_entities.clear();

        entities.push(entity);
        for transform in self.transforms.iter_mut() {
            for entity in entities.drain(..) {
                transform.transform(feedback, entity, temp_entities);
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
    fn transform(&mut self, _feedback: &Feedback, entity: Entity, out: &mut Vec<Entity>) {
        out.push(entity);
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use feedback::watcher;
    use nusamai_citygml::{object::Object, GeometryStore, Value};

    use super::*;
    use crate::pipeline::feedback;

    #[test]
    fn test_serial_transform() {
        let (_watcher, feedback, _canceller) = watcher();

        let mut transform = SerialTransform::default();
        transform.push(Box::new(IdentityTransform {}));
        transform.push(Box::new(IdentityTransform {}));
        transform.push(Box::new(IdentityTransform {}));
        let mut entities = Vec::new();
        transform.transform(
            &feedback,
            Entity {
                root: Value::Object(Object {
                    typename: "test".into(),
                    attributes: Default::default(),
                    stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                        id: "foobar".into(),
                        geometries: Default::default(),
                    },
                }),
                base_url: url::Url::parse("file:///dummy").unwrap(),
                geometry_store: RwLock::new(GeometryStore::default()).into(),
                appearance_store: Default::default(),
            },
            &mut entities,
        );
        assert_eq!(entities.len(), 1);
    }
}
