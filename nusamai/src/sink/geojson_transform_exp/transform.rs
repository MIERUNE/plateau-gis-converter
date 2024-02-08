use std::path::PathBuf;

use ahash::HashMap;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use nusamai_citygml::{
    object::{Map, Object, ObjectStereotype},
    Value,
};
use nusamai_plateau::Entity;

// Configure settings to control how city objects are transformed
// Assumed to be loaded from json file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mappings {
    types: HashMap<String, HashMap<String, Param>>,
    settings: Settings,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings {
    load_semantic_parts: bool,
    to_json_string: bool,
    to_tabular: bool,
}

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
struct Param {
    name: String,
    type_name: String,
    remove: bool,
}

// struct FeatureCollector {}
//
// impl FeatureCollector {
//     fn collect(&self, city_object: Entity) -> Entity {
//         // attributes内のFeature（子要素）を全て取り出す
//         let feature_ref = match &city_object.root {
//             Value::Feature(f) => f.clone(),
//             _ => panic!(
//                 "Root value type must be Feature, but found {:?}",
//                 city_object.root
//             ),
//         };
//
//         let mut child_features = Vec::new();
//         for (_, value) in feature_ref.attributes.iter() {
//             let features = extract_features(value);
//             child_features.extend(features);
//         }
//
//         // トップレベルのcity_objectsの地物を取り出す
//         let mut child_geometry_refs = Vec::new();
//         for g in &feature_ref.geometries {
//             child_geometry_refs.extend(g.clone());
//         }
//
//         // child_features内のgeometriesを全て取り出して、toplevel_featureのgeometriesに追加する
//         for f in &child_features {
//             if let Some(geometry_refs) = &f.geometries {
//                 for g in geometry_refs {
//                     child_geometry_refs.push(g.clone());
//                 }
//             }
//         }
//
//         // child_geometry_refの重複を除去する
//         let child_geometry_refs: Vec<GeometryRefEntry> =
//             child_geometry_refs
//                 .into_iter()
//                 .fold(Vec::new(), |mut acc, x| {
//                     if !acc.contains(&x) {
//                         acc.push(x);
//                     }
//                     acc
//                 });
//
//         let feature = Feature {
//             id: feature_ref.id.clone(),
//             typename: feature_ref.typename.clone(),
//             attributes: feature_ref.attributes.clone(),
//             geometries: Some(child_geometry_refs.clone()),
//         };
//
//         Entity {
//             root: Value::Feature(feature),
//             geometry_store: city_object.geometry_store.clone(),
//         }
//     }
// }

trait Transform {
    fn transform(&self, entity: Entity) -> Vec<Entity>;
}

struct FeatureFlatteningTransform {}

impl Transform for FeatureFlatteningTransform {
    fn transform(&self, mut entity: Entity) -> Vec<Entity> {
        // SeparateLodTransformerとは両立できない

        if !matches!(entity.root, Value::Object(_)) {
            panic!(
                "Root value type must be Feature, but found {:?}",
                entity.root
            );
        }
        let mut flattened = Vec::new();
        flatten(&mut entity.root, true, false, &mut flattened);
        flattened.push(entity.root);
        flattened
            .into_iter()
            .map(|value| Entity {
                root: value,
                geometry_store: entity.geometry_store.clone(),
                appearance_store: entity.appearance_store.clone(),
            })
            .collect()
    }
}

struct DataFlatteningTransform {}

impl Transform for DataFlatteningTransform {
    fn transform(&self, mut entity: Entity) -> Vec<Entity> {
        if !matches!(entity.root, Value::Object(_)) {
            panic!(
                "Root value type must be Feature, but found {:?}",
                entity.root
            );
        }
        let mut flattened = Vec::new();
        flatten(&mut entity.root, true, false, &mut flattened);
        flattened.push(entity.root);
        flattened
            .into_iter()
            .map(|value| Entity {
                root: value,
                geometry_store: entity.geometry_store.clone(),
                appearance_store: entity.appearance_store.clone(),
            })
            .collect()
    }
}

fn flatten(
    value: &mut Value,
    do_flatten_feature: bool,
    do_flatten_data: bool,
    out: &mut Vec<Value>,
) {
    if let Value::Object(obj) = value {
        let mut new_attrs = Map::default();
        for (key, value) in obj.attributes.drain(..) {
            match value {
                Value::Array(mut child_arr) => match child_arr.first() {
                    Some(Value::Object(obj)) => {
                        let do_flatten = match obj.stereotype {
                            ObjectStereotype::Feature { .. } if do_flatten_feature => true,
                            ObjectStereotype::Data { .. } if do_flatten_data => true,
                            _ => false,
                        };

                        if do_flatten {
                            for mut value in child_arr {
                                flatten(&mut value, do_flatten_feature, do_flatten_data, out);
                                out.push(value);
                            }
                        } else {
                            for value in child_arr.iter_mut() {
                                flatten(value, do_flatten_feature, do_flatten_data, out);
                            }
                            new_attrs.insert(key, Value::Array(child_arr));
                        }
                    }
                    _ => {
                        new_attrs.insert(key, Value::Array(child_arr));
                    }
                },
                mut value @ Value::Object(_) => {
                    flatten(&mut value, do_flatten_feature, do_flatten_data, out);
                    if do_flatten_feature {
                        out.push(value);
                    } else {
                        new_attrs.insert(key, value);
                    }
                }
                _ => {
                    new_attrs.insert(key, value);
                }
            }
        }
        obj.attributes = new_attrs;
    }
}

