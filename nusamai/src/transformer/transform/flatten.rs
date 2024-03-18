use std::sync::{Arc, RwLock};

use nusamai_citygml::{
    object::{Map, Object, ObjectStereotype, Value},
    schema::{Attribute, Schema, TypeDef, TypeRef},
    GeometryStore,
};
use nusamai_plateau::{appearance::AppearanceStore, Entity};

use crate::{pipeline::Feedback, transformer::Transform};

pub struct FlattenTreeTransform {
    feature: FeatureFlatteningOption,
    data: DataFlatteningOption,
    object: ObjectFlatteningOption,
}

/// Flattening option for the "feature" stereotype
#[derive(Debug, Copy, Clone)]
pub enum FeatureFlatteningOption {
    /// No feature flattening
    None,
    /// Flatten all features except thematic surfaces
    AllExceptThematicSurfaces,
    /// Flatten all features
    All,
}

/// Flattening option for the "data" stereotype
#[derive(Debug, Copy, Clone)]
pub enum DataFlatteningOption {
    /// No data flattening
    None,
    /// Flatten top-level data (i.e., data that is not a child of another data)
    TopLevelOnly,
    /// Flatten all data
    All,
}

/// Flattening option for the "object" stereotype
#[derive(Debug, Copy, Clone)]
pub enum ObjectFlatteningOption {
    /// No object flattening
    None,
    /// Flatten all objects
    All,
}

impl Default for FlattenTreeTransform {
    fn default() -> Self {
        Self {
            feature: FeatureFlatteningOption::None,
            data: DataFlatteningOption::None,
            object: ObjectFlatteningOption::None,
        }
    }
}

impl FlattenTreeTransform {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_options(
        feature: FeatureFlatteningOption,
        data: DataFlatteningOption,
        object: ObjectFlatteningOption,
    ) -> Self {
        Self {
            feature,
            data,
            object,
        }
    }

    pub fn set_feature_option(&mut self, option: FeatureFlatteningOption) {
        self.feature = option;
    }

    pub fn set_data_option(&mut self, option: DataFlatteningOption) {
        self.data = option;
    }

    pub fn set_object_option(&mut self, option: ObjectFlatteningOption) {
        self.object = option;
    }
}

impl Transform for FlattenTreeTransform {
    fn transform(&mut self, _feedback: &Feedback, entity: Entity, out: &mut Vec<Entity>) {
        let geom_store = entity.geometry_store;
        let appearance_store = entity.appearance_store;
        self.flatten_entity(entity.root, &geom_store, &appearance_store, out, &None);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for ty in schema.types.values_mut() {
            match ty {
                TypeDef::Feature(typedef) => {
                    if !matches!(self.feature, FeatureFlatteningOption::None) {
                        typedef.attributes.insert(
                            "parentId".into(),
                            Attribute {
                                type_ref: TypeRef::String,
                                min_occurs: 0,
                                max_occurs: Some(1),
                                original_name: None,
                            },
                        );
                        typedef.attributes.insert(
                            "parentType".into(),
                            Attribute {
                                type_ref: TypeRef::String,
                                min_occurs: 0,
                                max_occurs: Some(1),
                                original_name: None,
                            },
                        );
                    }
                }
                TypeDef::Data(typedef) => {
                    if !matches!(self.data, DataFlatteningOption::None) {
                        typedef.attributes.insert(
                            "parentId".into(),
                            Attribute {
                                type_ref: TypeRef::String,
                                min_occurs: 0,
                                max_occurs: Some(1),
                                original_name: None,
                            },
                        );
                        typedef.attributes.insert(
                            "parentType".into(),
                            Attribute {
                                type_ref: TypeRef::String,
                                min_occurs: 0,
                                max_occurs: Some(1),
                                original_name: None,
                            },
                        );
                    }
                }
                TypeDef::Property(_) => {
                    // Do nothing
                }
            }
        }
    }
}

enum Parent {
    Feature { id: String, typename: String },
    Data { typename: String }, // Data stereotype does not have an id
    Object { id: String, typename: String },
}

