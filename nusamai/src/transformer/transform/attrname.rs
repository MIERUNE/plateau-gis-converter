use crate::transformer::Transform;

use hashbrown::HashMap;
use indexmap::map::MutableKeys;
use nusamai_citygml::object::Value;
use nusamai_citygml::schema::Schema;
use nusamai_citygml::schema::TypeDef;
use nusamai_plateau::Entity;

/// Transform to edit field names
///
/// The current implementation performs the following operations:
///
/// - Remove the namespace prefix from the field names (e.g., `"ns:foo"` -> `"foo"`)
/// - Rename the field names for Shapefile according to the dictionary
/// - Rename the field names given the rules by the user
///
/// You may specify the rules in two ways:
/// - Exact match: Rename the field name if the key matches exactly (e.g., `{"ns:foo: "bar"}`)
/// - Wildcard match: Rename the field name for any namespace prefix `*:` (e.g., `{"*:foo": "bar"}`)
/// Note that the exact match takes precedence over the wildcard match.
#[derive(Default, Clone)]
pub struct EditFieldNamesTransform {
    // Exact string match dictionary
    exact_rename_map: HashMap<String, String>,
    // Wildcard match dictionary - the stored keys are the string after the prefix "*:"
    wildcard_rename_map: HashMap<String, String>,
}

impl EditFieldNamesTransform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_default_map_for_shape(&mut self) {
        const SHAPE_DICT: &str = include_str!("./shp_field_dict.json");
        let map: HashMap<String, String> = serde_json::from_str(SHAPE_DICT).unwrap();
        self.wildcard_rename_map.extend(map);
        for value in self.wildcard_rename_map.values() {
            if value.len() > 10 {
                panic!("The key length must be less than 10 characters: {}", value);
            }
        }
    }

    pub fn extend_rename_map(&mut self, map: HashMap<String, String>) {
        for (before, after) in map {
            if let Some(before_stripped) = before.strip_prefix("*:") {
                self.wildcard_rename_map
                    .insert(before_stripped.into(), after);
            } else {
                self.exact_rename_map.insert(before, after);
            }
        }
    }
}

impl Transform for EditFieldNamesTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        self.edit_tree(&mut entity.root);
        out.push(entity);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for ty in schema.types.values_mut() {
            let atrs = match ty {
                TypeDef::Data(data) => &mut data.attributes,
                TypeDef::Feature(feat) => &mut feat.attributes,
                TypeDef::Property(_) => continue,
            };
            atrs.retain2(|key, value| {
                if let Some(new_name) = self.rename(key) {
                    value.original_name = Some(key.clone());
                    *key = new_name.to_string();
                }
                true
            });
        }
    }
}

impl EditFieldNamesTransform {
    fn rename<'a>(&'a self, name: &'a str) -> Option<&str> {
        let mut new_name = None;

        // Lookup and rename: exact match
        if let Some(new_key) = self.exact_rename_map.get(name) {
            new_name = Some(new_key.as_ref());
            return new_name;
        }

        // Lookup and rename: wildcard match
        if let Some(pos) = name.find(':') {
            let key = &name[pos + 1..];
            if let Some(new_key) = self.wildcard_rename_map.get(key) {
                new_name = Some(new_key.as_ref());
                return new_name;
            }
        }

        // Just remove the namespace suffix
        if let Some(pos) = name.find(':') {
            new_name = Some(&name[pos + 1..]);
        }

        new_name
    }

    fn edit_tree(&self, value: &mut Value) {
        match value {
            Value::Object(obj) => {
                obj.attributes.retain2(|key, value| {
                    self.edit_tree(value);
                    if let Some(new_name) = self.rename(key) {
                        *key = new_name.to_string();
                    }
                    true
                });
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
        transform.extend_rename_map(map);

        // In any case, namespace suffix is removed
        assert_eq!(transform.rename("namespace:foo"), Some("foo"));

        // Note: a rule written with specific namespace takes precedence, compared to the general rule with "*:"
        assert_eq!(transform.rename("bldg:class"), Some("分類"));
        assert_eq!(transform.rename("luse:class"), Some("土地利用区分"));
    }

    #[test]
    fn test_rename_invalid_wildcard() {
        let mut transform = EditFieldNamesTransform::new();
        let mut map = HashMap::new();
        // Only the keys with the prefix "*:" are regarded as wildcard
        map.insert("*use:class".to_string(), "土地利用区分".to_string());
        transform.extend_rename_map(map);

        assert_eq!(transform.rename("luse:class"), Some("class")); // not renamed
        assert_eq!(transform.rename("bldg:class"), Some("class"));
        assert_eq!(transform.rename("*use:class"), Some("class"));
    }
}
