use std::sync::{Arc, RwLock};

use crate::transformer::Transform;

use nusamai_citygml::object::{Map, Object, ObjectStereotype, Value};
use nusamai_citygml::schema::{Attribute, Schema, TypeDef, TypeRef};
use nusamai_citygml::GeometryStore;
use nusamai_plateau::appearance::AppearanceStore;
use nusamai_plateau::Entity;

pub struct FlattenTreeTransform {
    feature: FeatureFlatteningOption,
    data: DataFlatteningOption,
    object: ObjectFlatteningOption,
}

/// Flattening option for the "feature" stereotype
// TODO: Use this to implement flattening process
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
// TODO: Use this to implement flattening process
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
// TODO: Use this to implement flattening process
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
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        let geom_store = entity.geometry_store;
        let appearance_store = entity.appearance_store;
        self.flatten_entity(entity.root, &geom_store, &appearance_store, out, &None);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for ty in schema.types.values_mut() {
            match ty {
                TypeDef::Feature(typedef) => {
                    typedef.attributes.insert(
                        "parentId".into(),
                        Attribute {
                            type_ref: TypeRef::String,
                            min_occurs: 0,
                            max_occurs: Some(1),
                        },
                    );
                    typedef.attributes.insert(
                        "parentType".into(),
                        Attribute {
                            type_ref: TypeRef::String,
                            min_occurs: 0,
                            max_occurs: Some(1),
                        },
                    );
                }
                TypeDef::Data(typedef) => {
                    typedef.attributes.insert(
                        "parentId".into(),
                        Attribute {
                            type_ref: TypeRef::String,
                            min_occurs: 0,
                            max_occurs: Some(1),
                        },
                    );
                    typedef.attributes.insert(
                        "parentType".into(),
                        Attribute {
                            type_ref: TypeRef::String,
                            min_occurs: 0,
                            max_occurs: Some(1),
                        },
                    );
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
