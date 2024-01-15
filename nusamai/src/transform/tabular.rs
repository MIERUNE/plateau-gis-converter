use indexmap::IndexMap;

use nusamai_citygml::object::{CityObject, Data, Feature};
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

    fn attribute_parser(&self, feature: &Feature, gml_id: String) -> Vec<Feature> {
        let mut primitive_attributes: IndexMap<String, Value> = IndexMap::new();
        let mut features: Vec<Feature> = Vec::new();
        let mut data_list: Vec<Data> = Vec::new();
        let mut array_list: Vec<Vec<Value>> = Vec::new();

        // フラット化されたdataとfeatureを取得
        // それ以外の情報も取得
        for (key, value) in feature.attributes.iter() {
            match value {
                Value::Array(a) => {
                    array_list.push(a.clone());
                }
                Value::Data(d) => {
                    data_list.push(d.clone());
                }
                Value::Feature(f) => {
                    features.push(f.clone());
                }
                _ => {
                    primitive_attributes.insert(key.clone(), value.clone());
                }
            }

            // let d = self.extract_data(value.clone());
            // data_list.extend(d);

            // let f = self.extract_features(value.clone());
            // features.extend(f);
        }

        // オプションなし→全ての最小のLODを取得し、一部の属性はテーブルを分割
        // ★ルール
        // ・最も単純なLODのみが読み込まれる
        // トップレベル.geometriesの中から最小LODのもの？
        // と思ったが、QGISプラグインはLOD0と1が読み込まれたら1が表示されていた
        // これがなければ、個別に読み取りに行く？
        // ・Dataは別テーブルに分割
        // Dataはattributesを展開して、DataLayerに突っ込む
        // ・FeatureはFeatureをまとめて1つの地物にする
        // トップレベルのtypenameが異なるなら、別のレイヤーにする

        println!("primitive_attributes: {:?}", primitive_attributes);
        println!("data_list: {:?}", data_list);
        println!("features: {:?}", features);
        println!("array_list: {:?}", array_list);

        // featuresが無い→attribute内に子要素がなかった→トップレベルのジオメトリを利用

        println!();

        features
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

        let features = &self.attribute_parser(toplevel_feature, parent_gml_id.clone().unwrap());

        // テスト用の出力
        let file = std::fs::File::create("/Users/satoru/Downloads/output/test.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &cityobj.root).unwrap();
        todo!("CityObject to Vec<Layer>")
    }
}
