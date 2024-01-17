use ahash::RandomState;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use nusamai_citygml::{
    object::{CityObject, Data, Feature},
    GeometryRefEntry, Value,
};

// 以下、仮実装
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
enum SettingValue {
    Json(String),
    Separate(bool),
    #[default]
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings {
    load_semantic_parts: bool,
    to_json_string: bool,
    to_tabular: bool,
    mappings: IndexMap<String, SettingValue, RandomState>,
}

trait Transformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject>;
}

struct FeatureCollectTransformer {}
impl Transformer for FeatureCollectTransformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject> {
        let mut results = Vec::new();

        // attributes内のFeature（子要素）を全て取り出す
        let feature_ref = match &city_object.root {
            Value::Feature(f) => f.clone(),
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                city_object.root
            ),
        };

        let mut child_features = Vec::new();
        for (_, value) in feature_ref.attributes.iter() {
            let features = extract_features(value);
            child_features.extend(features);
        }

        // トップレベルのcity_objectsの地物を取り出す
        let mut child_geometry_refs = Vec::new();
        for g in &feature_ref.geometries {
            child_geometry_refs.extend(g.clone());
        }

        // child_features内のgeometriesを全て取り出して、toplevel_featureのgeometriesに追加する
        for f in &child_features {
            if let Some(geometry_refs) = &f.geometries {
                for g in geometry_refs {
                    child_geometry_refs.push(g.clone());
                }
            }
        }

        // child_geometry_refの重複を除去する
        let child_geometry_refs: Vec<GeometryRefEntry> =
            child_geometry_refs
                .into_iter()
                .fold(Vec::new(), |mut acc, x| {
                    if !acc.contains(&x) {
                        acc.push(x);
                    }
                    acc
                });

        let feature = Feature {
            id: feature_ref.id.clone(),
            typename: feature_ref.typename.clone(),
            attributes: feature_ref.attributes.clone(),
            geometries: Some(child_geometry_refs.clone()),
        };

        let obj = CityObject {
            root: Value::Feature(feature),
            geometry_store: city_object.geometry_store.clone(),
        };
        results.push(obj);

        results
    }
}

struct SemanticSplitTransformer {}
impl Transformer for SemanticSplitTransformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject> {
        let mut results = Vec::new();

        // SeparateLodTransformerとは両立できない

        let mut child_features = Vec::new();

        if let Value::Feature(feature_lef) = &city_object.root {
            // attributes内のFeature（子要素）を全て取り出す
            for (_, value) in feature_lef.attributes.iter() {
                match value {
                    Value::Array(array) => {
                        for v in array {
                            let features = extract_features(v);
                            child_features.extend(features);
                        }
                    }
                    Value::Feature(f) => {
                        child_features.push(f.clone());
                    }
                    _ => {}
                }
            }
        }

        // child_featuresの重複を除去する
        let child_features: Vec<Feature> =
            child_features.into_iter().fold(Vec::new(), |mut acc, x| {
                if !acc.contains(&x) {
                    acc.push(x);
                }
                acc
            });

        // featuresをセマンティックごとに分割する
        for f in &child_features {
            let obj = CityObject {
                root: Value::Feature(f.clone()),
                geometry_store: city_object.geometry_store.clone(),
            };
            results.push(obj);
        }

        results
    }
}

struct DataCollectTransformer {}
impl Transformer for DataCollectTransformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject> {
        let mut results = Vec::new();
        let mut other_layer_data_list = Vec::new();

        // attributes内のArrayを取り出し、中身がData（子要素）で持つものを全て取り出す
        let feature_ref = match &city_object.root {
            Value::Feature(f) => f,
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                city_object.root
            ),
        };

        for (_, value) in feature_ref.attributes.iter() {
            let array = extract_array(value);
            for v in array {
                let data_list = extract_data(&v);
                for d in &data_list {
                    other_layer_data_list.push(d.clone());
                }
            }
        }

        // other_layer_data_listに重複があれば除去する
        let other_layer_data_list: Vec<Data> =
            other_layer_data_list
                .into_iter()
                .fold(Vec::new(), |mut acc, x| {
                    if !acc.contains(&x) {
                        acc.push(x);
                    }
                    acc
                });

        // other_layer_data_listを利用して、city_objectsを再構築する
        for d in &other_layer_data_list {
            let mut attributes = IndexMap::with_hasher(RandomState::new());
            for (key, value) in d.attributes.iter() {
                attributes.insert(key.clone(), value.clone());
            }

            let data = Data {
                typename: d.typename.clone(),
                attributes,
            };

            let obj = CityObject {
                root: Value::Data(data),
                geometry_store: city_object.geometry_store.clone(),
            };

            results.push(obj);
        }

        results.push(city_object);

        results
    }
}

struct SeparateLodTransformer {}
impl Transformer for SeparateLodTransformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject> {
        let mut results = Vec::new();

        let feature_ref = match &city_object.root {
            Value::Feature(f) => f,
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                city_object.root
            ),
        };

        // feature.geometriesから、lodごとに分割して、objectsに追加する
        let mut lods: IndexMap<usize, Vec<GeometryRefEntry>> = IndexMap::new();
        if let Some(geometry_ref_list) = &feature_ref.geometries {
            for g in geometry_ref_list.iter() {
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

            let feature = feature_ref.clone();

            for (_, geometry_refs) in lods.iter() {
                let feature = Feature {
                    id: feature.id.clone(),
                    typename: feature.typename.clone(),
                    attributes: feature.attributes.clone(),
                    geometries: Some(geometry_refs.clone()),
                };

                let obj = CityObject {
                    root: Value::Feature(feature),
                    geometry_store: city_object.geometry_store.clone(),
                };

                results.push(obj);
            }
        }

        results
    }
}

