use hashbrown::HashMap;
use indexmap::IndexMap;
use nusamai_citygml::{
    object::{Map, Value},
    schema,
    schema::{Schema, TypeDef},
};
use nusamai_plateau::Entity;

use crate::{pipeline::Feedback, transformer::Transform};

/// Transform to edit field names
///
/// The current implementation performs the following operations:
///
/// - Remove the namespace prefix from the field names (e.g., `"ns:foo"` -> `"foo"`)
/// - Rename the field names for Shapefile according to the dictionary (when the option is enabled)
/// - Rename the field names given the rules by the user
///
/// You may specify the rules in two ways:
/// - Exact match: Rename if the key matches exactly (e.g., `{"ns:foo": "bar"}`)
/// - General match: Rename for any namespace prefix (e.g., `{"*:foo": "bar"}`)
///   Note that the exact match takes precedence over the general match.
#[derive(Default, Clone)]
pub struct EditFieldNamesTransform {
    // Exact string match dictionary
    exact_rename_map: HashMap<String, String>,
    // general suffix match dictionary - the stored keys are the string after the prefix "*:"
    general_rename_map: HashMap<String, String>,
}

impl EditFieldNamesTransform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_default_map_for_shape(&mut self) {
        const SHAPE_DICT: &str = include_str!("./shp_field_dict.json");
        let map: HashMap<String, String> =
            serde_json::from_str(SHAPE_DICT).expect("should be valid");
        // This applies to field names with any namespace prefix (general match)
        self.general_rename_map.extend(map);
        for value in self.general_rename_map.values() {
            if value.len() > 10 {
                panic!("The key length must be less than 10 characters: {}", value);
            }
        }
    }

    pub fn extend_rename_map(&mut self, map: HashMap<String, String>) {
        for (before, after) in map {
            if let Some(before_stripped) = before.strip_prefix("*:") {
                self.general_rename_map
                    .insert(before_stripped.into(), after);
            } else {
                self.exact_rename_map.insert(before, after);
            }
        }
    }
}

impl Transform for EditFieldNamesTransform {
    fn transform(&mut self, _feedback: &Feedback, mut entity: Entity, out: &mut Vec<Entity>) {
        self.edit_tree(&mut entity.root);
        out.push(entity);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        let drain_to_new_attrs = |attrs: &mut schema::Map| {
            let mut new_attrs = IndexMap::default();
            for (key, mut value) in attrs.drain(..) {
                let new_name = self.rename(&key);
                value.original_name = Some(key.clone());
                new_attrs.insert(new_name.to_string(), value);
            }
            new_attrs
        };

        for ty in schema.types.values_mut() {
            match ty {
                TypeDef::Data(data) => {
                    data.attributes = drain_to_new_attrs(&mut data.attributes);
                }
                TypeDef::Feature(feat) => {
                    feat.attributes = drain_to_new_attrs(&mut feat.attributes);
                }
                TypeDef::Property(_) => continue,
            };
        }
    }
}

impl EditFieldNamesTransform {
    fn rename<'a>(&'a self, name: &'a str) -> &'a str {
        // Lookup and rename: exact match
        if let Some(new_key) = self.exact_rename_map.get(name) {
            return new_key.as_ref();
        }

        name.find(':')
            .map(|pos| {
                let key = &name[pos + 1..]; // remove the namespace prefix

                if let Some(new_key) = self.general_rename_map.get(key) {
                    return new_key.as_ref();
                }

                // If the namespace is removed, it will conflict with the global "id" column (= "gml:id").
                // Therefore, don't remove the namespace prefix
                if key == "id" {
                    return name;
                }

                key
            })
            .unwrap_or(name)
    }

    fn edit_tree(&self, value: &mut Value) {
        match value {
            Value::Object(obj) => {
                let mut new_attrs = Map::default();
                for (key, mut value) in obj.attributes.drain(..) {
                    self.edit_tree(&mut value);
                    let new_name = self.rename(&key);
                    new_attrs.insert(new_name.to_string(), value);
                }
                obj.attributes = new_attrs;
            }
            Value::Array(arr) => {
                for v in arr.iter_mut() {
                    self.edit_tree(v);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename() {
        let mut transform = EditFieldNamesTransform::new();
        let mut map = HashMap::new();

        map.insert("*:class".to_string(), "分類".to_string());
        map.insert("luse:class".to_string(), "土地利用区分".to_string());
        map.insert("wo_namespace".to_string(), "wo_namespace_new".to_string());
        transform.extend_rename_map(map);

        // In any case, namespace suffix is removed
        assert_eq!(transform.rename("namespace:foo"), "foo");

        // Rule written with specific namespace takes precedence
        assert_eq!(transform.rename("bldg:class"), "分類");
        assert_eq!(transform.rename("luse:class"), "土地利用区分");

        // When the input string has not namespace prefix
        assert_eq!(transform.rename("foo"), "foo");
        assert_eq!(transform.rename("wo_namespace"), "wo_namespace_new");
    }

    #[test]
    fn test_rename_invalid_wildcard() {
        let mut transform = EditFieldNamesTransform::new();
        let mut map = HashMap::new();
        // Only the keys with the prefix "*:" are regarded as wildcard
        map.insert("*use:class".to_string(), "土地利用区分".to_string());
        transform.extend_rename_map(map);

        assert_eq!(transform.rename("luse:class"), "class"); // not renamed
        assert_eq!(transform.rename("bldg:class"), "class");
        assert_eq!(transform.rename("*use:class"), "土地利用区分");
    }
}
