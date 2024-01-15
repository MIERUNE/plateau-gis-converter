use ahash::RandomState;
use indexmap::IndexMap;

use nusamai_citygml::object::{CityObject, Data, Feature, Map};
use nusamai_citygml::Value;

// 以下、仮実装
pub struct Settings {
    load_semantic_parts: bool,
    target_lods: Vec<bool>,
    load_lower_lods: bool,
    load_upper_lods: bool,
}

pub trait ObjectSeparator {
    fn separate(&self, cityobj: &CityObject) -> Vec<CityObject>;
}

pub struct SemanticObjectSeparator {
    pub settings: Option<Settings>,
}

impl ObjectSeparator for SemanticObjectSeparator {
    fn separate(&self, cityobj: &CityObject) -> Vec<CityObject> {
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

        let mut primitives: IndexMap<String, Value, RandomState> =
            IndexMap::with_hasher(RandomState::new());
        let mut features: IndexMap<String, Vec<Feature>> = IndexMap::new();
        let mut data_list: IndexMap<String, Vec<Data>> = IndexMap::new();
        let mut other_layer_data: IndexMap<String, Vec<Data>> = IndexMap::new();
        let mut other_layer_attributes: IndexMap<String, Vec<IndexMap<String, Value>>> =
            IndexMap::new();

        for (key, value) in toplevel_feature.attributes.iter() {
            match value {
                Value::Array(_) | Value::Feature(_) | Value::Data(_) => {}
                _ => {
                    primitives.insert(key.clone(), value.clone());
                }
            }
        }

        for (key, value) in toplevel_feature.attributes.iter() {
            if let Value::Data(d) = value {
                data_list.insert(key.clone(), vec![d.clone()]);
            }
        }

        for (key, value) in toplevel_feature.attributes.iter() {
            if let Value::Feature(f) = value {
                features.insert(key.clone(), vec![f.clone()]);
            }
        }

        for (key, value) in toplevel_feature.attributes.iter() {
            if let Value::Array(a) = value {
                for v in a.iter() {
                    match v {
                        Value::Data(d) => {
                            if other_layer_data.contains_key(key) {
                                other_layer_data.get_mut(key).unwrap().push(d.clone());
                            } else {
                                other_layer_data.insert(key.clone(), vec![d.clone()]);
                            }
                        }
                        Value::Feature(f) => {
                            features.insert(key.clone(), vec![f.clone()]);
                        }
                        _ => {
                            primitives.insert(key.clone(), value.clone());
                        }
                    }
                }
            }
        }

        for (key, value) in data_list.iter() {
            for d in value.iter() {
                let attributes = &d.attributes;
                for (k, v) in attributes.iter() {
                    primitives.insert(k.clone(), v.clone());
                }
            }
        }

        for (key, value) in other_layer_data.iter() {
            let mut other: IndexMap<String, Value> = IndexMap::new();
            for d in value.iter() {
                let attributes = &d.attributes;
                for (k, v) in attributes.iter() {
                    other.insert(k.clone(), v.clone());
                }
            }
            if other_layer_attributes.contains_key(key) {
                other_layer_attributes.get_mut(key).unwrap().push(other);
            } else {
                other_layer_attributes.insert(key.clone(), vec![other]);
            }
        }

        // 何かしらの設定ファイルを受け取り、以下のような設定ができると嬉しい
        // ・Arrayはjson文字列に変換する
        // ・Arrayの中身がDataなら、別のレイヤーにする
        // ・Arrayの中身がFeatureなら、別のレイヤーにする
        // ・特定の属性のみ形状を変換する
        // 最終的には上記のように改修されたVec<CityObject>を返す

        let objects = self.jsonify(vec![cityobj]);

        // CityObjectを大量に作成していくために、コピーしたデータを用意しておく
        // パフォーマンスなどは一旦考えない
        let geometry_store = cityobj.geometries.clone();
        let toplevel_attributes = toplevel_feature.attributes.clone();
        let toplevel_geometries = toplevel_feature.geometries.clone();

        // まずはtoplevel_feature.geometriesの分だけCityObjectを作成
        // 最小のLODのみを取得するオプションで必要
        if let Some(geometries) = &toplevel_feature.geometries {
            let attributes = &toplevel_feature.attributes;
            for g in geometries {
                let feature = Feature {
                    id: parent_gml_id.clone(),
                    typename: typename.clone(),
                    attributes: attributes.clone(),
                    geometries: Some(vec![g.clone()]),
                };
                let city_object = CityObject {
                    root: Value::Feature(feature),
                    geometries: cityobj.geometries.clone(),
                };
                objects.push(city_object);
            }
        }

        // ArrayをJSON文字列に変換してVec<CityObject>を返す

        objects
    }
}

impl SemanticObjectSeparator {
    fn jsonify(&self, objects: Vec<&CityObject>) -> Vec<&CityObject> {
        let mut jsonified_objects: Vec<CityObject> = Vec::new();

        objects
    }

    // fn attribute_parser(&self, feature: &Feature, gml_id: String) {
    //     let mut primitive_attributes: IndexMap<String, Value> = IndexMap::new();
    //     let mut features: IndexMap<String, Vec<Feature>> = IndexMap::new();
    //     let mut data_list: IndexMap<String, Vec<Data>> = IndexMap::new();
    //     let mut other_layer_data: IndexMap<String, Vec<Data>> = IndexMap::new();

