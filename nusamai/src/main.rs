use std::{
    env,
    io::Write,
    path::PathBuf,
    process::ExitCode,
    sync::{Arc, Mutex, OnceLock},
};

use clap::Parser;
use nusamai::{
    pipeline::Canceller,
    sink::{DataRequirements, DataSink, DataSinkProvider},
    source::{citygml::CityGmlSourceProvider, DataSource, DataSourceProvider},
    transformer::{
        self, MappingRules, MultiThreadTransformer, NusamaiTransformBuilder, ParameterType,
        TransformBuilder, TransformerConfig, TransformerRegistry,
    },
    BUILTIN_SINKS,
};
use nusamai_citygml::CityGmlElement;
use nusamai_plateau::models::TopLevelCityObject;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify path patterns to the input CityGML files
    #[arg()]
    file_patterns: Vec<String>,

    /// Select the output format
    #[arg(value_enum, long)]
    sink: SinkChoice,

    /// Specify the output path
    #[arg(long, value_parser = parse_non_empty)]
    output: String,

    /// Specify the output EPSG code (default: WGS84 3D)
    #[arg(long, default_value_t = 4979)]
    epsg: u16,

    /// Specify the mapping rules JSON file
    #[arg(long)]
    rules: Option<String>,

    /// Output schema
    #[arg(long)]
    schema: Option<String>,

    /// Add options for the output sink (key=value)
    /// These options affect how the data is written to the output sink
    #[arg(short = 'o', value_parser = parse_key_val)]
    sinkopt: Vec<(String, String)>,

    /// Add options for the transformer (key=value)
    /// These control feature coordinates and attribute trees
    #[arg(short = 't', value_parser = parse_key_val)]
    transformopt: Vec<(String, String)>,

    /// Add an option for the input source (key=value)
    #[arg(short = 'i', value_parser = parse_key_val)]
    sourceopt: Vec<(String, String)>,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].into(), s[pos + 1..].into()))
}

fn parse_non_empty(s: &str) -> Result<String, String> {
    if s.is_empty() {
        Err("value must not be empty".into())
    } else {
        Ok(s.into())
    }
}

#[derive(Clone)]
struct SinkChoice(String);

static SINK_CHOICE_VARIANTS: OnceLock<Vec<SinkChoice>> = OnceLock::new();

impl clap::ValueEnum for SinkChoice {
    fn value_variants<'a>() -> &'a [Self] {
        SINK_CHOICE_VARIANTS.get_or_init(|| {
            BUILTIN_SINKS
                .iter()
                .map(|provider| Self(provider.info().id_name))
                .collect()
        });
        SINK_CHOICE_VARIANTS.get().unwrap()
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        BUILTIN_SINKS
            .iter()
            .find(|provider| provider.info().id_name == self.0)
            .map(|provider| {
                let info = provider.info();
                clap::builder::PossibleValue::new(info.id_name).help(info.name)
            })
    }
}

impl SinkChoice {
    fn create_sink(&self) -> &dyn DataSinkProvider {
        for &provider in nusamai::BUILTIN_SINKS {
            if self.0 == provider.info().id_name {
                return provider;
            }
        }
        panic!("Unknown sink choice: {:?}", self.0);
    }
}

