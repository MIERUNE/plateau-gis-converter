use hashbrown::HashMap;
use nusamai_citygml::object;
use prost::Message;
use tinymvt::{geometry::GeometryEncoder, tag::TagsEncoder, vector_tile};

use crate::{
    pipeline::{PipelineError, Result},
    sink::vector_tile::{
        feature::{hash_feature_id, SlicedFeature, SlicedGeometry},
        geometry::{quantize_linestrings, quantize_points, quantize_polygons},
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
    fn encode_tile(&self, detail: i32, serialized_features: &[Vec<u8>]) -> Result<Option<Vec<u8>>> {
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

            let Some(encoded_geometry) = encode_geometry(&feature.geometry, extent) else {
                continue;
            };

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
                        r#type: Some(encoded_geometry.geometry_type as i32),
                        geometry: encoded_geometry.commands,
                    });
                }
                _ if matches!(self.profile, EncodingProfile::MvtDirectory) => {
                    let layer = layers.entry_ref("Unknown").or_default();
                    layer.features.push(vector_tile::tile::Feature {
                        id: None,
                        tags: layer.tags_encoder.take_tags(),
                        r#type: Some(encoded_geometry.geometry_type as i32),
                        geometry: encoded_geometry.commands,
                    });
                }
                _ => {}
            }
        }

        let layers: Vec<_> = layers
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

        if layers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(vector_tile::Tile { layers }.encode_to_vec()))
        }
    }
}

struct EncodedGeometry {
    geometry_type: vector_tile::tile::GeomType,
    commands: Vec<u32>,
}

fn encode_geometry(geometry: &SlicedGeometry, extent: i32) -> Option<EncodedGeometry> {
    let (geometry_type, commands) = match geometry {
        SlicedGeometry::Point(points) => {
            let points = quantize_points(points, extent);
            if points.is_empty() {
                return None;
            }
            let mut encoder = GeometryEncoder::new();
            encoder.add_points(points);
            (vector_tile::tile::GeomType::Point, encoder.into_vec())
        }
        SlicedGeometry::LineString(lines) => {
            let lines = quantize_linestrings(lines, extent);
            if lines.is_empty() {
                return None;
            }
            let mut encoder = GeometryEncoder::new();
            for line in lines {
                encoder.add_linestring(line);
            }
            (vector_tile::tile::GeomType::Linestring, encoder.into_vec())
        }
        SlicedGeometry::Polygon(polygons) => {
            let polygons = quantize_polygons(polygons, extent);
            if polygons.is_empty() {
                return None;
            }
            let mut encoder = GeometryEncoder::new();
            for polygon in polygons {
                encoder.add_ring(polygon.exterior);
                for interior in polygon.interiors {
                    encoder.add_ring(interior);
                }
            }
            (vector_tile::tile::GeomType::Polygon, encoder.into_vec())
        }
    };

    (!commands.is_empty()).then_some(EncodedGeometry {
        geometry_type,
        commands,
    })
}
