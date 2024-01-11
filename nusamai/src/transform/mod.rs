use ahash::random_state::RandomState;
use indexmap::IndexMap;

use crate::pipeline::{Feedback, Parcel, Sender, TransformError, Transformer};

use nusamai_citygml::object::{CityObject, Feature, Map};
use nusamai_citygml::Value;
use nusamai_projection::crs::*;
use nusamai_projection::vshift::JGD2011ToWGS84;

pub struct DummyTransformer {
    jgd2wgs: JGD2011ToWGS84,
}

impl Default for DummyTransformer {
    fn default() -> Self {
        Self {
            jgd2wgs: JGD2011ToWGS84::from_embedded_model(),
        }
    }
}

impl Transformer for DummyTransformer {
    fn transform(
        &self,
        mut parcel: Parcel,
        downstream: &Sender,
        feedback: &Feedback,
    ) -> Result<(), TransformError> {
        // 仮実装
        parcel.cityobj.geometries.vertices.iter_mut().for_each(|v| {
            // Swap x and y (lat, lng -> lng, lat)
            let (lng, lat, height) = (v[1], v[0], v[2]);

            // JGD2011 to WGS 84 (elevation to ellipsoidal height)
            (v[0], v[1], v[2]) = self.jgd2wgs.convert(lng, lat, height);
        });

        // Ensure that the original CRS is JGD2011 and the new CRS is WGS 84
        assert_eq!(parcel.cityobj.geometries.epsg, EPSG_JGD2011_GEOGRAPHIC_3D);
        parcel.cityobj.geometries.epsg = EPSG_WGS84_GEOGRAPHIC_3D;

        // Send to the next stage
        if downstream.send(parcel).is_err() {
            feedback.cancel();
        };
        Ok(())
    }
}

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

pub struct ObjectController {
    pub settings: Option<Settings>,
}

impl ObjectController {
    pub fn to_tabular(&self, cityobj: &CityObject) -> Vec<Layer> {
        let manager = LayerManager { layers: None };

        let feature: &Feature = match &cityobj.root {
            Value::Feature(f) => f,
            _ => todo!(),
        };

        let features = &self.parse_feature(feature);
        println!("{:?}", features);

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
