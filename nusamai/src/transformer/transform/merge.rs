use crate::transformer::Transform;

use hashbrown::HashSet;
use nusamai_citygml::object::{Map, Object, ObjectStereotype, Value};
use nusamai_citygml::schema::{Schema, TypeDef};
use nusamai_citygml::GeometryRef;
use nusamai_plateau::Entity;

/// Collect all attributes and geometries from the descendants and merge them into the root object.
#[derive(Default, Clone)]
pub struct FullMergedownTransform {
    geoms_buf: HashSet<GeometryRef>,
    path_buf: String,
}

impl Transform for FullMergedownTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(obj) = entity.root {
            let mut new_attrs = Default::default();
            let new_geoms = &mut self.geoms_buf;
            let path = &mut self.path_buf;
            path.clear();

            collect_all_attrs_and_geoms(&mut new_attrs, new_geoms, path, obj.attributes, false);

            if let ObjectStereotype::Feature { id, geometries } = obj.stereotype {
                new_geoms.extend(geometries);
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

fn collect_all_attrs_and_geoms(
    new_attrs: &mut Map,
    new_geoms: &mut HashSet<GeometryRef>,
    path: &mut String,
    attributes: Map,
    in_child_feature: bool,
) {
    for (key, value) in attributes {
        let path_len = path.len();
        path.push_str(&key);
        path.push('.');
        match value {
            Value::Object(obj) => {
                let in_child_feature =
                    if let ObjectStereotype::Feature { geometries, .. } = obj.stereotype {
                        new_geoms.extend(geometries);
                        true
                    } else {
                        in_child_feature
                    };
                collect_all_attrs_and_geoms(
                    new_attrs,
                    new_geoms,
                    path,
                    obj.attributes,
                    in_child_feature,
                );
            }
            Value::Array(arr) => {
                for (i, value) in arr.into_iter().enumerate() {
                    let len = path.len();
                    path.push_str(&format!("{i}"));
                    path.push('.');
                    match value {
                        Value::Object(obj) => {
                            let in_child_feature = if let ObjectStereotype::Feature {
                                geometries,
                                ..
                            } = obj.stereotype
                            {
                                new_geoms.extend(geometries);
                                true
                            } else {
                                in_child_feature
                            };
                            collect_all_attrs_and_geoms(
                                new_attrs,
                                new_geoms,
                                path,
                                obj.attributes,
                                in_child_feature,
                            );
                        }
                        _ => {
                            new_attrs.insert(path.clone(), value);
                        }
                    }
                    path.truncate(len);
                }
            }
            _ => {
                if !in_child_feature {
                    new_attrs.insert(path[..path.len() - 1].to_string(), value);
                }
            }
        }
        path.truncate(path_len);
    }
}

#[derive(Default, Clone)]
pub struct GeometricMergedownTransform {
    geoms_buf: HashSet<GeometryRef>,
}

impl Transform for GeometricMergedownTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(obj) = &mut entity.root {
            let new_geoms = &mut self.geoms_buf;
            collect_all_geoms(new_geoms, obj);
            if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
                *geometries = new_geoms.drain().collect();
            }
            out.push(entity);
        }
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

fn collect_all_geoms(new_geoms: &mut HashSet<GeometryRef>, obj: &mut Object) {
    if let ObjectStereotype::Feature { geometries, .. } = &mut obj.stereotype {
        new_geoms.extend(geometries.drain(..));
    }
    for value in obj.attributes.values_mut() {
        match value {
            Value::Object(obj) => {
                collect_all_geoms(new_geoms, obj);
            }
            Value::Array(arr) => {
                for value in arr {
                    if let Value::Object(obj) = value {
                        collect_all_geoms(new_geoms, obj);
                    }
                }
            }
            _ => {}
        }
    }
}
