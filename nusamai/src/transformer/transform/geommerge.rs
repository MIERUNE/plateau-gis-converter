use hashbrown::HashSet;
use nusamai_citygml::{
    object::{Object, ObjectStereotype, Value},
    schema::Schema,
    GeometryRef,
};
use nusamai_plateau::Entity;

use crate::{pipeline::Feedback, transformer::Transform};

#[derive(Default, Clone)]
pub struct GeometricMergedownTransform {
    geoms_buf: HashSet<GeometryRef>,
    /// If false, all descendant features will be removed after merging geometries.
    retain_descendant_features: bool,
}

impl GeometricMergedownTransform {
    pub fn new(retain_descendant_features: bool) -> Self {
        Self {
            retain_descendant_features,
            ..Default::default()
        }
    }
}

impl Transform for GeometricMergedownTransform {
    fn transform(&mut self, _feedback: &Feedback, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(obj) = &mut entity.root {
            self.collect_all_geoms(obj);
            if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
                *geometries = self.geoms_buf.drain().collect();
            }
            out.push(entity);
        }
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

impl GeometricMergedownTransform {
    fn collect_all_geoms(&mut self, obj: &mut Object) -> bool {
        let mut is_feature = false;
        if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
            is_feature = true;
            self.geoms_buf.extend(geometries.drain(..));
        }

        obj.attributes.retain(|_key, value| match value {
            Value::Object(obj) => self.retain_descendant_features || !self.collect_all_geoms(obj),
            Value::Array(arr) => {
                arr.retain_mut(|value| {
                    if let Value::Object(obj) = value {
                        self.retain_descendant_features || !self.collect_all_geoms(obj)
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
