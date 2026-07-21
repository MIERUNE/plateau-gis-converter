//! MapLibre Tiles (MLT) sink

use std::{
    collections::BTreeMap,
    convert::Infallible,
    fs,
    io::prelude::*,
    path::{Path, PathBuf},
    sync::mpsc,
};

use flate2::{write::ZlibEncoder, Compression};
use flatgeom::MultiPolygon2;
use itertools::Itertools;
use mlt_core::{
    encoder::EncoderConfig,
    geo_types::{Coord, Geometry, LineString, MultiPolygon as GeoMultiPolygon, Polygon},
    PropKind, PropValue, TileLayer,
};
use nusamai_citygml::{
    object,
    schema::{Schema, TypeDef, TypeRef},
    Value as CityGmlValue,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{
        mvt::{
            slice::{slice_cityobj_geoms, validate_zoom_range},
            tileid::TileIdMethod,
            DEFAULT_MAX_COMPRESSED_TILE_SIZE,
        },
        DataRequirements, DataSink, DataSinkProvider, SinkInfo,
    },
    transformer,
    transformer::{use_lod_config, TransformerSettings},
};

use super::option::output_parameter;

pub struct MltSinkProvider {}

impl DataSinkProvider for MltSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "mlt".to_string(),
            name: "MapLibre Tiles (MLT)".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());
        params.define(ParameterDefinition {
            key: "min_z".into(),
            entry: ParameterEntry {
                description: "Minimum zoom level".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: Some(7),
                    min: Some(0),
                    max: Some(20),
                }),
                label: Some("最小ズームレベル".into()),
            },
        });
        params.define(ParameterDefinition {
            key: "max_z".into(),
            entry: ParameterEntry {
                description: "Maximum zoom level".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: Some(15),
                    min: Some(0),
                    max: Some(20),
                }),
                label: Some("最大ズームレベル".into()),
            },
        });
        params.define(ParameterDefinition {
            key: "max_compressed_tile_size".into(),
            entry: ParameterEntry {
                description:
                    "Maximum zlib-compressed tile size in bytes before lowering tile detail".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: Some(DEFAULT_MAX_COMPRESSED_TILE_SIZE as i64),
                    min: Some(1),
                    max: None,
                }),
                label: Some("最大圧縮タイルサイズ".into()),
            },
        });

        params
    }

    fn transformer_options(&self) -> TransformerSettings {
        let mut settings: TransformerSettings = TransformerSettings::new();
        settings.insert(use_lod_config("min_lod", None));

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_options = self.transformer_options();
        let min_z = get_parameter_value!(params, "min_z", Integer).unwrap() as u8;
        let max_z = get_parameter_value!(params, "max_z", Integer).unwrap() as u8;
        validate_zoom_range(min_z, max_z);
        let max_compressed_tile_size =
            get_parameter_value!(params, "max_compressed_tile_size", Integer).unwrap() as usize;

        Box::<MltSink>::new(MltSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings: transform_options,
            mlt_options: MltParams {
                min_z,
                max_z,
                max_compressed_tile_size,
            },
        })
    }
}

struct MltSink {
    output_path: PathBuf,
    transform_settings: TransformerSettings,
    mlt_options: MltParams,
}

struct MltParams {
    min_z: u8,
    max_z: u8,
    max_compressed_tile_size: usize,
}

#[derive(Serialize, Deserialize)]
struct SlicedFeature<'a> {
    geometry: MultiPolygon2<'a>,
    properties: object::Value,
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

fn exact_i64_to_f64(value: i64) -> Option<f64> {
    let converted = value as f64;
    ((converted as i128) == i128::from(value)).then_some(converted)
}

fn exact_u64_to_f64(value: u64) -> Option<f64> {
    let converted = value as f64;
    ((converted as u128) == u128::from(value)).then_some(converted)
}