struct LodSeparationTransform {}

impl Transform for LodSeparationTransform {
    fn transform(&self, entity: Entity) -> Vec<Entity> {
        let mut results = Vec::new();

        let Entity {
            root: Value::Object(obj),
            ..
        } = entity
        else {
            panic!(
                "Root value type must be Object (class), but found {:?}",
                entity.root
            );
        };

        let ObjectStereotype::Feature { id, geometries } = obj.stereotype else {
            panic!("Stereotype must be Feature");
        };

        // feature.geometriesから、lodごとに分割して、objectsに追加する
        let mut lods: IndexMap<usize, Vec<_>> = IndexMap::new();
        for g in geometries {
            match g.lod {
                0 => lods.entry(0).or_default().push(g),
                1 => lods.entry(1).or_default().push(g),
                2 => lods.entry(2).or_default().push(g),
                3 => lods.entry(3).or_default().push(g),
                4 => lods.entry(4).or_default().push(g),
                _ => unreachable!("lod must be [0-4]"),
            }
        }

        for (_, geometry) in lods {
            results.push(Entity {
                root: Value::Object(Object {
                    typename: obj.typename.clone(),
                    attributes: obj.attributes.clone(),
                    stereotype: ObjectStereotype::Feature {
                        id: id.clone(),
                        geometries: geometry,
                    },
                }),
                geometry_store: entity.geometry_store.to_owned(),
                appearance_store: entity.appearance_store.to_owned(),
            });
        }

        results
    }
}

struct SerialTransform {
    transforms: Vec<Box<dyn Transform>>,
}

impl SerialTransform {
    fn push(&mut self, transformer: Box<dyn Transform>) {
        self.transforms.push(transformer);
    }
}

impl Transform for SerialTransform {
    fn transform(&self, entity: Entity) -> Vec<Entity> {
        let mut entities = vec![entity];
        let mut temp_entities = Vec::new();

        for transformer in &self.transforms {
            temp_entities.clear();

            for entity in entities.drain(..) {
                temp_entities.extend(transformer.transform(entity));
            }
            std::mem::swap(&mut entities, &mut temp_entities);
        }

        entities
    }
}

trait Editor {
    fn edit(&self, entity: &mut Entity, mappings: &Mappings);
}

struct JSONStringifyEditor {}

impl Editor for JSONStringifyEditor {
    fn edit(&self, entity: &mut Entity, mappings: &Mappings) {
        let root_typename = match &entity.root {
            Value::Object(f) => &f.typename,
            _ => panic!(
                "Root value type must be Feature or Data, but found {:?}",
                entity.root
            ),
        };
        let Some(target) = mappings.types.get(root_typename.as_ref()) else {
            return;
        };

        if let Value::Object(obj) = &mut entity.root {
            let attributes = std::mem::take(&mut obj.attributes);

            for (key, mut value) in attributes {
                let param = target.get(&key);

                if let Some(p) = param {
                    if p.remove {
                        continue;
                    };

                    value = if p.type_name == "string" {
                        match value {
                            Value::Array(a) => {
                                let array = a
                                    .into_iter()
                                    .map(|v| {
                                        if let Value::String(s) = v {
                                            s
                                        } else {
                                            panic!("Array must be String, but found {:?}", v)
                                        }
                                    })
                                    .collect::<Vec<String>>()
                                    .join(",");
                                Value::String(array)
                            }
                            Value::Object(f) => {
                                let json_feature = serde_json::to_string(&f.attributes).unwrap();
                                Value::String(json_feature)
                            }
                            _ => value,
                        }
                    } else if p.type_name == "json" {
                        match value {
                            Value::Array(_) | Value::Object(_) => {
                                Value::String(value.to_attribute_json().to_string())
                            }
                            _ => value,
                        }
                    } else {
                        value
                    };

                    let key = match p.name.as_ref() {
                        "" => key,
                        _ => p.name.clone(),
                    };
                    obj.attributes.insert(key, value);
                } else {
                    obj.attributes.insert(key, value);
                };
            }
        }
    }
}

struct SerialEditor<'a> {
    editors: Vec<Box<dyn Editor>>,
    mappings: &'a Mappings,
}

impl<'a> SerialEditor<'a> {
    fn add(&mut self, traversal: Box<dyn Editor>) {
        self.editors.push(traversal);
    }
}

impl<'a> Editor for SerialEditor<'a> {
    fn edit(&self, entity: &mut Entity, _mappings: &Mappings) {
        for traversal in &self.editors {
            traversal.edit(entity, self.mappings);
        }
    }
}