fn main() -> ExitCode {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    let args = {
        // output path
        let mut args = Args::parse();
        args.sinkopt.push(("@output".into(), args.output.clone()));
        args
    };

    let mut canceller = Arc::new(Mutex::new(Canceller::default()));
    {
        let canceller = canceller.clone();
        ctrlc::set_handler(move || {
            log::info!("request cancellation");
            canceller.lock().unwrap().cancel();
        })
        .expect("Error setting Ctrl-C handler");
    }

    let sink_provider: &dyn DataSinkProvider = args.sink.create_sink();
    let mut sink_params = sink_provider.sink_options();
    if let Err(err) = sink_params.update_values_with_str(&args.sinkopt) {
        log::error!("Error parsing sink options: {:?}", err);
        return ExitCode::FAILURE;
    };
    if let Err(err) = sink_params.validate() {
        log::error!("Error validating sink parameters: {:?}", err);
        return ExitCode::FAILURE;
    }

    // If the directory for the output path does not exist, create it
    if let Some(output_parent_dir) = PathBuf::from(&args.output).parent() {
        if !output_parent_dir.exists() {
            if std::fs::create_dir_all(output_parent_dir).is_err() {
                log::error!("Failed to create output directory: {:?}", output_parent_dir);
                return ExitCode::FAILURE;
            };
            log::info!("Created output directory: {:?}", output_parent_dir);
        }
    }

    let mut sink = sink_provider.create(&sink_params);
    let transformer_registry = sink_provider.transformer_options();

    let valid_keys = transformer_registry.initialize_valid_keys();

    // Check if the keys specified in args.transformopt are valid
    for (key, _) in &args.transformopt {
        if !valid_keys.contains(key) {
            let valid_keys_formatted = valid_keys
                .iter()
                .map(|k| format!("'{}'", k))
                .collect::<Vec<_>>()
                .join(", ");
            log::error!(
            "Invalid key '{}' specified for transformer option. Valid keys for {} format are: {}",
            key,
            args.sink.0,
            valid_keys_formatted
        );
            return ExitCode::FAILURE;
        }
    }

    let update_result: Result<Vec<TransformerConfig>, String> = transformer_registry
    .configs
    .into_iter()
    .map(|mut config| {
        // Check if the key from args.transformopt matches the current config's key
        if let Some((_, value)) = args.transformopt.iter().find(|(key, _)| *key == config.key) {
            match &mut config.parameter {
                // If the parameter is of type Selection, update the selected value
                ParameterType::Selection(selection) => {
                    if selection.set_selected_value(value).is_err() {
                        let available_options: Vec<String> = selection.get_options()
                            .iter()
                            .map(|option| format!("'{}'", option.get_value()))
                            .collect();
                        return Err(format!(
                            "Non-existent value '{}' specified for option '{}'. Available options are: {}",
                            value,
                            config.key,
                            available_options.join(", ")
                        ));
                    }
                }
                // If the parameter is of type Boolean, update the boolean value
                ParameterType::Boolean(bool_param) => match value.as_str() {
                    "true" => *bool_param = true,
                    "false" => *bool_param = false,
                    _ => {
                        return Err(format!(
                            "Invalid boolean value '{}' for option '{}'. Only 'true' or 'false' are allowed.",
                            value,
                            config.key
                        ));
                    }
                },
                _ => {
                    return Err(format!("Unsupported parameter type for key '{}'", config.key));
                }
            }
        }
        Ok(config)
    })
    .collect();

    let updated_transformer_registry = match update_result {
        Ok(configs) => TransformerRegistry { configs },
        Err(error_message) => {
            log::error!("{}", error_message);
            return ExitCode::FAILURE;
        }
    };

    let mut requirements = sink.make_requirements(updated_transformer_registry);
    requirements.set_output_epsg(match args.sink.0.as_ref() {
        "kml" => 6697, // temporary hack for KML output
        _ => args.epsg,
    });

    let mapping_rules = match &args.rules {
        Some(rules_path) => {
            let Ok(file_contents) = std::fs::read_to_string(rules_path) else {
                log::error!("Error reading rules file: {}", rules_path);
                return ExitCode::FAILURE;
            };
            let Ok(mapping_rules) = serde_json::from_str::<MappingRules>(&file_contents) else {
                log::error!("Error parsing rules file");
                return ExitCode::FAILURE;
            };
            Some(mapping_rules)
        }
        None => None,
    };

    let source = {
        // glob input file patterns
        let mut filenames = vec![];
        for file_pattern in &args.file_patterns {
            let file_pattern = shellexpand::tilde(file_pattern);
            let mut pattern_hits = 0;
            for entry in glob::glob(&file_pattern).unwrap() {
                filenames.push(entry.unwrap());
                pattern_hits += 1;
            }
            if pattern_hits == 0 {
                log::warn!("no files matched the path pattern: {}", file_pattern);
            }
        }

        if filenames.is_empty() {
            log::error!("No input CityGML files found");
            return ExitCode::FAILURE;
        }

        let source_provider: Box<dyn DataSourceProvider> =
            Box::new(CityGmlSourceProvider { filenames });
        let mut source_params = source_provider.sink_options();
        if let Err(err) = source_params.update_values_with_str(&args.sourceopt) {
            log::error!("Error parsing source parameters: {:?}", err);
            return ExitCode::FAILURE;
        };
        if let Err(err) = source_params.validate() {
            log::error!("Error validating source parameters: {:?}", err);
            return ExitCode::FAILURE;
        }

        // create source
        let mut source = source_provider.create(&source_params);
        source.set_appearance_parsing(requirements.use_appearance);
        source
    };

    run(
        &args,
        source,
        requirements,
        mapping_rules,
        sink,
        &mut canceller,
    );

    ExitCode::SUCCESS
}

