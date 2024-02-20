//! No-op Sink (demo)
//!
//! This is a demo sync that only counts the number of features and vertices and does not generate any output.

use nusamai_citygml::schema::Schema;

use crate::parameters::{FileSystemPathParameter, ParameterEntry, ParameterType, Parameters};
use crate::pipeline::{Feedback, Receiver, Result};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use crate::transformer;

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
            id_name: "noop".to_string(),
            name: "No Output".to_string(),
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
        // REMOVE: deprecated
        // params.define(
        //     "schema".into(),
        //     ParameterEntry {
        //         description: "Output schema file".into(),
        //         required: false,
        //         parameter: ParameterType::Boolean(BooleanParameter { value: None }),
        //     },
        // );
        params
    }
}

pub struct NoopSink {
    num_features: usize,
    num_vertices: usize,
}

impl DataSink for NoopSink {
    fn make_transform_requirements(&self) -> transformer::Requirements {
        transformer::Requirements {
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        for parcel in upstream {
            feedback.ensure_not_canceled()?;

            self.num_features += 1;
            self.num_vertices += parcel.entity.geometry_store.read().unwrap().vertices.len();

            log::info!("feature: {:?}", parcel.entity.root);
        }

        feedback.ensure_not_canceled()?;
        log::info!("total number of features: {:#?}", self.num_features);
        log::info!("total vertices: {}", self.num_vertices);

        Ok(())
    }
}
