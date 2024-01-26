use crate::transformer::Transform;

use hashbrown::HashSet;
use nusamai_citygml::object::{Entity, Map, Object, ObjectStereotype, Value};
use nusamai_citygml::schema::{Schema, TypeDef};
use nusamai_citygml::GeometryRefEntry;

/// Collect all attributes and geometries from the descendants and merge them into the root object.
#[derive(Default, Clone)]
pub struct FullMergeTransform {
    geoms_buf: HashSet<GeometryRefEntry>,
}

impl Transform for FullMergeTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(obj) = entity.root {
            if let ObjectStereotype::Feature { id, .. } = obj.stereotype {
                let mut new_attrs = Default::default();
                let new_geoms = &mut self.geoms_buf;
                let mut path = String::with_capacity(50);
                full_merge_traverse(&mut new_attrs, new_geoms, &mut path, obj.attributes);
                entity.root = Value::Object(Object {
                    typename: obj.typename,
                    attributes: new_attrs,
                    stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                        id,
                        geometries: new_geoms.drain().collect(),
                    },
                });
                out.push(entity);
            }
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

fn full_merge_traverse(
    new_attrs: &mut Map,
    new_geoms: &mut HashSet<GeometryRefEntry>,
    path: &mut String,
    attributes: Map,
) {
    for (key, value) in attributes {
        let path_len = path.len();
        path.push_str(&key);
        path.push('.');
        match value {
            Value::Object(obj) => {
                if let ObjectStereotype::Feature { geometries, .. } = obj.stereotype {
                    new_geoms.extend(geometries);
                }
                full_merge_traverse(new_attrs, new_geoms, path, obj.attributes);
            }
            Value::Array(arr) => {
                for (i, value) in arr.into_iter().enumerate() {
                    let len = path.len();
                    path.push_str(&format!("{i}"));
                    path.push('.');
                    match value {
                        Value::Object(obj) => {
                            full_merge_traverse(new_attrs, new_geoms, path, obj.attributes);
                        }
                        _ => {
                            new_attrs.insert(path.clone(), value);
                        }
                    }
                    path.truncate(len);
                }
            }
            _ => {
                new_attrs.insert(path[..path.len() - 1].to_string(), value);
            }
        }
        path.truncate(path_len);
    }
}

#[derive(Default, Clone)]
pub struct GeometryMergeTransform {
    geoms_buf: HashSet<GeometryRefEntry>,
}

impl Transform for GeometryMergeTransform {
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
