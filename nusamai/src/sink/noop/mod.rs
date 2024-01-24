//! No-op Sink (demo)
//!
//! This is a demo sync that only counts the number of features and vertices and does not generate any output.

use nusamai_citygml::schema::Schema;

use crate::parameters::{FileSystemPathParameter, ParameterEntry, ParameterType, Parameters};
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

pub struct NoopSinkProvider {}

impl DataSinkProvider for NoopSinkProvider {
    fn create(&self, _params: &Parameters) -> Box<dyn DataSink> {
        Box::new(NoopSink {
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
        params
    }
}

pub struct NoopSink {
    num_features: usize,
    num_vertices: usize,
}

impl DataSink for NoopSink {
    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) {
        for parcel in upstream {
            if feedback.is_cancelled() {
                log::info!("sink cancelled");
                return;
            }

            self.num_features += 1;
            self.num_vertices += parcel.entity.geometry_store.read().unwrap().vertices.len();

            // log::info!("feature: {:?}", parcel.cityobj.root);
        }

        if feedback.is_cancelled() {
            return;
        }
        log::info!("total number of features: {:#?}", self.num_features);
        log::info!("total vertices: {}", self.num_vertices);
    }
}
