use std::default;

use ahash::RandomState;
use indexmap::IndexMap;

use nusamai_citygml::object::{self, CityObject, Data, Feature, Map};
use nusamai_citygml::Value;
use serde::{Deserialize, Serialize};

// 以下、仮実装
#[derive(Debug, Default, Serialize, Deserialize)]
enum SettingValue {
    Json(String),
    Separate(bool),
    #[default]
    None,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    load_semantic_parts: bool,
    to_json_string: bool,
    target_lods: Vec<bool>,
    load_lower_lods: bool,
    load_upper_lods: bool,
    mapping: IndexMap<String, SettingValue, RandomState>,
}

pub trait ObjectSeparator {
    fn separate(&self, cityobj: &CityObject) -> Vec<CityObject>;
}

#[derive(Debug, Default)]
pub struct SemanticObjectSeparator {
    pub settings: Settings,
}

impl ObjectSeparator for SemanticObjectSeparator {
    fn separate(&self, cityobj: &CityObject) -> Vec<CityObject> {
        // パフォーマンスなどを無視し、わかりやすさのためにコピーしたデータを用意しておく
        let toplevel_feature = match &cityobj.root {
            Value::Feature(f) => f.clone(),
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                cityobj.root
            ),
        };
        let root_gml_id = &toplevel_feature.id;
        let typename = &toplevel_feature.typename;
        println!("{:?}, {:?}", root_gml_id, typename);

        let toplevel_geometries = &cityobj.geometries;
        let toplevel_geometry_ref = &toplevel_feature.geometries;
        let toplevel_attributes = &toplevel_feature.attributes;

        // let mut primitives: IndexMap<String, Value, RandomState> =
        //     IndexMap::with_hasher(RandomState::new());
        // let mut features: IndexMap<String, Vec<Feature>> = IndexMap::new();
        // let mut data_list: IndexMap<String, Vec<Data>> = IndexMap::new();
        // let mut other_layer_data: IndexMap<String, Vec<Data>> = IndexMap::new();
        // let mut other_layer_attributes: IndexMap<String, Vec<IndexMap<String, Value>>> =
        //     IndexMap::new();

        // for (key, value) in toplevel_feature.attributes.iter() {
        //     match value {
        //         Value::Array(_) | Value::Feature(_) | Value::Data(_) => {}
        //         _ => {
        //             primitives.insert(key.clone(), value.clone());
        //         }
        //     }
        // }

        // for (key, value) in toplevel_feature.attributes.iter() {
        //     if let Value::Data(d) = value {
        //         data_list.insert(key.clone(), vec![d.clone()]);
        //     }
        // }

        // for (key, value) in toplevel_feature.attributes.iter() {
        //     if let Value::Feature(f) = value {
        //         features.insert(key.clone(), vec![f.clone()]);
        //     }
        // }

        // for (key, value) in toplevel_feature.attributes.iter() {
        //     if let Value::Array(a) = value {
        //         for v in a.iter() {
        //             match v {
        //                 Value::Data(d) => {
        //                     if other_layer_data.contains_key(key) {
        //                         other_layer_data.get_mut(key).unwrap().push(d.clone());
        //                     } else {
        //                         other_layer_data.insert(key.clone(), vec![d.clone()]);
        //                     }
        //                 }
        //                 Value::Feature(f) => {
        //                     features.insert(key.clone(), vec![f.clone()]);
        //                 }
        //                 _ => {
        //                     primitives.insert(key.clone(), value.clone());
        //                 }
        //             }
        //         }
        //     }
        // }

        // for (key, value) in data_list.iter() {
        //     for d in value.iter() {
        //         let attributes = &d.attributes;
        //         for (k, v) in attributes.iter() {
        //             primitives.insert(k.clone(), v.clone());
        //         }
        //     }
        // }

        // for (key, value) in other_layer_data.iter() {
        //     let mut other: IndexMap<String, Value> = IndexMap::new();
        //     for d in value.iter() {
        //         let attributes = &d.attributes;
        //         for (k, v) in attributes.iter() {
        //             other.insert(k.clone(), v.clone());
        //         }
        //     }
        //     if other_layer_attributes.contains_key(key) {
        //         other_layer_attributes.get_mut(key).unwrap().push(other);
        //     } else {
        //         other_layer_attributes.insert(key.clone(), vec![other]);
        //     }
        // }

        // 上記は参考程度

        // ここから処理を書いていく
        // 何かしらの設定ファイルを受け取り、以下のような設定ができると嬉しい
        // ・Arrayはjson文字列に変換する
        // ・Arrayの中身がDataなら、別のレイヤーにする
        // ・Arrayの中身がFeatureなら、別のレイヤーにする
        // ・特定の属性のみ形状を変換する
        // 最終的には上記のように改修されたVec<CityObject>を返す