fn run(
    args: &Args,
    source: Box<dyn DataSource>,
    requirements: DataRequirements,
    mapping_rules: Option<MappingRules>,
    sink: Box<dyn DataSink>,
    canceller: &mut Arc<Mutex<Canceller>>,
) {
    let total_time = std::time::Instant::now();

    // Prepare the transformer for the pipeline and transform the schema
    let (transformer, schema) = {
        let request = {
            let mut request = transformer::Request::from(requirements);
            request.set_mapping_rules(mapping_rules);
            request
        };
        let transform_builder = NusamaiTransformBuilder::new(request);
        let mut schema = nusamai_citygml::schema::Schema::default();
        TopLevelCityObject::collect_schema(&mut schema);
        transform_builder.transform_schema(&mut schema);

        if let Some(schema_path) = &args.schema {
            let mut file = std::fs::File::create(schema_path).unwrap();
            file.write_all(serde_json::to_string_pretty(&schema).unwrap().as_bytes())
                .unwrap(); // FIXME: error handling
        }

        let transformer = Box::new(MultiThreadTransformer::new(transform_builder));
        (transformer, schema)
    };

    // start the pipeline
    let (handle, watcher, inner_canceller) =
        nusamai::pipeline::run(source, transformer, sink, schema.into());
    *canceller.lock().unwrap() = inner_canceller;

    std::thread::scope(|scope| {
        // log watcher
        scope.spawn(move || {
            for msg in watcher {
                let msg_source = format!("{:?}", msg.source_component);
                match msg.error {
                    Some(error) => {
                        log::log!(msg.level, "[{msg_source}]: {}: {error:?}", msg.message);
                    }
                    None => {
                        log::log!(msg.level, "[{msg_source}]: {}", msg.message);
                    }
                }
            }
        });
    });

    // wait for the pipeline to finish
    if let Err(msg) = handle.join() {
        log::error!("Pipeline thread panicked: {:?}", msg);
    }

    if canceller.lock().unwrap().is_canceled() {
        log::info!("Pipeline canceled");
    }

    log::info!("Total processing time: {:?}", total_time.elapsed());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run_cmd() {
        use assert_cmd::Command;

        let mut cmd = Command::cargo_bin("nusamai").unwrap();
        let assert = cmd
            .arg("../nusamai-plateau/tests/data/sendai-shi/udx/urf/574026_urf_6668_huchi_op.gml")
            .arg("--sink")
            .arg("noop")
            .arg("--output")
            .arg("dummy")
            .arg("--rules")
            .arg("./tests/rules.json")
            .arg("--schema")
            .arg("schema.json")
            .assert();
        assert.success();
    }
}
