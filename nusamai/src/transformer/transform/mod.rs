mod projection;

pub use projection::*;

use super::Transform;
use nusamai_citygml::{object::Entity, schema::Schema};

// Perform transforms in sequence
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
        entities.push(entity);
        temp_entities.clear();

        for transform in self.transforms.iter_mut() {
            temp_entities.clear();
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
pub struct IdentityTransform {}

impl Transform for IdentityTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        out.push(entity);
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}
