use ahash::random_state::RandomState;
use indexmap::IndexMap;

use nusamai_citygml::object::{CityObject, Feature, Map};
use nusamai_citygml::Value;

// 以下、仮実装
pub struct Layer {
    id: String,
    layer_name: String,
    objects: Vec<CityObject>,
}

pub struct LayerManager {
    layers: Option<Vec<Layer>>,
}

impl LayerManager {
    pub fn merge(&mut self) {
        // 同じレイヤー名のレイヤーをマージする
        todo!()
    }
}

pub struct Settings {
    load_semantic_parts: bool,
    target_lods: Vec<bool>,
    load_lower_lods: bool,
    load_upper_lods: bool,
}

pub struct ObjectSeparator {
    pub settings: Option<Settings>,
}

impl ObjectSeparator {
    fn extract_features(&self, value: Value) -> Vec<Feature> {
        match value {
            Value::Array(vec) => vec
                .iter()
                .flat_map(|v| self.extract_features(v.clone()))
                .collect(),
            Value::Feature(feature) => {
                vec![feature.clone()]
            }
            _ => Vec::new(),
        }
    }

    fn attribute_parser(&self, feature: &Feature, gml_id: String) -> Vec<Feature> {
        let mut features: Vec<Feature> = Vec::new();

        for (key, value) in feature.attributes.iter() {
            let f = self.extract_features(value.clone());
            features.extend(f);
        }

        features
    }

    pub fn to_tabular(&self, cityobj: &CityObject) -> Vec<Layer> {
        let manager = LayerManager { layers: None };

        let toplevel_feature: &Feature = match &cityobj.root {
            Value::Feature(f) => f,
            _ => todo!(),
        };
        let parent_gml_id = &toplevel_feature.id;
        println!("{:?}", parent_gml_id);

        let features = &self.attribute_parser(toplevel_feature, parent_gml_id.clone().unwrap());

        // テスト用の出力
        let file = std::fs::File::create("/Users/satoru/Downloads/output/test.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &cityobj.root).unwrap();
        todo!("CityObject to Vec<Layer>")
    }

    fn parse_feature(&self, feature: &Feature) -> Vec<Feature> {
        let id = &feature.id;
        let typename = &feature.typename;
        let attributes = &feature.attributes;

        let geometries = feature.geometries.as_ref().unwrap();

        // 空のattributesを作成
        let mut a: Map = IndexMap::with_hasher(RandomState::new());

        // attributesの中身を見て、Value::Array, Value::Data, Value::Feature以外のものをresultsに入れる
        for (k, v) in attributes.iter() {
            if !matches!(v, Value::Array(_) | Value::Data(_) | Value::Feature(_)) {
                a.insert(k.clone(), v.clone());
            }
        }

        let mut features = Vec::new();

        for geometry in geometries {
            features.push(Feature {
                typename: typename.clone(),
                id: id.clone(),
                attributes: a.clone(),
                geometries: Some(vec![geometry.clone()]),
            });
        }

        features
    }
}

fn parse_array() {}

fn parse_data() {}

// fn parse_cityobj(cityobj: &CityObject) -> Vec<Layer> {
//     let root = &cityobj.root;

//     let features = match root {
//         Value::Feature(f) => parse_feature(f),
//         _ => todo!(),
//     };
//     println!("{:?}", features);

//     let attributes = match root {
//         Value::Array(a) => {
//             todo!();
//         }
//         Value::Data(d) => {
//             todo!();
//         }
//         _ => todo!(),
//     };
// }
