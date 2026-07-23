//! Mapbox Vector Tiles (MVT) sink

pub(crate) mod encode;
mod tags;

use std::path::PathBuf;

use nusamai_citygml::schema::Schema;
use nusamai_projection::crs::EPSG_WGS84_GEOGRAPHIC_3D;

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, Receiver, Result},
    sink::{
        vector_tile::{
            run_vector_tile_pipeline, slice::validate_zoom_range, TilePipelineOptions,
            DEFAULT_MAX_COMPRESSED_TILE_SIZE,
        },
        DataRequirements, DataSink, DataSinkProvider, SinkInfo, SinkInputCrsRequirement,
    },
    transformer,
    transformer::{use_lod_config, TransformerSettings},
};

use self::encode::MvtTileEncoder;
use super::option::output_parameter;

pub struct MvtSinkProvider {}

impl DataSinkProvider for MvtSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "mvt".to_string(),
            name: "Mapbox Vector Tiles (MVT)".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());
        params.define(ParameterDefinition {
            key: "min_z".into(),
            entry: ParameterEntry {
                description: "Minumum zoom level".into(),
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
        params
    }

    fn transformer_options(&self) -> TransformerSettings {
        let mut settings = TransformerSettings::new();
        settings.insert(use_lod_config("min_lod", None));
        settings
    }

    fn sink_input_crs_requirement(&self) -> SinkInputCrsRequirement {
        // TODO: Switch to Fixed(EPSG:3857). ProjectionTransform should produce Web Mercator
        // coordinates, and the shared vector-tile sink should consume them directly instead of
        // calling lnglat_to_web_mercator, performing only the tile-space conversion itself.
        SinkInputCrsRequirement::Fixed(EPSG_WGS84_GEOGRAPHIC_3D)
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let min_z = get_parameter_value!(params, "min_z", Integer).unwrap() as u8;
        let max_z = get_parameter_value!(params, "max_z", Integer).unwrap() as u8;
        validate_zoom_range(min_z, max_z);

        Box::new(MvtSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings: self.transformer_options(),
            options: MvtParams { min_z, max_z },
        })
    }
}

struct MvtSink {
    output_path: PathBuf,
    transform_settings: TransformerSettings,
    options: MvtParams,
}

struct MvtParams {
    min_z: u8,
    max_z: u8,
}

impl DataSink for MvtSink {
    fn make_requirements(&mut self, properties: TransformerSettings) -> DataRequirements {
        let default_requirements = DataRequirements {
            key_value: transformer::KeyValueSpec::DotNotation,
            lod_filter: transformer::LodFilterSpec {
                mode: transformer::LodFilterMode::Lowest,
                ..Default::default()
            },
            geom_stats: transformer::GeometryStatsSpec::MinMaxHeights,
            ..Default::default()
        };

        for config in &properties.configs {
            self.transform_settings.update_transformer(config.clone());
        }
        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        run_vector_tile_pipeline(
            &self.output_path,
            "pbf",
            upstream,
            feedback,
            TilePipelineOptions {
                min_z: self.options.min_z,
                max_z: self.options.max_z,
                max_compressed_tile_size: DEFAULT_MAX_COMPRESSED_TILE_SIZE,
            },
            &MvtTileEncoder::for_mvt_directory(),
        )
    }
}
