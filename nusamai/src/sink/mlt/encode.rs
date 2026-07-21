use std::collections::BTreeMap;

use mlt_core::{
    encoder::EncoderConfig,
    geo_types::{Coord, Geometry, LineString, MultiPolygon, Polygon},
    PropKind, PropValue, TileLayer,
};
use nusamai_citygml::{
    object,
    schema::{Schema, TypeDef, TypeRef},
    Value as CityGmlValue,
};

use crate::{
    pipeline::{PipelineError, Result},
    sink::vector_tile::{
        geometry::{quantize_polygons, QuantizedPolygon},
        model::{hash_feature_id, SlicedFeature},
        TileEncoder,
    },
};

pub(crate) struct MltTileEncoder<'a> {
    schema: &'a Schema,
}

impl<'a> MltTileEncoder<'a> {
    pub(crate) fn new(schema: &'a Schema) -> Self {
        Self { schema }
    }
}

#[derive(Default)]
struct LayerData {
    features: Vec<MltFeature>,
    property_kinds: BTreeMap<String, PropKind>,
}

struct MltFeature {
    geometry: Geometry<i32>,
    id: Option<u64>,
    properties: BTreeMap<String, MltPropertyValue>,
}

#[derive(Clone)]
enum MltPropertyValue {
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    Str(String),
}

impl MltPropertyValue {
    fn to_prop_value(&self, kind: PropKind) -> PropValue {
        match kind {
            PropKind::Bool => match self {
                Self::Bool(value) => PropValue::Bool(Some(*value)),
                _ => PropValue::Bool(None),
            },
            PropKind::I64 => match self {
                Self::I64(value) => PropValue::I64(Some(*value)),
                _ => PropValue::I64(None),
            },
            PropKind::U64 => match self {
                Self::U64(value) => PropValue::U64(Some(*value)),
                _ => PropValue::U64(None),
            },
            PropKind::F64 => match self {
                Self::F64(value) => PropValue::F64(Some(*value)),
                Self::I64(value) => PropValue::F64(exact_i64_to_f64(*value)),
                Self::U64(value) => PropValue::F64(exact_u64_to_f64(*value)),
                _ => PropValue::F64(None),
            },
            PropKind::Str => PropValue::Str(Some(self.as_string())),
            _ => PropValue::null(kind),
        }
    }

    fn as_string(&self) -> String {
        match self {
            Self::Bool(value) => value.to_string(),
            Self::I64(value) => value.to_string(),
            Self::U64(value) => value.to_string(),
            Self::F64(value) => value.to_string(),
            Self::Str(value) => value.clone(),
        }
    }
}

impl TileEncoder for MltTileEncoder<'_> {
    fn encode_tile(&self, detail: i32, serialized_features: &[Vec<u8>]) -> Result<Vec<u8>> {
        let mut layers: BTreeMap<String, LayerData> = BTreeMap::new();
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

            let Some(geometry) = make_geometry(&feature.geometry, extent) else {
                continue;
            };

            if let object::Value::Object(object) = &feature.properties {
                let typename: &str = object.typename.as_ref();
                let layer = layers.entry(typename.to_string()).or_default();

                let mut properties = BTreeMap::new();
                for (key, value) in &object.attributes {
                    let Some(value) = convert_property_value(value) else {
                        continue;
                    };
                    layer
                        .property_kinds
                        .entry(key.clone())
                        .or_insert_with(|| schema_property_kind(self.schema, typename, key));
                    properties.insert(key.clone(), value);
                }

                layer.features.push(MltFeature {
                    geometry,
                    id: object.stereotype.id().map(hash_feature_id),
                    properties,
                });
            } else {
                layers
                    .entry("Unknown".to_string())
                    .or_default()
                    .features
                    .push(MltFeature {
                        geometry,
                        id: None,
                        properties: BTreeMap::new(),
                    });
            }
        }

        let mut tile = Vec::new();
        for (name, layer_data) in layers {
            if layer_data.features.is_empty() {
                continue;
            }
            tile.extend(encode_layer(name, layer_data, extent as u32)?);
        }
        Ok(tile)
    }
}