#[derive(Debug, Default)]
pub struct ObjectTransformer {}

impl ObjectTransformer {
    pub fn transform(&self, entity: Entity, mapping_path: &PathBuf) -> Vec<Entity> {
        // attributes内の子要素（feature）を収集して、トップレベルに引き上げる
        // let obj = FeatureCollector {}.collect(obj);
        // REVIEW(fukada): ↑これは必要?

        // 仮にJSONファイルから設定を読み込む
        let file = std::fs::File::open(mapping_path).unwrap();
        let reader = std::io::BufReader::new(file);
        let mappings: Mappings = serde_json::from_reader(reader).unwrap();
        let settings = &mappings.settings;

        // LODごとの分離などの、Entity->Vec<Entity>を行うTransformer
        let mut serial_transform = SerialTransform {
            transforms: Vec::new(),
        };
        if settings.to_tabular {
            serial_transform.push(Box::new(DataFlatteningTransform {}));
        }
        if settings.load_semantic_parts {
            serial_transform.push(Box::new(FeatureFlatteningTransform {}));
        } else {
            serial_transform.push(Box::new(LodSeparationTransform {}));
        }
        let mut entities = serial_transform.transform(entity);

        // 属性の変換などを行うEditor
        let mut traversal = SerialEditor {
            editors: Vec::new(),
            mappings: &mappings,
        };
        if settings.to_json_string {
            traversal.add(Box::new(JSONStringifyEditor {}));
        }
        entities.iter_mut().for_each(|entity| {
            traversal.edit(entity, &mappings);
        });

        // 参考情報としてのファイル出力
        let file = std::fs::File::create("./output_object.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &entities).unwrap();

        entities
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use nusamai_citygml::{
        object::{self, Map, ObjectStereotype},
        Code, GeometryRefEntry, GeometryStore, GeometryType, Value,
    };
    use nusamai_plateau::Entity;

    fn make_mapping_rule() -> Mappings {
        let json_text = r#"
            {
                "settings": {
                    "load_semantic_parts": false,
                    "to_tabular": true,
                    "to_json_string": true
                },
                "types": {
                    "bldg:Building": {
                        "uro:buildingIDAttribute": {
                            "name": "",
                            "type_name": "",
                            "remove": true
                        },
                        "bldg:boundedBy": {
                            "name": "",
                            "type_name": "",
                            "remove": true
                        },
                        "gen:genericAttribute": {
                            "name": "",
                            "type_name": "",
                            "remove": true
                        },
                        "uro:buildingDataQualityAttribute": {
                            "name": "",
                            "type_name": "",
                            "remove": true
                        },
                        "uro:buildingDetailAttribute": {
                            "name": "",
                            "type_name": "",
                            "remove": true
                        },
                        "uro:buildingDisasterRiskAttribute": {
                            "name": "",
                            "type_name": "",
                            "remove": true
                        }
                    }
                }
            }
        "#;
        let mappings: Mappings = serde_json::from_str(json_text).unwrap();

        assert!(!mappings.settings.load_semantic_parts);
        assert!(mappings.settings.to_tabular);
        assert!(mappings.settings.to_json_string);

        mappings
    }

    fn make_dummy_entity() -> Entity {
        let mut attributes_1 = Map::default();
        attributes_1.insert("String".to_string(), Value::String("test".to_string()));
        attributes_1.insert("Integer".to_string(), Value::Integer(1));
        attributes_1.insert(
            "Code".to_string(),
            Value::Code(Code::new("test".to_string(), "1".to_string())),
        );
        let geometries_1 = vec![GeometryRefEntry {
            ty: GeometryType::Surface,
            lod: 0,
            pos: 0,
            len: 0,
        }];

        let obj_1 = Object {
            typename: "bldg_47af72fa-7135-4a1c-b93c-d69e3b245cc6".into(),
            attributes: attributes_1,
            stereotype: object::ObjectStereotype::Feature {
                id: "bldg:Building".to_string(),
                geometries: geometries_1,
            },
        };

        Entity {
            root: Value::Object(obj_1),
            geometry_store: RwLock::new(GeometryStore {
                ..Default::default()
            })
            .into(),
            appearance_store: Default::default(),
        }
    }

    #[test]
    fn test_load_mappings() {
        let mappings = make_mapping_rule();
        assert!(mappings.types.get("bldg:Building").is_some());
    }

    #[test]
    fn test_entity() {
        let entity = make_dummy_entity();
        if let Value::Object(obj) = &entity.root {
            assert_eq!(obj.typename, "bldg_47af72fa-7135-4a1c-b93c-d69e3b245cc6");
            assert_eq!(obj.attributes.len(), 3);

            let ObjectStereotype::Feature { id, geometries } = &obj.stereotype else {
                panic!("Stereotype must be Feature");
            };

            assert_eq!(id, "bldg:Building");
            assert_eq!(geometries.len(), 1);
        } else {
            panic!(
                "Root value type must be Feature, but found {:?}",
                entity.root
            );
        }
    }
}