impl DataSink for MltSink {
    fn make_requirements(&mut self, properties: TransformerSettings) -> DataRequirements {
        let default_requirements = DataRequirements {
            key_value: transformer::KeyValueSpec::JsonifyObjectsAndArrays,
            lod_filter: transformer::LodFilterSpec {
                mode: transformer::LodFilterMode::Lowest,
                ..Default::default()
            },
            geom_stats: transformer::GeometryStatsSpec::MinMaxHeights,
            ..Default::default()
        };

        for config in properties.configs.iter() {
            let _ = &self.transform_settings.update_transformer(config.clone());
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let (sender_sliced, receiver_sliced) = mpsc::sync_channel(2000);
        let (sender_sorted, receiver_sorted) = mpsc::sync_channel(2000);

        let tile_id_conv = TileIdMethod::Hilbert;

        std::thread::scope(|s| {
            {
                s.spawn(|| {
                    if let Err(error) = geometry_slicing_stage(
                        feedback,
                        upstream,
                        tile_id_conv,
                        sender_sliced,
                        &self.mlt_options,
                    ) {
                        feedback.fatal_error(error);
                    }
                });
            }

            {
                s.spawn(move || {
                    if let Err(error) =
                        feature_sorting_stage(feedback, receiver_sliced, sender_sorted)
                    {
                        feedback.fatal_error(error);
                    }
                });
            }

            {
                let output_path = &self.output_path;
                let max_compressed_tile_size = self.mlt_options.max_compressed_tile_size;
                s.spawn(move || {
                    let pool = rayon::ThreadPoolBuilder::new()
                        .use_current_thread()
                        .build()
                        .unwrap();
                    pool.install(|| {
                        if let Err(error) = tile_writing_stage(
                            output_path,
                            feedback,
                            receiver_sorted,
                            tile_id_conv,
                            max_compressed_tile_size,
                            schema,
                        ) {
                            feedback.fatal_error(error);
                        }
                    })
                });
            }
        });

        Ok(())
    }
}

fn geometry_slicing_stage(
    feedback: &Feedback,
    upstream: mpsc::Receiver<crate::pipeline::Parcel>,
    tile_id_conv: TileIdMethod,
    sender_sliced: mpsc::SyncSender<(u64, Vec<u8>)>,
    mlt_options: &MltParams,
) -> Result<()> {
    let bincode_config = bincode::config::standard();

    upstream.into_iter().par_bridge().try_for_each(|parcel| {
        feedback.ensure_not_canceled()?;

        let max_detail = 12; // 4096
        let buffer_pixels = 5;
        slice_cityobj_geoms(
            &parcel.entity,
            mlt_options.min_z,
            mlt_options.max_z,
            max_detail,
            buffer_pixels,
            |(z, x, y), mpoly| {
                feedback.ensure_not_canceled()?;

                let feature = SlicedFeature {
                    geometry: mpoly,
                    properties: parcel.entity.root.clone(),
                };
                let bytes = bincode::serde::encode_to_vec(&feature, bincode_config).unwrap();
                let tile_id = tile_id_conv.zxy_to_id(z, x, y);
                if sender_sliced.send((tile_id, bytes)).is_err() {
                    return Err(PipelineError::Canceled);
                };
                Ok(())
            },
        )
    })?;
    Ok(())
}

fn feature_sorting_stage(
    feedback: &Feedback,
    receiver_sliced: mpsc::Receiver<(u64, Vec<u8>)>,
    sender_sorted: mpsc::SyncSender<(u64, Vec<Vec<u8>>)>,
) -> Result<()> {
    let config = kv_extsort::SortConfig::default()
        .max_chunk_bytes(256 * 1024 * 1024)
        .set_cancel_flag(feedback.get_cancellation_flag());

    let sorted_iter = kv_extsort::sort(
        receiver_sliced
            .into_iter()
            .map(|(tile_id, body)| std::result::Result::<_, Infallible>::Ok((tile_id, body))),
        config,
    );

    for ((_, tile_id), grouped) in &sorted_iter.chunk_by(|feat| match feat {
        Ok((tile_id, _)) => (false, *tile_id),
        Err(_) => (true, 0),
    }) {
        let grouped = grouped
            .into_iter()
            .map_ok(|(_, serialized_feats)| serialized_feats)
            .collect::<kv_extsort::Result<Vec<_>, _>>();
        match grouped {
            Ok(serialized_feats) => {
                feedback.ensure_not_canceled()?;
                if sender_sorted.send((tile_id, serialized_feats)).is_err() {
                    return Err(PipelineError::Canceled);
                }
            }
            Err(kv_extsort::Error::Canceled) => {
                return Err(PipelineError::Canceled);
            }
            Err(err) => {
                return Err(PipelineError::Other(format!(
                    "Failed to sort features: {err:?}"
                )));
            }
        }
    }

    Ok(())
}

fn tile_writing_stage(
    output_path: &Path,
    feedback: &Feedback,
    receiver_sorted: mpsc::Receiver<(u64, Vec<Vec<u8>>)>,
    tile_id_conv: TileIdMethod,
    max_compressed_tile_size: usize,
    schema: &Schema,
) -> Result<()> {
    let default_detail = 12;
    let min_detail = 9;

    receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|(tile_id, serialized_feats)| {
            feedback.ensure_not_canceled()?;

            let (zoom, x, y) = tile_id_conv.id_to_zxy(tile_id);

            if serialized_feats.len() > 200_000 {
                feedback.warn(format!(
                    "Too many features in a tile ({} features)",
                    serialized_feats.len()
                ));
            }

            let path = output_path.join(Path::new(&format!("{zoom}/{x}/{y}.mlt")));
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }

            for detail in (min_detail..=default_detail).rev() {
                feedback.ensure_not_canceled()?;

                let bytes = make_tile(detail, &serialized_feats, schema)?;

                let compressed_size = {
                    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
                    e.write_all(&bytes)?;
                    let compressed_bytes = e.finish()?;
                    compressed_bytes.len()
                };
                if detail != min_detail && compressed_size > max_compressed_tile_size {
                    let extent = 1 << detail;
                    feedback.info(format!(
                        "Tile size is too large: {zoom}/{x}/{y} (extent: {extent}), trying a \
                         lower detail level."
                    ));
                    continue;
                }

                feedback.info(format!(
                    "Writing a tile: {} ({} bytes, {} compressed)",
                    path.to_string_lossy(),
                    bytesize::ByteSize(bytes.len() as u64),
                    bytesize::ByteSize(compressed_size as u64),
                ));
                fs::write(&path, &bytes)?;
                break;
            }

            Ok::<(), PipelineError>(())
        })?;

    Ok(())
}