fn encode_layer(name: String, layer_data: LayerData, extent: u32) -> Result<Vec<u8>> {
    let mut builder = TileLayer::builder(name, extent)
        .map_err(|error| PipelineError::Other(format!("Failed to create MLT layer: {error:?}")))?;

    let mut property_definitions = Vec::with_capacity(layer_data.property_kinds.len());
    for (name, kind) in &layer_data.property_kinds {
        let key = builder.add_property(name, *kind).map_err(|error| {
            PipelineError::Other(format!("Failed to add MLT property column: {error:?}"))
        })?;
        property_definitions.push((name.as_str(), key, *kind));
    }

    for feature in layer_data.features {
        let mut feature_builder = builder.feature(feature.geometry);
        feature_builder.id(feature.id);

        for (name, key, kind) in &property_definitions {
            let value = feature
                .properties
                .get(*name)
                .map(|value| value.to_prop_value(*kind))
                .unwrap_or_else(|| PropValue::null(*kind));
            feature_builder.property(*key, value).map_err(|error| {
                PipelineError::Other(format!("Failed to set MLT property value: {error:?}"))
            })?;
        }

        feature_builder.finish().map_err(|error| {
            PipelineError::Other(format!("Failed to add MLT feature: {error:?}"))
        })?;
    }

    builder
        .finish()
        .encode(
            EncoderConfig::default()
                .with_tessellation(false)
                .with_shared_dict(false)
                .with_fsst(false)
                .with_fastpfor(false)
                .with_spatial_morton_sort(false)
                .with_spatial_hilbert_sort(false)
                .with_id_sort(false),
        )
        .map_err(|error| PipelineError::Other(format!("Failed to encode MLT layer: {error:?}")))
}

fn make_geometry(multipolygon: &flatgeom::MultiPolygon2, extent: i32) -> Option<Geometry<i32>> {
    let polygons = quantize_polygons(multipolygon, extent)
        .into_iter()
        .map(|polygon| {
            let QuantizedPolygon {
                exterior,
                interiors,
            } = polygon;
            Polygon::new(
                to_linestring(&exterior),
                interiors
                    .into_iter()
                    .map(|ring| to_linestring(&ring))
                    .collect(),
            )
        })
        .collect::<Vec<_>>();

    (!polygons.is_empty()).then_some(Geometry::MultiPolygon(MultiPolygon(polygons)))
}

fn to_linestring(ring: &[[i32; 2]]) -> LineString<i32> {
    let mut coordinates = ring
        .iter()
        .map(|[x, y]| Coord { x: *x, y: *y })
        .collect::<Vec<_>>();
    if coordinates.first() != coordinates.last() {
        if let Some(first) = coordinates.first().copied() {
            coordinates.push(first);
        }
    }
    LineString(coordinates)
}

fn exact_i64_to_f64(value: i64) -> Option<f64> {
    let converted = value as f64;
    ((converted as i128) == i128::from(value)).then_some(converted)
}

fn exact_u64_to_f64(value: u64) -> Option<f64> {
    let converted = value as f64;
    ((converted as u128) == u128::from(value)).then_some(converted)
}

fn convert_property_value(value: &object::Value) -> Option<MltPropertyValue> {
    match value {
        CityGmlValue::String(value) => Some(MltPropertyValue::Str(value.clone())),
        CityGmlValue::Code(value) => Some(MltPropertyValue::Str(value.value().to_string())),
        CityGmlValue::Integer(value) => Some(MltPropertyValue::I64(*value)),
        CityGmlValue::NonNegativeInteger(value) => Some(MltPropertyValue::U64(*value)),
        CityGmlValue::Double(value) => Some(MltPropertyValue::F64(*value)),
        CityGmlValue::Measure(value) => Some(MltPropertyValue::F64(value.value())),
        CityGmlValue::Boolean(value) => Some(MltPropertyValue::Bool(*value)),
        CityGmlValue::Uri(value) => Some(MltPropertyValue::Str(value.value().to_string())),
        CityGmlValue::Date(value) => Some(MltPropertyValue::Str(value.to_string())),
        CityGmlValue::Point(value) => Some(MltPropertyValue::Str(format!("{value:?}"))),
        CityGmlValue::Array(_) | CityGmlValue::Object(_) => None,
    }
}

