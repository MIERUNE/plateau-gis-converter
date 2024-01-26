//! No-op Sink (demo)
//!
//! This is a demo sync that only counts the number of features and vertices and does not generate any output.

use std::io::Write;

use nusamai_citygml::schema::Schema;

use crate::parameters::{
    BooleanParameter, FileSystemPathParameter, ParameterEntry, ParameterType, Parameters,
};
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use crate::get_parameter_value;

pub struct NoopSinkProvider {}

impl DataSinkProvider for NoopSinkProvider {
    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let write_schema = get_parameter_value!(params, "schema", Boolean);
        Box::new(NoopSink {
            write_schema: write_schema.unwrap_or_default(),
            num_features: 0,
            num_vertices: 0,
        })
    }

    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "No-op".to_string(),
        }
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(
            "@output".into(),
            ParameterEntry {
                description: "Output file path (dummy, no effect)".into(),
                required: false,
                parameter: ParameterType::FileSystemPath(FileSystemPathParameter {
                    value: None,
                    must_exist: false,
                }),
            },
        );
        params.define(
            "schema".into(),
            ParameterEntry {
                description: "Output schema file".into(),
                required: false,
                parameter: ParameterType::Boolean(BooleanParameter { value: None }),
            },
        );
        params
    }
}

pub struct NoopSink {
    write_schema: bool,
    num_features: usize,
    num_vertices: usize,
}

impl DataSink for NoopSink {
    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) {
        if self.write_schema {
            let mut file = std::fs::File::create("schema.json").unwrap();
            file.write_all(serde_json::to_string_pretty(schema).unwrap().as_bytes())
                .unwrap();
        }

        for parcel in upstream {
            if feedback.is_cancelled() {
                log::info!("sink cancelled");
                return;
            }

            self.num_features += 1;
            self.num_vertices += parcel.entity.geometry_store.read().unwrap().vertices.len();

            log::info!("feature: {:?}", parcel.entity.root);
        }

        if feedback.is_cancelled() {
            return;
        }
        log::info!("total number of features: {:#?}", self.num_features);
        log::info!("total vertices: {}", self.num_vertices);
    }
}