        // 返却用の入れ物を用意する
        let mut objects = Vec::new();

        // デフォルトで、LODごとに分割してobjectsに追加する
        if let Some(f) = &toplevel_feature.geometries {
            for g in f.iter() {
                let toplevel_feature = Feature {
                    id: toplevel_feature.id.clone(),
                    typename: toplevel_feature.typename.clone(),
                    attributes: toplevel_feature.attributes.clone(),
                    geometries: Some(vec![g.clone()]),
                };

                let obj = CityObject {
                    root: Value::Feature(toplevel_feature),
                    geometries: cityobj.geometries.clone(),
                };

                objects.push(obj);
            }
        }

        // attributes内のFeature（子要素）を全て取り出す
        let mut child_features = Vec::new();
        for (key, value) in toplevel_feature.attributes.iter() {
            let features = self.extract_features(value);
            child_features.extend(features);
        }

        // 仮の設定を作成する
        let mut settings = Settings::default();
        settings.load_semantic_parts = false;

        // 設定に応じてfeaturesをセマンティックごとに分割する
        if settings.load_semantic_parts {
            for f in &child_features {
                let obj = CityObject {
                    root: Value::Feature(f.clone()),
                    geometries: cityobj.geometries.clone(),
                };
                objects.push(obj);
            }
        } else {
            // セマンティックごとに分割しない場合は、child_features内のgeometriesを全て取り出して、toplevel_featureのgeometriesに追加する
            let mut child_geometry_refs = Vec::new();
            for f in &child_features {
                if let Some(geometry_refs) = &f.geometries {
                    child_geometry_refs.extend(geometry_refs.clone());
                }
            }

            // todo: さらに、LODごとに分割する必要がある

            if !child_geometry_refs.is_empty() {
                let mut root = toplevel_feature.clone();
                root.geometries = Some(child_geometry_refs);

                let obj = CityObject {
                    root: Value::Feature(root),
                    geometries: cityobj.geometries.clone(),
                };

                objects.push(obj);
            }
        }

        // Array・Data・featureは全てJSON文字列に変換する
        settings.to_json_string = true;

        if settings.to_json_string {
            for _ in 0..objects.len() {
                let object = objects.remove(0);
                let mut attributes = IndexMap::with_hasher(RandomState::new());
                if let Value::Feature(f) = &object.root {
                    for (key, value) in f.attributes.iter() {
                        match value {
                            Value::Array(a) => {
                                let json_array = serde_json::to_string(a).unwrap();
                                attributes.insert(key.clone(), Value::String(json_array));
                            }
                            Value::Data(d) => {
                                let json_data = serde_json::to_string(&d.attributes).unwrap();
                                attributes.insert(key.clone(), Value::String(json_data));
                            }
                            Value::Feature(f) => {
                                let json_feature = serde_json::to_string(&f.attributes).unwrap();
                                attributes.insert(key.clone(), Value::String(json_feature));
                            }
                            _ => {
                                attributes.insert(key.clone(), value.clone());
                            }
                        }
                    }

                    let feature = Feature {
                        id: f.id.clone(),
                        typename: f.typename.clone(),
                        attributes,
                        geometries: f.geometries.clone(),
                    };

                    let obj = CityObject {
                        root: Value::Feature(feature),
                        geometries: object.geometries,
                    };

                    objects.push(obj);
                }
            }
        } else {
            // todo: JSON化しない場合は、テーブル分割される
        }

        println!("{:?}", objects.len());
        for o in &objects {
            if let Value::Feature(f) = &o.root {
                println!("{:?}", f.geometries);
            }
        }

        if objects.len() > 2 {
            let file = std::fs::File::create("/Users/satoru/Downloads/output/test.json").unwrap();
            let mut writer = std::io::BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &objects).unwrap();
        }

        println!();

        objects
    }
}

impl SemanticObjectSeparator {
    // 子要素収集のためのユーティリティ
    fn extract_features(&self, value: &Value) -> Vec<Feature> {
        match value {
            Value::Array(vec) => vec.iter().flat_map(|v| self.extract_features(v)).collect(),
            Value::Feature(feature) => {
                vec![feature.clone()]
            }
            _ => Vec::new(),
        }
    }

    fn jsonify(&self, objects: Vec<&CityObject>) -> Vec<CityObject> {
        todo!();
    }

    fn separate_semantics(&self, objects: Vec<&CityObject>) -> Vec<CityObject> {
        todo!();
    }

    fn separate_layers(&self, objects: Vec<&CityObject>) -> Vec<CityObject> {
        todo!();
    }

    fn change_attribute_name(&self, objects: Vec<&CityObject>) -> Vec<CityObject> {
        todo!();
    }
}