impl FlattenTreeTransform {
    fn flatten_entity(
        &self,
        value: Value,
        geom_store: &Arc<RwLock<GeometryStore>>,
        appearance_store: &Arc<RwLock<AppearanceStore>>,
        out: &mut Vec<Entity>,
        parent: &Option<Parent>,
    ) -> Option<Value> {
        match value {
            Value::Object(mut obj) => {
                let new_parent = match &obj.stereotype {
                    ObjectStereotype::Feature { id, .. } => Some(Parent::Feature {
                        id: id.to_string(),
                        typename: obj.typename.to_string(),
                    }),
                    ObjectStereotype::Data => Some(Parent::Data {
                        typename: obj.typename.to_string(),
                    }),
                    ObjectStereotype::Object { id, .. } => Some(Parent::Object {
                        id: id.to_string(),
                        typename: obj.typename.to_string(),
                    }),
                };

                // Attributes
                let mut new_attribs = Map::default();
                for (key, value) in obj.attributes.drain(..) {
                    if let Some(v) =
                        self.flatten_entity(value, geom_store, appearance_store, out, &new_parent)
                    {
                        new_attribs.insert(key, v);
                    }
                }
                obj.attributes = new_attribs;

                if self.is_flatten_target(&obj, parent) {
                    // set parent id and type to attributes
                    if let Some(parent) = parent {
                        match parent {
                            Parent::Feature { id, typename } => {
                                obj.attributes
                                    .insert("parentId".to_string(), Value::String(id.to_string()));
                                obj.attributes.insert(
                                    "parentType".to_string(),
                                    Value::String(typename.to_string()),
                                );
                            }
                            Parent::Data { typename } => {
                                obj.attributes.insert(
                                    "parentType".to_string(),
                                    Value::String(typename.to_string()),
                                );
                            }
                            Parent::Object { id, typename } => {
                                obj.attributes
                                    .insert("parentId".to_string(), Value::String(id.to_string()));
                                obj.attributes.insert(
                                    "parentType".to_string(),
                                    Value::String(typename.to_string()),
                                );
                            }
                        }
                    }
                    out.push(Entity {
                        root: Value::Object(obj),
                        base_url: url::Url::parse("file:///dummy").expect("should be valid"),
                        geometry_store: geom_store.clone(),
                        appearance_store: appearance_store.clone(),
                    });
                    return None;
                }

                Some(Value::Object(obj))
            }
            Value::Array(mut arr) => {
                let mut new_arr = Vec::with_capacity(arr.len());
                for value in arr.drain(..) {
                    if let Some(v) =
                        self.flatten_entity(value, geom_store, appearance_store, out, parent)
                    {
                        new_arr.push(v)
                    }
                }
                if new_arr.is_empty() {
                    None
                } else {
                    Some(Value::Array(new_arr))
                }
            }
            _ => Some(value),
        }
    }

