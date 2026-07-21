use hashbrown::HashMap;
use nusamai_citygml::object;
use prost::Message;
use tinymvt::{geometry::GeometryEncoder, tag::TagsEncoder, vector_tile};

use crate::{
    pipeline::{PipelineError, Result},
    sink::vector_tile::{
        geometry::quantize_polygons,
        model::{hash_feature_id, SlicedFeature},
        TileEncoder,
    },
};

use super::tags::convert_properties;

#[derive(Clone, Copy)]
enum EncodingProfile {
    MvtDirectory,
    PmTilesLegacy,
}

pub(crate) struct MvtTileEncoder {
    profile: EncodingProfile,
}

impl MvtTileEncoder {
    pub(crate) fn for_mvt_directory() -> Self {
        Self {
            profile: EncodingProfile::MvtDirectory,
        }
    }

    pub(crate) fn for_pmtiles_legacy() -> Self {
        Self {
            profile: EncodingProfile::PmTilesLegacy,
        }
    }
}

#[derive(Default)]
struct LayerData {
    features: Vec<vector_tile::tile::Feature>,
    tags_encoder: TagsEncoder,
}

impl TileEncoder for MvtTileEncoder {
    fn encode_tile(&self, detail: i32, serialized_features: &[Vec<u8>]) -> Result<Vec<u8>> {
        let mut layers: HashMap<String, LayerData> = HashMap::new();
        let extent = 1_i32 << detail;
        let bincode_config = bincode::config::standard();

        for serialized_feature in serialized_features {
            let (feature, _): (SlicedFeature, _) = bincode::serde::decode_from_slice(
                serialized_feature,
                bincode_config,
            )
            .map_err(|error| {
                PipelineError::Other(format!("Failed to deserialize a sliced feature: {error:?}"))
            })?;

            let mut geometry_encoder = GeometryEncoder::new();
            for polygon in quantize_polygons(&feature.geometry, extent) {
                geometry_encoder.add_ring(
                    polygon
                        .exterior
                        .into_iter()
                        .map(|[x, y]| [x as i16, y as i16]),
                );
                for interior in polygon.interiors {
                    geometry_encoder
                        .add_ring(interior.into_iter().map(|[x, y]| [x as i16, y as i16]));
                }
            }
            let geometry = geometry_encoder.into_vec();
            if geometry.is_empty() {
                continue;
            }

            match &feature.properties {
                object::Value::Object(object) => {
                    let typename: &str = object.typename.as_ref();
                    let layer = layers.entry_ref(typename).or_default();
                    for (key, value) in &object.attributes {
                        convert_properties(&mut layer.tags_encoder, key, value);
                    }

                    let id = match self.profile {
                        EncodingProfile::MvtDirectory => {
                            object.stereotype.id().map(hash_feature_id)
                        }
                        EncodingProfile::PmTilesLegacy => None,
                    };
                    layer.features.push(vector_tile::tile::Feature {
                        id,
                        tags: layer.tags_encoder.take_tags(),
                        r#type: Some(vector_tile::tile::GeomType::Polygon as i32),
                        geometry,
                    });
                }
                _ if matches!(self.profile, EncodingProfile::MvtDirectory) => {
                    let layer = layers.entry_ref("Unknown").or_default();
                    layer.features.push(vector_tile::tile::Feature {
                        id: None,
                        tags: layer.tags_encoder.take_tags(),
                        r#type: Some(vector_tile::tile::GeomType::Polygon as i32),
                        geometry,
                    });
                }
                _ => {}
            }
        }

        let layers = layers
            .into_iter()
            .filter_map(|(name, layer_data)| {
                if layer_data.features.is_empty() {
                    return None;
                }
                let (keys, values) = layer_data.tags_encoder.into_keys_and_values();
                Some(vector_tile::tile::Layer {
                    version: 2,
                    name,
                    features: layer_data.features,
                    keys,
                    values,
                    extent: Some(extent as u32),
                })
            })
            .collect();

        Ok(vector_tile::Tile { layers }.encode_to_vec())
    }
}
