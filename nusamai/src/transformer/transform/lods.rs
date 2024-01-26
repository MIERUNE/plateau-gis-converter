use crate::transformer::Transform;

use hashbrown::HashSet;
use nusamai_citygml::object::{Entity, Map, ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryRefEntry;

#[derive(Default, Clone)]
pub struct LodFilteringTransform {
    geoms_buf: HashSet<GeometryRefEntry>,
}

impl Transform for LodFilteringTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(obj) = &mut entity.root {
            if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
                let new_geoms = &mut self.geoms_buf;
                geometry_merge_traverse(new_geoms, &mut obj.attributes);
                *geometries = new_geoms.drain().collect();
            }
            out.push(entity);
        }
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

fn geometry_merge_traverse(new_geoms: &mut HashSet<GeometryRefEntry>, attributes: &mut Map) {
    for value in attributes.values_mut() {
        match value {
            Value::Object(obj) => {
                if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
                    new_geoms.extend(geometries.drain(..));
                }
                geometry_merge_traverse(new_geoms, &mut obj.attributes);
            }
            Value::Array(arr) => {
                for value in arr {
                    if let Value::Object(obj) = value {
                        geometry_merge_traverse(new_geoms, &mut obj.attributes);
                    }
                }
            }
            _ => {}
        }
    }
}