fn schema_property_kind(schema: &Schema, typename: &str, property: &str) -> PropKind {
    let Some(type_definition) = schema.types.get(typename) else {
        return PropKind::Str;
    };

    if let Some(attributes) = type_definition_attributes(type_definition) {
        if let Some(attribute) = attributes.get(property) {
            if let Some(kind) = resolve_type_ref_property_kind(schema, &attribute.type_ref, &[], 0)
            {
                return kind;
            }
        }
    }

    let path = property.split('.').collect::<Vec<_>>();
    resolve_type_definition_property_kind(schema, type_definition, &path, 0)
        .unwrap_or(PropKind::Str)
}

fn type_ref_to_property_kind(type_ref: &TypeRef) -> Option<PropKind> {
    match type_ref {
        TypeRef::String
        | TypeRef::Code
        | TypeRef::JsonString(_)
        | TypeRef::URI
        | TypeRef::Date
        | TypeRef::DateTime
        | TypeRef::Point => Some(PropKind::Str),
        TypeRef::Integer => Some(PropKind::I64),
        TypeRef::NonNegativeInteger => Some(PropKind::U64),
        TypeRef::Double | TypeRef::Measure => Some(PropKind::F64),
        TypeRef::Boolean => Some(PropKind::Bool),
        TypeRef::Named(_) | TypeRef::Unknown => None,
    }
}

fn resolve_type_definition_property_kind(
    schema: &Schema,
    type_definition: &TypeDef,
    path: &[&str],
    depth: usize,
) -> Option<PropKind> {
    if depth > 64 {
        return None;
    }

    match type_definition {
        TypeDef::Feature(feature) => {
            resolve_attribute_property_kind(schema, &feature.attributes, path, depth + 1)
        }
        TypeDef::Data(data) => {
            resolve_attribute_property_kind(schema, &data.attributes, path, depth + 1)
        }
        TypeDef::Property(property) => property
            .members
            .iter()
            .filter_map(|member| {
                resolve_type_ref_property_kind(schema, &member.type_ref, path, depth + 1)
            })
            .reduce(merge_property_kind),
    }
}

fn resolve_attribute_property_kind(
    schema: &Schema,
    attributes: &nusamai_citygml::schema::Map,
    path: &[&str],
    depth: usize,
) -> Option<PropKind> {
    let path = skip_array_indices(path);
    let (name, remaining) = path.split_first()?;
    let attribute = attributes.get(*name)?;
    resolve_type_ref_property_kind(schema, &attribute.type_ref, remaining, depth + 1)
}

fn resolve_type_ref_property_kind(
    schema: &Schema,
    type_ref: &TypeRef,
    path: &[&str],
    depth: usize,
) -> Option<PropKind> {
    if depth > 64 {
        return None;
    }

    let path = skip_array_indices(path);
    if path.is_empty() {
        return type_ref_to_property_kind(type_ref);
    }

    let TypeRef::Named(name) = type_ref else {
        return None;
    };
    let type_definition = schema.types.get(name)?;
    resolve_type_definition_property_kind(schema, type_definition, path, depth + 1)
}

fn type_definition_attributes(type_definition: &TypeDef) -> Option<&nusamai_citygml::schema::Map> {
    match type_definition {
        TypeDef::Feature(feature) => Some(&feature.attributes),
        TypeDef::Data(data) => Some(&data.attributes),
        TypeDef::Property(_) => None,
    }
}

fn skip_array_indices<'a>(path: &'a [&'a str]) -> &'a [&'a str] {
    let first_property = path
        .iter()
        .position(|segment| segment.parse::<usize>().is_err())
        .unwrap_or(path.len());
    &path[first_property..]
}

fn merge_property_kind(current: PropKind, incoming: PropKind) -> PropKind {
    if current == incoming {
        current
    } else if matches!(
        (current, incoming),
        (PropKind::I64 | PropKind::U64, PropKind::F64)
            | (PropKind::F64, PropKind::I64 | PropKind::U64)
    ) {
        PropKind::F64
    } else {
        PropKind::Str
    }
}