fn make_tile(
    default_detail: i32,
    serialized_feats: &[Vec<u8>],
    schema: &Schema,
) -> Result<Vec<u8>> {
    let mut layers: BTreeMap<String, LayerData> = BTreeMap::new();
    let extent = 1 << default_detail;
    let bincode_config = bincode::config::standard();

    for serialized_feat in serialized_feats {
        let (feature, _): (SlicedFeature, _) =
            bincode::serde::decode_from_slice(serialized_feat, bincode_config).map_err(|err| {
                PipelineError::Other(format!("Failed to deserialize a sliced feature: {err:?}"))
            })?;

        let Some(geometry) = make_geometry(&feature.geometry, extent) else {
            continue;
        };

        if let object::Value::Object(obj) = &feature.properties {
            let typename: &str = obj.typename.as_ref();
            let layer = layers.entry(typename.to_string()).or_default();

            let mut properties = BTreeMap::new();
            for (key, value) in &obj.attributes {
                let Some(value) = convert_property_value(value) else {
                    continue;
                };
                layer
                    .property_kinds
                    .entry(key.clone())
                    .or_insert_with(|| schema_property_kind(schema, typename, key));
                properties.insert(key.clone(), value);
            }

            let id = obj.stereotype.id().map(hash_feature_id);
            layer.features.push(MltFeature {
                geometry,
                id,
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
        let bytes = encode_layer(name, layer_data, extent as u32)?;
        tile.extend(bytes);
    }

    Ok(tile)
}

fn encode_layer(name: String, layer_data: LayerData, extent: u32) -> Result<Vec<u8>> {
    let mut builder = TileLayer::builder(name, extent)
        .map_err(|err| PipelineError::Other(format!("Failed to create MLT layer: {err:?}")))?;

    let mut property_defs = Vec::with_capacity(layer_data.property_kinds.len());
    for (name, kind) in &layer_data.property_kinds {
        let key = builder.add_property(name, *kind).map_err(|err| {
            PipelineError::Other(format!("Failed to add MLT property column: {err:?}"))
        })?;
        property_defs.push((name.as_str(), key, *kind));
    }

    for feature in layer_data.features {
        let MltFeature {
            geometry,
            id,
            properties,
        } = feature;

        let mut feature_builder = builder.feature(geometry);
        feature_builder.id(id);

        for (name, key, kind) in &property_defs {
            let value = properties
                .get(*name)
                .map(|value| value.to_prop_value(*kind))
                .unwrap_or_else(|| PropValue::null(*kind));
            feature_builder.property(*key, value).map_err(|err| {
                PipelineError::Other(format!("Failed to set MLT property value: {err:?}"))
            })?;
        }

        feature_builder
            .finish()
            .map_err(|err| PipelineError::Other(format!("Failed to add MLT feature: {err:?}")))?;
    }

    // Keep output compatible with MapLibre GL JS's current MLT decoder. Optional encodings can be
    // enabled after decoder support for them has been confirmed.
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
        .map_err(|err| PipelineError::Other(format!("Failed to encode MLT layer: {err:?}")))
}

fn make_geometry(mpoly: &MultiPolygon2, extent: i32) -> Option<Geometry<i32>> {
    let mut polygons = Vec::new();
    let mut int_ring_buf = Vec::new();
    let mut simplified_ring_buf = Vec::new();

    for poly in mpoly {
        let mut exterior = None;
        let mut interiors = Vec::new();

        for (ring_index, ring) in poly.rings().enumerate() {
            int_ring_buf.clear();
            int_ring_buf.extend(ring.into_iter().map(|[x, y]| {
                let x = (x * extent as f64 + 0.5) as i32;
                let y = (y * extent as f64 + 0.5) as i32;
                [x, y]
            }));

            if int_ring_buf.len() < 3 {
                continue;
            }

            simplified_ring_buf.clear();
            simplified_ring_buf.push(int_ring_buf[0]);
            for c in int_ring_buf.windows(3) {
                let &[prev, curr, next] = c.try_into().unwrap();

                if prev == curr {
                    continue;
                }

                if is_collinear(prev, curr, next) {
                    continue;
                }

                simplified_ring_buf.push(curr);
            }
            simplified_ring_buf.push(*int_ring_buf.last().unwrap());

            if simplified_ring_buf.len() < 3 {
                continue;
            }

            let area = signed_ring_area2(&simplified_ring_buf);
            if ring_index == 0 {
                if area > 0 {
                    exterior = Some(to_linestring(&simplified_ring_buf));
                }
            } else if area < 0 {
                interiors.push(to_linestring(&simplified_ring_buf));
            }
        }

        if let Some(exterior) = exterior {
            polygons.push(Polygon::new(exterior, interiors));
        }
    }

    if polygons.is_empty() {
        None
    } else {
        Some(Geometry::MultiPolygon(GeoMultiPolygon(polygons)))
    }
}

fn is_collinear(prev: [i32; 2], curr: [i32; 2], next: [i32; 2]) -> bool {
    if curr == next {
        return false;
    }
    let [curr_x, curr_y] = curr.map(i64::from);
    let [prev_x, prev_y] = prev.map(i64::from);
    let [next_x, next_y] = next.map(i64::from);
    ((next_y - prev_y) * (curr_x - prev_x)).abs() == ((curr_y - prev_y) * (next_x - prev_x)).abs()
}

fn signed_ring_area2(ring: &[[i32; 2]]) -> i64 {
    ring.iter()
        .zip(ring.iter().cycle().skip(1))
        .take(ring.len())
        .map(|([x1, y1], [x2, y2])| (*x1 as i64 * *y2 as i64) - (*x2 as i64 * *y1 as i64))
        .sum()
}

fn to_linestring(ring: &[[i32; 2]]) -> LineString<i32> {
    let mut coords: Vec<_> = ring.iter().map(|[x, y]| Coord { x: *x, y: *y }).collect();
    if coords.first() != coords.last() {
        if let Some(first) = coords.first().copied() {
            coords.push(first);
        }
    }
    LineString(coords)
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
    let Some(type_def) = schema.types.get(typename) else {
        return PropKind::Str;
    };

    if let Some(attributes) = type_def_attributes(type_def) {
        if let Some(attribute) = attributes.get(property) {
            if let Some(kind) = resolve_type_ref_property_kind(schema, &attribute.type_ref, &[], 0)
            {
                return kind;
            }
        }
    }

    let path = property.split('.').collect::<Vec<_>>();
    resolve_type_def_property_kind(schema, type_def, &path, 0).unwrap_or(PropKind::Str)
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

fn resolve_type_def_property_kind(
    schema: &Schema,
    type_def: &TypeDef,
    path: &[&str],
    depth: usize,
) -> Option<PropKind> {
    if depth > 64 {
        return None;
    }

    match type_def {
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
    let type_def = schema.types.get(name)?;
    resolve_type_def_property_kind(schema, type_def, path, depth + 1)
}

fn type_def_attributes(type_def: &TypeDef) -> Option<&nusamai_citygml::schema::Map> {
    match type_def {
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

fn hash_feature_id(id: &str) -> u64 {
    id.as_bytes()
        .iter()
        .fold(5381u64, |a, c| a.wrapping_mul(33) ^ *c as u64)
}