    //     // 一旦、割り切って実装
    //     // オブションは後でつけるなりするとして
    //     // 仮実装としてとりあえず、Arrayのフィールドがあって、尚且つDataなら、別のレイヤーにする
    //     // それ以外はなるべく、primitive_attributesに格納する
    //     // featureは、featuresに格納する

    //     // まずはprimitive_attributesを抽出
    //     for (key, value) in feature.attributes.iter() {
    //         match value {
    //             Value::Array(_) | Value::Feature(_) | Value::Data(_) => {}
    //             _ => {
    //                 primitive_attributes.insert(key.clone(), value.clone());
    //             }
    //         }
    //     }

    //     // トップレベルのDataを抽出
    //     for (key, value) in feature.attributes.iter() {
    //         if let Value::Data(d) = value {
    //             data_list.insert(key.clone(), vec![d.clone()]);
    //         }
    //     }

    //     // トップレベルのFeatureを抽出
    //     for (key, value) in feature.attributes.iter() {
    //         if let Value::Feature(f) = value {
    //             features.insert(key.clone(), vec![f.clone()]);
    //         }
    //     }

    //     // Arrayを抽出
    //     // 特に、Arrayの取り扱いは色々あるが、一旦はArrayの中身がDataなら、別のレイヤーにするつもりで実装
    //     // featureもセマンティックごとに分割するつもりで、featuresに格納する
    //     for (key, value) in feature.attributes.iter() {
    //         if let Value::Array(a) = value {
    //             for v in a.iter() {
    //                 match v {
    //                     Value::Data(d) => {
    //                         if other_layer_data.contains_key(key) {
    //                             other_layer_data.get_mut(key).unwrap().push(d.clone());
    //                         } else {
    //                             other_layer_data.insert(key.clone(), vec![d.clone()]);
    //                         }
    //                     }
    //                     Value::Feature(f) => {
    //                         features.insert(key.clone(), vec![f.clone()]);
    //                     }
    //                     _ => {
    //                         primitive_attributes.insert(key.clone(), value.clone());
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     // data_list内のDataの属性を抽出し、primitive_attributesに格納
    //     for (key, value) in data_list.iter() {
    //         for d in value.iter() {
    //             let attributes = &d.attributes;
    //             for (k, v) in attributes.iter() {
    //                 // これ以上の入れ子を考えるのがちょっと大変なので、Data内の属性は全てprimitive_attributesに格納してしまう
    //                 primitive_attributes.insert(k.clone(), v.clone());
    //             }
    //         }
    //     }

    //     // other_layer_data内のDataの属性を抽出し、キー名ごとにattributeを取り出してフラット化
    //     let mut other_layer_attributes: IndexMap<String, Vec<IndexMap<String, Value>>> =
    //         IndexMap::new();
    //     for (key, value) in other_layer_data.iter() {
    //         let mut other: IndexMap<String, Value> = IndexMap::new();
    //         for d in value.iter() {
    //             let attributes = &d.attributes;
    //             for (k, v) in attributes.iter() {
    //                 // これ以上の入れ子を考えるのがちょっと大変なので、Data内の属性は全てprimitive_attributesに格納してしまう
    //                 other.insert(k.clone(), v.clone());
    //             }
    //         }
    //         if other_layer_attributes.contains_key(key) {
    //             other_layer_attributes.get_mut(key).unwrap().push(other);
    //         } else {
    //             other_layer_attributes.insert(key.clone(), vec![other]);
    //         }
    //     }

    //     println!("primitive_attributes: {:?}", primitive_attributes);
    //     println!("features: {:?}", features);
    //     println!("other_layer_data: {:?}", other_layer_data);
    //     println!("other_layer_attributes: ");
    //     for (key, value) in other_layer_attributes.iter() {
    //         println!(" key: {:?}", key);
    //         serde_json::to_writer_pretty(std::io::stdout(), value).unwrap();
    //     }

    //     // ここまでで、primitive_attributesにData、とData以外の属性が格納されたArrayと、それ以外のValueが集められたはず
    //     let primitive_file =
    //         std::fs::File::create("/Users/satoru/Downloads/output/primitive.json").unwrap();
    //     let primitive_writer = std::io::BufWriter::new(primitive_file);
    //     serde_json::to_writer_pretty(primitive_writer, &primitive_attributes).unwrap();

    //     let features_file =
    //         std::fs::File::create("/Users/satoru/Downloads/output/features.json").unwrap();
    //     let features_writer = std::io::BufWriter::new(features_file);
    //     serde_json::to_writer_pretty(features_writer, &features).unwrap();

    //     let other_layer_attributes_file =
    //         std::fs::File::create("/Users/satoru/Downloads/output/other_layer_attributes.json")
    //             .unwrap();
    //     let others_writer = std::io::BufWriter::new(other_layer_attributes_file);
    //     serde_json::to_writer_pretty(others_writer, &other_layer_attributes).unwrap();

    //     // 属性抽出の概要はなんとなく整理できたので、CityObjectを返すように変更していく、もしくはどんどんCityObjectを作っていくようにする

    //     // 何かしらの設定ファイルを受け取り、以下のような設定ができると嬉しい
    //     // ・Arrayはjson文字列に変換する
    //     // ・Arrayの中身がDataなら、別のレイヤーにする
    //     // ・Arrayの中身がFeatureなら、別のレイヤーにする
    //     // ・特定の属性のみ形状を変換する
    //     // 最終的には上記のように改修されたVec<CityObject>を返す

    //     // Valueに対して個別に上記のような処理を行うアルゴリズムを作っていってStrategyパターンのようなイメージで適用していく？

    //     println!();
    // }
}