    fn is_flatten_target(&self, obj: &Object, parent: &Option<Parent>) -> bool {
        // Do not flattten generic attributes:
        // It may hold any arbitrary attributes, therefore you cannot have schema information about it in advance.
        // (In schema, generic attribute has `additional_attributes = true`)
        // This is problematic for the GeoPackage sink, as it requires schema information to create a table.
        if obj.typename == "gen:genericAttribute" {
            return false;
        }

        match obj.stereotype {
            ObjectStereotype::Feature { .. } => match self.feature {
                FeatureFlatteningOption::None => false,
                FeatureFlatteningOption::All => true,
                FeatureFlatteningOption::AllExceptThematicSurfaces => {
                    !obj.typename.ends_with("Surface")
                        && !obj.typename.ends_with(":Window")
                        && !obj.typename.ends_with(":Door")
                        && !obj.typename.ends_with("TrafficArea")
                }
            },
            ObjectStereotype::Data => match self.data {
                DataFlatteningOption::None => false,
                DataFlatteningOption::TopLevelOnly => {
                    if let Some(parent) = parent {
                        // If the parent is Data, do not flatten
                        !matches!(parent, Parent::Data { .. })
                    } else {
                        true
                    }
                }
                DataFlatteningOption::All => true,
            },
            ObjectStereotype::Object { .. } => match self.object {
                ObjectFlatteningOption::None => false,
                ObjectFlatteningOption::All => true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_entity_feature() {
        // Prepare test entity hierarchy
        let mut attributes = Map::default();
        attributes.insert("attr_1".into(), Value::String("value1".into()));
        attributes.insert(
            "child_feat_1".into(),
            Value::Object(Object {
                typename: "child_feat_1".into(),
                stereotype: ObjectStereotype::Feature {
                    id: "child_feat_1_id".into(),
                    geometries: Vec::default(),
                },
                attributes: Map::default(),
            }),
        );
        let root: Value = Value::Object(Object {
            typename: "root".into(),
            stereotype: ObjectStereotype::Feature {
                id: "root_id".into(),
                geometries: Vec::default(),
            },
            attributes,
        });

        // Apply flatten transform with the options
        let transform = FlattenTreeTransform {
            feature: FeatureFlatteningOption::All,
            data: DataFlatteningOption::None,
            object: ObjectFlatteningOption::None,
        };
        let geom_store = Arc::new(RwLock::new(GeometryStore::default()));
        let appearance_store = Arc::new(RwLock::new(AppearanceStore::default()));
        let mut out: Vec<Entity> = vec![];
        transform.flatten_entity(root, &geom_store, &appearance_store, &mut out, &None);

        // Check the result
        assert_eq!(out.len(), 2);
        out.iter().enumerate().for_each(|(i, entity)| match i {
            0 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "child_feat_1");
                    assert_eq!(obj.attributes.len(), 2); // 2 items (parentId, parentType) added
                    assert_eq!(
                        obj.attributes.get("parentId").unwrap(),
                        &Value::String("root_id".into())
                    );
                    assert_eq!(
                        obj.attributes.get("parentType").unwrap(),
                        &Value::String("root".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            1 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "root");
                    assert_eq!(obj.attributes.len(), 1); // child_feature_1 is removed
                    assert_eq!(
                        obj.attributes.get("attr_1").unwrap(),
                        &Value::String("value1".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            _ => panic!("Unexpected entity"),
        });
    }

    #[test]
    fn test_flatten_entity_feature_and_data() {
        // Prepare test entity hierarchy
        let mut attributes = Map::default();
        attributes.insert("attr_1".into(), Value::String("value1".into()));
        attributes.insert(
            "child_feat_1".into(),
            Value::Object(Object {
                typename: "child_feat_1".into(),
                stereotype: ObjectStereotype::Feature {
                    id: "child_feat_1_id".into(),
                    geometries: Vec::default(),
                },
                attributes: Map::default(),
            }),
        );
        attributes.insert(
            "child_data_1".into(),
            Value::Object(Object {
                typename: "child_data_1".into(),
                stereotype: ObjectStereotype::Data, // No id
                attributes: Map::default(),
            }),
        );
        let root: Value = Value::Object(Object {
            typename: "root".into(),
            stereotype: ObjectStereotype::Feature {
                id: "root_id".into(),
                geometries: Vec::default(),
            },
            attributes,
        });

        // Apply flatten transform with the options
        let transform = FlattenTreeTransform {
            feature: FeatureFlatteningOption::All,
            data: DataFlatteningOption::All, // flatten data as well
            object: ObjectFlatteningOption::None,
        };
        let geom_store = Arc::new(RwLock::new(GeometryStore::default()));
        let appearance_store = Arc::new(RwLock::new(AppearanceStore::default()));
        let mut out: Vec<Entity> = vec![];
        transform.flatten_entity(root, &geom_store, &appearance_store, &mut out, &None);

        // Check the result
        assert_eq!(out.len(), 3);
        out.iter().enumerate().for_each(|(i, entity)| match i {
            0 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "child_feat_1"); // feature child
                    assert_eq!(obj.attributes.len(), 2); // 2 items (parentId, parentType) added
                    assert_eq!(
                        obj.attributes.get("parentId").unwrap(),
                        &Value::String("root_id".into())
                    );
                    assert_eq!(
                        obj.attributes.get("parentType").unwrap(),
                        &Value::String("root".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            1 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "child_data_1"); // data child
                    assert_eq!(obj.attributes.len(), 2); // 2 items (parentId, parentType) added
                    assert_eq!(
                        obj.attributes.get("parentId").unwrap(),
                        &Value::String("root_id".into())
                    );
                    assert_eq!(
                        obj.attributes.get("parentType").unwrap(),
                        &Value::String("root".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            2 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "root");
                    assert_eq!(obj.attributes.len(), 1); // child_feature_1 is removed
                    assert_eq!(
                        obj.attributes.get("attr_1").unwrap(),
                        &Value::String("value1".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            _ => panic!("Unexpected entity"),
        });
    }

    #[test]
    fn test_flatten_entity_nested_features() {
        // Prepare test entity hierarchy
        let mut attributes_child = Map::default();
        attributes_child.insert("child_attr_1".into(), Value::String("value1".into()));
        attributes_child.insert(
            "grandchild_feat_1".into(),
            Value::Object(Object {
                typename: "grandchild_feat_1".into(),
                stereotype: ObjectStereotype::Feature {
                    id: "grandchild_feat_1_id".into(),
                    geometries: Vec::default(),
                },
                attributes: Map::default(),
            }),
        );
        let mut attributes_root = Map::default();
        attributes_root.insert("attr_1".into(), Value::String("value1".into()));
        attributes_root.insert(
            "child_feat_1".into(),
            Value::Object(Object {
                typename: "child_feat_1".into(),
                stereotype: ObjectStereotype::Feature {
                    id: "child_feat_1_id".into(),
                    geometries: Vec::default(),
                },
                attributes: attributes_child,
            }),
        );
        let root: Value = Value::Object(Object {
            typename: "root".into(),
            stereotype: ObjectStereotype::Feature {
                id: "root_id".into(),
                geometries: Vec::default(),
            },
            attributes: attributes_root,
        });

        // Apply flatten transform with the options
        let transform = FlattenTreeTransform {
            feature: FeatureFlatteningOption::All,
            data: DataFlatteningOption::None,
            object: ObjectFlatteningOption::None,
        };
        let geom_store = Arc::new(RwLock::new(GeometryStore::default()));
        let appearance_store = Arc::new(RwLock::new(AppearanceStore::default()));
        let mut out: Vec<Entity> = vec![];
        transform.flatten_entity(root, &geom_store, &appearance_store, &mut out, &None);

        // Check the result
        assert_eq!(out.len(), 3);
        out.iter().enumerate().for_each(|(i, entity)| match i {
            0 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "grandchild_feat_1");
                    assert_eq!(obj.attributes.len(), 2); // 2 items (parentId, parentType) added
                    assert_eq!(
                        obj.attributes.get("parentId").unwrap(),
                        &Value::String("child_feat_1_id".into())
                    );
                    assert_eq!(
                        obj.attributes.get("parentType").unwrap(),
                        &Value::String("child_feat_1".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            1 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "child_feat_1");
                    assert_eq!(obj.attributes.len(), 3); // 2 items (parentId, parentType) added
                    assert_eq!(
                        obj.attributes.get("child_attr_1").unwrap(),
                        &Value::String("value1".into())
                    );
                    assert_eq!(
                        obj.attributes.get("parentId").unwrap(),
                        &Value::String("root_id".into())
                    );
                    assert_eq!(
                        obj.attributes.get("parentType").unwrap(),
                        &Value::String("root".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            2 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "root");
                    assert_eq!(obj.attributes.len(), 1); // child_feature_1 is removed
                    assert_eq!(
                        obj.attributes.get("attr_1").unwrap(),
                        &Value::String("value1".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            _ => panic!("Unexpected entity"),
        });
    }

    #[test]
    fn test_flatten_entity_nested_data_toplevel_only() {
        // Prepare test entity hierarchy
        let mut attributes_child = Map::default();
        attributes_child.insert("child_attr_1".into(), Value::String("value1".into()));
        attributes_child.insert(
            "grandchild_data_1".into(),
            Value::Object(Object {
                typename: "grandchild_data_1".into(),
                stereotype: ObjectStereotype::Data,
                attributes: Map::default(),
            }),
        );
        let mut attributes_root = Map::default();
        attributes_root.insert("attr_1".into(), Value::String("value1".into()));
        attributes_root.insert(
            "child_data_1".into(),
            Value::Object(Object {
                typename: "child_data_1".into(),
                stereotype: ObjectStereotype::Data,
                attributes: attributes_child,
            }),
        );
        let root: Value = Value::Object(Object {
            typename: "root".into(),
            stereotype: ObjectStereotype::Feature {
                id: "root_id".into(),
                geometries: Vec::default(),
            },
            attributes: attributes_root,
        });

        // Apply flatten transform with the options
        let transform = FlattenTreeTransform {
            feature: FeatureFlatteningOption::All,
            data: DataFlatteningOption::TopLevelOnly, // flatten data, but only top-level
            object: ObjectFlatteningOption::None,
        };
        let geom_store = Arc::new(RwLock::new(GeometryStore::default()));
        let appearance_store = Arc::new(RwLock::new(AppearanceStore::default()));
        let mut out: Vec<Entity> = vec![];
        transform.flatten_entity(root, &geom_store, &appearance_store, &mut out, &None);

        // Check the result
        assert_eq!(out.len(), 2); // not 3, as the grand child is not considered
        out.iter().enumerate().for_each(|(i, entity)| match i {
            0 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "child_data_1");
                    assert_eq!(obj.attributes.len(), 4); // 2 items (parentId, parentType) added
                    assert_eq!(
                        obj.attributes.get("child_attr_1").unwrap(),
                        &Value::String("value1".into())
                    );
                    // this is not flattened
                    assert_eq!(
                        obj.attributes.get("grandchild_data_1").unwrap(),
                        &Value::Object(Object {
                            typename: "grandchild_data_1".into(),
                            stereotype: ObjectStereotype::Data,
                            attributes: Map::default(),
                        })
                    );
                    assert_eq!(
                        obj.attributes.get("parentId").unwrap(),
                        &Value::String("root_id".into())
                    );
                    assert_eq!(
                        obj.attributes.get("parentType").unwrap(),
                        &Value::String("root".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            1 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "root");
                    assert_eq!(obj.attributes.len(), 1); // child_feature_1 is removed
                    assert_eq!(
                        obj.attributes.get("attr_1").unwrap(),
                        &Value::String("value1".into())
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            _ => panic!("Unexpected entity"),
        });
    }

    #[test]
    fn test_flatten_entity_generic_attribute() {
        // Prepare test entity hierarchy
        let mut attributes_root = Map::default();
        attributes_root.insert("attr_1".into(), Value::String("value1".into()));
        attributes_root.insert(
            "child_data_1".into(),
            Value::Object(Object {
                typename: "gen:genericAttribute".into(),
                stereotype: ObjectStereotype::Data,
                attributes: Map::default(),
            }),
        );
        attributes_root.insert(
            "child_data_2".into(),
            Value::Object(Object {
                typename: "child_data_2".into(),
                stereotype: ObjectStereotype::Data,
                attributes: Map::default(),
            }),
        );
        let root: Value = Value::Object(Object {
            typename: "root".into(),
            stereotype: ObjectStereotype::Feature {
                id: "root_id".into(),
                geometries: Vec::default(),
            },
            attributes: attributes_root,
        });

        // Apply flatten transform with the options
        let transform = FlattenTreeTransform {
            feature: FeatureFlatteningOption::All,
            data: DataFlatteningOption::TopLevelOnly, // flatten data, but only top-level
            object: ObjectFlatteningOption::None,
        };
        let geom_store = Arc::new(RwLock::new(GeometryStore::default()));
        let appearance_store = Arc::new(RwLock::new(AppearanceStore::default()));
        let mut out: Vec<Entity> = vec![];
        transform.flatten_entity(root, &geom_store, &appearance_store, &mut out, &None);

        // Check the result
        assert_eq!(out.len(), 2); // a "generic attribute" is not flattened
        out.iter().enumerate().for_each(|(i, entity)| match i {
            0 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "child_data_2");
                    assert_eq!(obj.attributes.len(), 2);
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            1 => {
                if let Value::Object(obj) = &entity.root {
                    assert_eq!(obj.typename, "root");
                    assert_eq!(obj.attributes.len(), 2);
                    // this child is not flattened
                    assert_eq!(
                        obj.attributes.get("child_data_1").unwrap(),
                        &Value::Object(Object {
                            typename: "gen:genericAttribute".into(),
                            stereotype: ObjectStereotype::Data,
                            attributes: Map::default(),
                        })
                    );
                } else {
                    panic!("Expected Value::Object, but got {:?}", entity.root);
                }
            }
            _ => panic!("Unexpected entity"),
        });
    }
}
