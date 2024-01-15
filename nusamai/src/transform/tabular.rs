use indexmap::IndexMap;

use nusamai_citygml::object::{CityObject, Data, Feature};
use nusamai_citygml::Value;
use rayon::array;

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

    fn extract_data(&self, value: Value) -> Vec<Data> {
        match value {
            Value::Array(vec) => vec
                .iter()
                .flat_map(|v: &Value| self.extract_data(v.clone()))
                .collect(),
            Value::Data(data) => {
                vec![data.clone()]
            }
            _ => Vec::new(),
        }
    }

    fn attribute_parser(&self, feature: &Feature, gml_id: String) -> () {
        let mut primitive_attributes: IndexMap<String, Value> = IndexMap::new();
        let mut features: IndexMap<String, Vec<Feature>> = IndexMap::new();
        let mut data_list: IndexMap<String, Vec<Data>> = IndexMap::new();
        let mut other_layer_data: IndexMap<String, Vec<Value>> = IndexMap::new();

        // 一旦、割り切って実装
        // オブションは後でつけるなりするとして
        // 仮実装としてとりあえず、Arrayのフィールドがあって、尚且つDataなら、別のレイヤーにする
        // それ以外はなるべく、primitive_attributesに格納する
        // featureは、featuresに格納する
        for (key, value) in feature.attributes.iter() {
            match value {
                Value::Array(a) => {
                    for v in a.iter() {
                        match v {
                            Value::Data(_) => {
                                if other_layer_data.contains_key(key) {
                                    other_layer_data.get_mut(key).unwrap().push(v.clone());
                                } else {
                                    other_layer_data.insert(key.clone(), vec![v.clone()]);
                                }
                            }
                            Value::Feature(f) => {
                                features.insert(key.clone(), vec![f.clone()]);
                            }
                            _ => {
                                primitive_attributes.insert(key.clone(), value.clone());
                            }
                        }
                    }
                }
                Value::Data(d) => {
                    data_list.insert(key.clone(), vec![d.clone()]);
                }
                Value::Feature(f) => {
                    features.insert(key.clone(), vec![f.clone()]);
                }
                _ => {
                    primitive_attributes.insert(key.clone(), value.clone());
                }
            }
        }

        for (key, value) in data_list.iter() {
            for v in value.iter() {
                primitive_attributes.insert(key.clone(), Value::Data(v.clone()));
            }
        }

        println!("primitive_attributes: {:?}", primitive_attributes);
        println!("features: {:?}", features);
        println!("other_layer_data: ");
        for (key, value) in other_layer_data.iter() {
            println!(" key: {:?}", key);
            serde_json::to_writer_pretty(std::io::stdout(), value).unwrap();
        }

        // ここまでで、primitive_attributesにData、とData以外の属性が格納されたArrayと、それ以外のValueが集められたはず
        let primitive_file =
            std::fs::File::create("/Users/satoru/Downloads/output/primitive.json").unwrap();
        let primitive_writer = std::io::BufWriter::new(primitive_file);
        serde_json::to_writer_pretty(primitive_writer, &primitive_attributes).unwrap();

        let features_file =
            std::fs::File::create("/Users/satoru/Downloads/output/features.json").unwrap();
        let features_writer = std::io::BufWriter::new(features_file);
        serde_json::to_writer_pretty(features_writer, &features).unwrap();

        let other_layer_data_file =
            std::fs::File::create("/Users/satoru/Downloads/output/other_layer_data.json").unwrap();
        let other_layer_data_writer = std::io::BufWriter::new(other_layer_data_file);
        serde_json::to_writer_pretty(other_layer_data_writer, &other_layer_data).unwrap();

        println!();
    }

    pub fn to_tabular(&self, cityobj: &CityObject) -> Vec<Layer> {
        let toplevel_feature: &Feature = match &cityobj.root {
            Value::Feature(f) => f,
            _ => panic!(
                "Root value type must be Feature, but found {:?}",
                cityobj.root
            ),
        };
        let parent_gml_id = &toplevel_feature.id;
        let typename = &toplevel_feature.typename;
        println!("{:?}, {:?}", parent_gml_id, typename);

        &self.attribute_parser(toplevel_feature, parent_gml_id.clone().unwrap());

        // テスト用の出力
        let file = std::fs::File::create("/Users/satoru/Downloads/output/test.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &cityobj.root).unwrap();
        todo!("CityObject to Vec<Layer>")
    }
}
