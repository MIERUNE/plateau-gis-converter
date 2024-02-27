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
/// - Remove the namespace prefix from the field names
/// - Rename the field names for Shapefile according to the dictionary
#[derive(Default, Clone)]
pub struct EditFieldNamesTransform {
    rename_map: HashMap<String, String>,
}

impl EditFieldNamesTransform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_default_map_for_shape(&mut self) {
        const SHAPE_DICT: &str = include_str!("./shp_field_dict.json");
        let map: HashMap<String, String> = serde_json::from_str(SHAPE_DICT).unwrap();
        self.rename_map.extend(map);
        for value in self.rename_map.values() {
            if value.len() > 10 {
                panic!("The key length must be less than 10 characters: {}", value);
            }
        }
    }

    pub fn extend_rename_map(&mut self, map: HashMap<String, String>) {
        // Note: the value is updated if the key already exists
        self.rename_map.extend(map);
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
        // remove namespace prefix
        if let Some(pos) = name.find(':') {
            new_name = Some(&name[pos + 1..]);
        }
        // lookup dictionary
        if !self.rename_map.is_empty() {
            if let Some(name) = new_name {
                if let Some(new_key) = self.rename_map.get(name) {
                    new_name = Some(new_key.as_ref());
                }
            }
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
        map.insert("old_name".to_string(), "new_name".to_string());
        transform.extend_rename_map(map);

        assert_eq!(transform.rename("ns:old_name"), Some("new_name"));
        assert_eq!(transform.rename("ns:old_name2"), Some("old_name2"));
    }
}
