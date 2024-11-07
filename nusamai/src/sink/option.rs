use crate::parameters::{
    BooleanParameter, FileSystemPathParameter, ParameterDefinition, ParameterEntry, ParameterType,
};

pub fn output_parameter() -> ParameterDefinition {
    ParameterDefinition {
        key: "@output".into(),
        entry: ParameterEntry {
            description: "Output file path".into(),
            required: true,
            parameter: ParameterType::FileSystemPath(FileSystemPathParameter {
                value: None,
                must_exist: false,
            }),
            label: None,
        },
    }
}

pub fn limit_texture_resolution_parameter(default_value: bool) -> ParameterDefinition {
    ParameterDefinition {
        key: "limit_texture_resolution".into(),
        entry: ParameterEntry {
            description: "limiting texture resolution".into(),
            required: false,
            parameter: ParameterType::Boolean(BooleanParameter {
                value: Some(default_value),
            }),
            label: Some("距離あたりの解像度を制限する".into()),
        },
    }
}
