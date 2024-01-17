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

pub trait Transformer {
    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject>;
}

pub struct FeatureCollectTransformer {}
impl Transformer for FeatureCollectTransformer {
    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject> {
        let mut results = Vec::new();
        let mut child_geometry_refs = Vec::new();
        let mut child_features = Vec::new();

        // 仮実装: 渡されるcity_objectsは、元々同じCityObjectであるため、すべて同じroot_gml_idを持つ
        if city_objects.is_empty() {
            return city_objects;
        }
        let toplevel_city_object = &city_objects[0];
        let toplevel_feature = match &toplevel_city_object.root {
            Value::Feature(f) => f.clone(),
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                city_objects[0].root
            ),
        };

        // まずは元々のcity_objectsの地物を取り出す
        child_geometry_refs.extend(
            toplevel_feature
                .geometries
                .clone()
                .unwrap_or_else(|| Vec::new()),
        );

        // attributes内のFeature（子要素）を全て取り出す
        for o in &city_objects {
            let feature_ref = match &o.root {
                Value::Feature(f) => f,
                _ => panic!("Root value type must be Feature, but found {:?}", o.root),
            };

            for (_, value) in feature_ref.attributes.iter() {
                let features = extract_features(value);
                child_features.extend(features);
            }
        }

        // child_features内のgeometriesを全て取り出して、toplevel_featureのgeometriesに追加する
        for f in &child_features {
            if let Some(geometry_refs) = &f.geometries {
                child_geometry_refs.extend(geometry_refs.clone());
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
            id: toplevel_feature.id.clone(),
            typename: toplevel_feature.typename.clone(),
            attributes: toplevel_feature.attributes.clone(),
            geometries: Some(child_geometry_refs.clone()),
        };

        let obj = CityObject {
            root: Value::Feature(feature),
            geometry_store: toplevel_city_object.geometry_store.clone(),
        };
        results.push(obj);

        results
    }
}

pub struct SemanticSplitTransformer {}
impl Transformer for SemanticSplitTransformer {
    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject> {
        let mut results = Vec::new();

        // 仮実装: FeatureCollectTransformerとは両立しない

        // attributes内のFeature（子要素）を全て取り出す
        for o in &city_objects {
            let feature_ref = match &o.root {
                Value::Feature(f) => f,
                _ => panic!("Root value type must be Feature, but found {:?}", o.root),
            };

            let mut child_features = Vec::new();
            for (_, value) in feature_ref.attributes.iter() {
                let features = extract_features(value);
                child_features.extend(features);
            }

            // featuresをセマンティックごとに分割する
            for f in &child_features {
                let obj = CityObject {
                    root: Value::Feature(f.clone()),
                    geometry_store: o.geometry_store.clone(),
                };
                results.push(obj);
            }
        }

        results
    }
}

pub struct FlattenTreeTransformer {}
impl Transformer for FlattenTreeTransformer {
    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject> {
        let mut results = Vec::new();
        let mut other_layer_data_list = Vec::new();

        // 仮実装: 渡されるcity_objectsは、元々同じCityObjectであるため、すべて同じroot_gml_idを持つ
        if city_objects.is_empty() {
            return city_objects;
        }
        let toplevel_feature = match &city_objects[0].root {
            Value::Feature(f) => f.clone(),
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                city_objects[0].root
            ),
        };
        let root_gml_id = &toplevel_feature.id;
        let geometry_store = city_objects[0].geometry_store.clone();

        // attributes内のArrayを取り出し、中身がData（子要素）で持つものを全て取り出す
        for o in &city_objects {
            let feature_ref = match &o.root {
                Value::Feature(f) => f,
                _ => panic!("Root value type must be Feature, but found {:?}", o.root),
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

            let feature = Feature {
                id: root_gml_id.clone(),
                typename: d.typename.clone(),
                attributes,
                geometries: None,
            };

            let obj = CityObject {
                root: Value::Feature(feature),
                geometry_store: geometry_store.clone(),
            };

            results.push(obj);
        }

        for o in city_objects {
            let obj = CityObject {
                root: o.root.clone(),
                geometry_store: o.geometry_store.clone(),
            };
            results.push(obj)
        }

        results
    }
}

struct SeparateLodTransformer {}
impl Transformer for SeparateLodTransformer {
    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject> {
        let mut results = Vec::new();

        for o in &city_objects {
            let feature_ref = match &o.root {
                Value::Feature(f) => f,
                _ => panic!("Root value type must be Feature, but found {:?}", o.root),
            };

            // feature.geometriesから、lodごとに分割して、objectsに追加する
            if let Value::Feature(f) = &o.root {
                if let Some(geometry_ref_list) = &f.geometries {
                    for g in geometry_ref_list.iter() {
                        let mut lods: IndexMap<usize, Vec<GeometryRefEntry>> = IndexMap::new();

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

                        let root = feature_ref.clone();

                        for (_, geometry_refs) in lods.iter() {
                            let feature = Feature {
                                id: root.id.clone(),
                                typename: root.typename.clone(),
                                attributes: root.attributes.clone(),
                                geometries: Some(geometry_refs.clone()),
                            };

                            let obj = CityObject {
                                root: Value::Feature(feature),
                                geometry_store: o.geometry_store.clone(),
                            };

                            results.push(obj);
                        }
                    }
                }
            }
        }

        results
    }
}

struct FilterFeaturesTransformer {}
impl Transformer for FilterFeaturesTransformer {
    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject> {
        todo!();
    }
}

struct FilterAttributesTransformer {}
impl Transformer for FilterAttributesTransformer {
    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject> {
        todo!();
    }
}

struct AttributesTransformer {}
impl Transformer for AttributesTransformer {
    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject> {
        todo!();
    }
}

struct TransformerPipeline {
    transformers: Vec<Box<dyn Transformer>>,
}
impl TransformerPipeline {
    fn new(transformers: Vec<Box<dyn Transformer>>) -> Self {
        Self { transformers }
    }

    fn add(&mut self, transformer: Box<dyn Transformer>) {
        self.transformers.push(transformer);
    }

    fn transform(&self, city_objects: Vec<CityObject>) -> Vec<CityObject> {
        let mut objects = city_objects;
        for transformer in &self.transformers {
            objects = transformer.transform(objects);
        }
        objects
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
        let mut settings = Settings::default();
        settings.load_semantic_parts = false;
        settings.to_tabular = true;
        settings.to_json_string = true;

        let mut transformer_pipeline = TransformerPipeline {
            transformers: Vec::new(),
        };

        transformer_pipeline.add(Box::new(FeatureCollectTransformer {}));
        // if settings.load_semantic_parts {
        //     transformer_pipeline.add(Box::new(SemanticSplitTransformer {}));
        // } else {
        //     transformer_pipeline.add(Box::new(SeparateLodTransformer {}));
        // }
        if settings.to_tabular {
            transformer_pipeline.add(Box::new(FlattenTreeTransformer {}));
        }

        let obj = CityObject {
            root: Value::Feature(toplevel_feature),
            geometry_store: cityobj.geometry_store.clone(),
        };
        let mut objects = transformer_pipeline.transform(vec![obj]);

        // Array・Data・featureは全てJSON文字列に変換するかどうか
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

        if objects.len() > 0 {
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

        println!();

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
