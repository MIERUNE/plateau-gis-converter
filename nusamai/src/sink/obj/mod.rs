//! Minecraft sink

use log::error;
use std::{io::Write, path::PathBuf, sync::Mutex};

use nusamai_citygml::schema::Schema;

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
    transformer::{TransformerOption, TransformerRegistry},
};

pub struct ObjSinkProvider {}

impl DataSinkProvider for ObjSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "obj".to_string(),
            name: "OBJ".to_string(),
        }
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(
            "@output".into(),
            ParameterEntry {
                description: "Output file path".into(),
                required: true,
                parameter: ParameterType::FileSystemPath(FileSystemPathParameter {
                    value: None,
                    must_exist: false,
                }),
                label: None,
            },
        );
        params
    }

    fn available_transformer(&self) -> TransformerRegistry {
        let settings: TransformerRegistry = TransformerRegistry::new();

        settings
    }
    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_options = self.available_transformer();

        Box::<ObjSink>::new(ObjSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings: transform_options,
        })
    }
}

pub struct ObjSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
}

impl DataSink for ObjSink {
    fn make_requirements(&mut self, properties: Vec<TransformerOption>) -> DataRequirements {
        let default_requirements = DataRequirements {
            tree_flattening: transformer::TreeFlatteningSpec::Flatten {
                feature: transformer::FeatureFlatteningOption::AllExceptThematicSurfaces,
                data: transformer::DataFlatteningOption::None,
                object: transformer::ObjectFlatteningOption::None,
            },
            ..Default::default()
        };

        for prop in properties {
            let _ = &self
                .transform_settings
                .update_transformer(&prop.key, prop.is_enabled);
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        Ok(())
    }
}
