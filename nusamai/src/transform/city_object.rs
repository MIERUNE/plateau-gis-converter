use ahash::{HashMap, RandomState};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use nusamai_citygml::{
    object::{CityObject, Data, Feature},
    GeometryRefEntry, GeometryStore, Value,
};

// utility for child element collection
fn extract_features(value: &Value) -> Vec<Feature> {
    match value {
        Value::Array(vec) => vec.iter().flat_map(extract_features).collect(),
        Value::Feature(feature) => {
            vec![feature.clone()]
        }
        _ => Vec::new(),
    }
}

fn extract_data(value: &Value) -> Vec<Data> {
    match value {
        Value::Array(vec) => vec.iter().flat_map(extract_data).collect(),
        Value::Data(data) => {
            vec![data.clone()]
        }
        _ => Vec::new(),
    }
}

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

fn extract_array(value: &Value) -> Vec<Value> {
    match value {
        Value::Array(vec) => vec.clone(),
        _ => Vec::new(),
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
struct Param {
    name: String,
    type_name: String,
    remove: bool,
}

struct FeatureCollector {}
impl FeatureCollector {
    fn collect(&self, city_object: CityObject) -> CityObject {
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

        CityObject {
            root: Value::Feature(feature),
            geometry_store: city_object.geometry_store.clone(),
        }
    }
}

trait Transformer {
    fn transform(&self, city_object: CityObject) -> Vec<CityObject>;
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
                geometry_store: GeometryStore {
                    ..Default::default()
                },
            };

            results.push(obj);
        }

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

trait Traversal {
    fn traverse(&self, city_object: CityObject, mappings: &Mappings) -> CityObject;
}

struct JsonStringTraversal {}
impl Traversal for JsonStringTraversal {
    fn traverse(&self, city_object: CityObject, mappings: &Mappings) -> CityObject {
        let root_typename = match &city_object.root {
            Value::Feature(f) => &f.typename,
            Value::Data(d) => &d.typename,
            _ => panic!(
                "Root value type must be Feature or Data, but found {:?}",
                city_object.root
            ),
        };

        let targets = mappings.types.get(root_typename.as_ref());
        if let Some(targets) = targets {
            let mut attributes = IndexMap::with_hasher(RandomState::new());
            if let Value::Feature(f) = &city_object.root {
                for (key, value) in f.attributes.iter() {
                    let param = targets.get(key);

                    if let Some(p) = param {
                        if p.remove {
                            continue;
                        };

                        let key = if p.name.is_empty() { key } else { &p.name };

                        let value = if p.type_name == "string" {
                            match value {
                                Value::Array(a) => {
                                    let array = a
                                        .iter()
                                        .map(|v| {
                                            if let Value::String(s) = v {
                                                s.clone()
                                            } else {
                                                panic!("Array must be String, but found {:?}", v)
                                            }
                                        })
                                        .collect::<Vec<String>>()
                                        .join(",");
                                    Value::String(array)
                                }
                                Value::Data(d) => {
                                    let json_data = serde_json::to_string(&d.attributes).unwrap();
                                    Value::String(json_data)
                                }
                                Value::Feature(f) => {
                                    let json_feature =
                                        serde_json::to_string(&f.attributes).unwrap();
                                    Value::String(json_feature)
                                }
                                _ => value.clone(),
                            }
                        } else if p.type_name == "json" {
                            match value {
                                Value::Array(a) => {
                                    let json_array = serde_json::to_string(a).unwrap();
                                    Value::String(json_array)
                                }
                                Value::Data(d) => {
                                    let json_data = serde_json::to_string(&d.attributes).unwrap();
                                    Value::String(json_data)
                                }
                                Value::Feature(f) => {
                                    let json_feature =
                                        serde_json::to_string(&f.attributes).unwrap();
                                    Value::String(json_feature)
                                }
                                _ => value.clone(),
                            }
                        } else {
                            value.clone()
                        };
                        attributes.insert(key.clone(), value.clone());
                    };
                    attributes.insert(key.clone(), value.clone());
                }

                let feature = Feature {
                    id: f.id.clone(),
                    typename: f.typename.clone(),
                    attributes,
                    geometries: f.geometries.clone(),
                };

                let obj = CityObject {
                    root: Value::Feature(feature),
                    geometry_store: city_object.geometry_store,
                };
                return obj;
            }
        }
        city_object
    }
}

struct ObjectTraversalPipeline {
    traversals: Vec<Box<dyn Traversal>>,
    mappings: Mappings,
}
impl ObjectTraversalPipeline {
    fn add(&mut self, traversal: Box<dyn Traversal>) {
        self.traversals.push(traversal);
    }

    fn traverse(&self, city_object: CityObject) -> CityObject {
        let mut obj = city_object;
        for traversal in &self.traversals {
            obj = traversal.traverse(obj, &self.mappings);
        }
        obj
    }
}

#[derive(Debug, Default)]
pub struct ObjectTransformer {}

impl ObjectTransformer {
    pub fn transform(&self, cityobj: &CityObject) -> Vsec<CityObject> {
        let toplevel_feature = match &cityobj.root {
            Value::Feature(f) => f.clone(),
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                cityobj.root
            ),
        };

        // 一旦コピーを作成
        let obj = CityObject {
            root: Value::Feature(toplevel_feature),
            geometry_store: cityobj.geometry_store.clone(),
        };

        // attributes内の子要素（feature）を収集して、トップレベルに引き上げる
        let obj = FeatureCollector {}.collect(obj);

        // 仮にJSONファイルから設定を読み込む
        // マッピングルールのパスを読み込めば良い？
        // sinkから渡したい
        let file = std::fs::File::open("/Users/satoru/Downloads/output/mappings.json").unwrap();
        let reader = std::io::BufReader::new(file);
        let mappings: Mappings = serde_json::from_reader(reader).unwrap();

        let settings = mappings.settings.clone();

        // LODごとの分離などの、CityObject->Vec<CityObject>を行うTransformer
        let mut transformer_pipeline = TransformerPipeline {
            transformers: Vec::new(),
        };
        if settings.to_tabular {
            transformer_pipeline.add(Box::new(DataCollectTransformer {}));
        }
        if settings.load_semantic_parts {
            transformer_pipeline.add(Box::new(SemanticSplitTransformer {}));
        } else {
            transformer_pipeline.add(Box::new(SeparateLodTransformer {}));
        }
        let objects = transformer_pipeline.transform(obj);

        // 属性の変換などの、CityObject->CityObjectを行うTraversal
        let mut traversal_pipeline = ObjectTraversalPipeline {
            traversals: Vec::new(),
            mappings,
        };
        if settings.to_json_string {
            traversal_pipeline.add(Box::new(JsonStringTraversal {}));
        }
        let objects: Vec<CityObject> = objects
            .into_iter()
            .map(|o| traversal_pipeline.traverse(o))
            .collect();

        let file =
            std::fs::File::create("/Users/satoru/Downloads/output/output_object.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &objects).unwrap();

        objects
    }
}
