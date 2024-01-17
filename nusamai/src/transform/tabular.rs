use ahash::RandomState;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use nusamai_citygml::{
    object::{CityObject, Data, Feature},
    GeometryRefEntry, Value,
};

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
    to_tabular: bool,
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
                    geometry_store: cityobj.geometry_store.clone(),
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

        // attributes内のArrayを取り出し、中身がData（子要素）で、なおかつattributesが複数のkeyを持つものを全て取り出す
        let mut other_layer_data_list = Vec::new();
        for (key, value) in toplevel_feature.attributes.iter() {
            let array = self.extract_array(value);
            for v in array {
                let data_list = self.extract_data(&v);
                for d in &data_list {
                    if d.attributes.len() >= 2 {
                        other_layer_data_list.push(d.clone());
                    }
                }
            }
        }

        // 仮の設定を作成する
        let mut settings = Settings::default();

        // 設定に応じてfeaturesをセマンティックごとに分割する
        settings.load_semantic_parts = false;
        if settings.load_semantic_parts {
            for f in &child_features {
                let obj = CityObject {
                    root: Value::Feature(f.clone()),
                    geometry_store: cityobj.geometry_store.clone(),
                };
                objects.push(obj);
            }
        } else {
            // セマンティックごとに分割しない場合は、child_features内のgeometriesを全て取り出して、toplevel_featureのgeometriesに追加する
            let mut child_geometry_refs = Vec::new();
            for f in &child_features {
                if let Some(geometry_refs) = &f.geometries {
                    // lodを確認する
                    child_geometry_refs.extend(geometry_refs.clone());
                }
            }

            if !child_geometry_refs.is_empty() {
                let mut lods: IndexMap<usize, Vec<GeometryRefEntry>> = IndexMap::new();

                for g in &child_geometry_refs {
                    if g.lod == 0 {
                        lods.entry(0).or_insert_with(Vec::new).push(g.clone());
                    } else if g.lod == 1 {
                        lods.entry(1).or_insert_with(Vec::new).push(g.clone());
                    } else if g.lod == 2 {
                        lods.entry(2).or_insert_with(Vec::new).push(g.clone());
                    } else if g.lod == 3 {
                        lods.entry(3).or_insert_with(Vec::new).push(g.clone());
                    } else if g.lod == 4 {
                        lods.entry(4).or_insert_with(Vec::new).push(g.clone());
                    }
                }

                let mut root = toplevel_feature.clone();

                for (lod, geometry_refs) in lods.iter() {
                    let feature = Feature {
                        id: root.id.clone(),
                        typename: root.typename.clone(),
                        attributes: root.attributes.clone(),
                        geometries: Some(geometry_refs.clone()),
                    };

                    let obj = CityObject {
                        root: Value::Feature(feature),
                        geometry_store: cityobj.geometry_store.clone(),
                    };

                    objects.push(obj);
                }
            }
        }

        // other_layer_data_listを利用する
        settings.to_tabular = true;
        if settings.to_tabular {
            for d in &other_layer_data_list {
                let mut attributes = IndexMap::with_hasher(RandomState::new());
                for (key, value) in d.attributes.iter() {
                    attributes.insert(key.clone(), value.clone());
                }

                let feature = Feature {
                    id: root_gml_id.clone(),
                    typename: d.typename.clone(),
                    attributes,
                    geometries: None,
                };

                let obj = CityObject {
                    root: Value::Feature(feature),
                    geometry_store: cityobj.geometry_store.clone(),
                };

                objects.push(obj);
            }
        }

        // Array・Data・featureは全てJSON文字列に変換するかどうか
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
                        geometry_store: object.geometry_store,
                    };

                    objects.push(obj);
                }
            }
        }

        // todo: 特定の属性のみ形状を変換するような構造を組み込む
        // todo: 上記の設定の内容を検討する
        // todo: プログラムをもう少し構造化する

        for o in &objects {
            if let Value::Feature(f) = &o.root {
                println!("{:?}: {:?}", f.id, f.geometries);
            }

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

    fn extract_data(&self, value: &Value) -> Vec<Data> {
        match value {
            Value::Array(vec) => vec.iter().flat_map(|v| self.extract_data(v)).collect(),
            Value::Data(data) => {
                vec![data.clone()]
            }
            _ => Vec::new(),
        }
    }

    fn extract_array(&self, value: &Value) -> Vec<Value> {
        match value {
            Value::Array(vec) => vec.clone(),
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