struct FilterFeaturesTransformer {}
impl Transformer for FilterFeaturesTransformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject> {
        todo!();
    }
}

struct FilterAttributesTransformer {}
impl Transformer for FilterAttributesTransformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject> {
        todo!();
    }
}

struct AttributesTransformer {}
impl Transformer for AttributesTransformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject> {
        todo!();
    }
}

struct TransformerPipeline {
    transformers: Vec<Box<dyn Transformer>>,
}
impl TransformerPipeline {
    fn add(&mut self, transformer: Box<dyn Transformer>) {
        self.transformers.push(transformer);
    }

    fn transform(&self, city_object: CityObject) -> Vec<CityObject> {
        let mut results = Vec::new();
        for transformer in &self.transformers {
            let obj = CityObject {
                root: city_object.root.clone(),
                geometry_store: city_object.geometry_store.clone(),
            };
            let objects = transformer.transform(obj);
            results.extend(objects)
        }

        let results: Vec<CityObject> = results.into_iter().fold(Vec::new(), |mut acc, x| {
            if !acc.contains(&x) {
                acc.push(x);
            }
            acc
        });

        results
    }
}

#[derive(Debug, Default)]
pub struct ObjectTransformer {
    pub settings: Settings,
}

impl ObjectTransformer {
    pub fn transform(&self, cityobj: &CityObject) -> Vec<CityObject> {
        let toplevel_feature = match &cityobj.root {
            Value::Feature(f) => f.clone(),
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                cityobj.root
            ),
        };

        // 仮の設定を作成する
        let mut settings = Settings {
            load_semantic_parts: false,
            to_tabular: true,
            to_json_string: true,
            mappings: IndexMap::with_hasher(RandomState::new()),
        };

        let mut transformer_pipeline = TransformerPipeline {
            transformers: Vec::new(),
        };

        transformer_pipeline.add(Box::new(FeatureCollectTransformer {}));
        // if settings.to_tabular {
        //     transformer_pipeline.add(Box::new(DataCollectTransformer {}));
        // }
        // if settings.load_semantic_parts {
        //     transformer_pipeline.add(Box::new(SemanticSplitTransformer {}));
        // } else {
        //     transformer_pipeline.add(Box::new(SeparateLodTransformer {}));
        // }

        let obj = CityObject {
            root: Value::Feature(toplevel_feature),
            geometry_store: cityobj.geometry_store.clone(),
        };

        let mut objects = transformer_pipeline.transform(obj);

        // Array・Data・featureは全てJSON文字列に変換するかどうか
        // if settings.to_json_string {
        //     for _ in 0..objects.len() {
        //         let object = objects.remove(0);
        //         let mut attributes = IndexMap::with_hasher(RandomState::new());
        //         if let Value::Feature(f) = &object.root {
        //             for (key, value) in f.attributes.iter() {
        //                 match value {
        //                     Value::Array(a) => {
        //                         let json_array = serde_json::to_string(a).unwrap();
        //                         attributes.insert(key.clone(), Value::String(json_array));
        //                     }
        //                     Value::Data(d) => {
        //                         let json_data = serde_json::to_string(&d.attributes).unwrap();
        //                         attributes.insert(key.clone(), Value::String(json_data));
        //                     }
        //                     Value::Feature(f) => {
        //                         let json_feature = serde_json::to_string(&f.attributes).unwrap();
        //                         attributes.insert(key.clone(), Value::String(json_feature));
        //                     }
        //                     _ => {
        //                         attributes.insert(key.clone(), value.clone());
        //                     }
        //                 }
        //             }

        //             let feature = Feature {
        //                 id: f.id.clone(),
        //                 typename: f.typename.clone(),
        //                 attributes,
        //                 geometries: f.geometries.clone(),
        //             };

        //             let obj = CityObject {
        //                 root: Value::Feature(feature),
        //                 geometry_store: object.geometry_store,
        //             };

        //             objects.push(obj);
        //         }
        //     }
        // }

        // todo: 特定の属性のみ形状を変換するような構造を組み込む
        // todo: 上記の設定の内容を検討する
        // todo: プログラムをもう少し構造化する

        println!("objects.len(): {}", objects.len());
        if objects.len() >= 1 {
            for o in &objects {
                if let Value::Feature(f) = &o.root {
                    println!("{:?}: {:?}", f.id, f.geometries);
                }

                let file =
                    std::fs::File::create("/Users/satoru/Downloads/output/test.json").unwrap();
                let writer = std::io::BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &objects).unwrap();
            }
        }

        // println!();

        objects
    }
}

// 子要素収集のためのユーティリティ
fn extract_features(value: &Value) -> Vec<Feature> {
    match value {
        Value::Array(vec) => vec.iter().flat_map(|v| extract_features(v)).collect(),
        Value::Feature(feature) => {
            vec![feature.clone()]
        }
        _ => Vec::new(),
    }
}

fn extract_data(value: &Value) -> Vec<Data> {
    match value {
        Value::Array(vec) => vec.iter().flat_map(|v| extract_data(v)).collect(),
        Value::Data(data) => {
            vec![data.clone()]
        }
        _ => Vec::new(),
    }
}

fn extract_array(value: &Value) -> Vec<Value> {
    match value {
        Value::Array(vec) => vec.clone(),
        _ => Vec::new(),
    }
}
