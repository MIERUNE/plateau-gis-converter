// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use log::LevelFilter;
use nusamai::parameters::Parameters;
use nusamai::{
    pipeline::{feedback, Canceller},
    sink::{
        cesiumtiles::CesiumTilesSinkProvider, czml::CzmlSinkProvider, geojson::GeoJsonSinkProvider,
        gltf::GltfSinkProvider, gpkg::GpkgSinkProvider, kml::KmlSinkProvider,
        minecraft::MinecraftSinkProvider, mvt::MvtSinkProvider, obj::ObjSinkProvider,
        serde::SerdeSinkProvider, shapefile::ShapefileSinkProvider, DataSinkProvider,
    },
    source::{citygml::CityGmlSourceProvider, DataSourceProvider},
    transformer::{
        self, MappingRules, MultiThreadTransformer, NusamaiTransformBuilder, TransformBuilder,
        TransformerOption, TransformerRegistry,
    },
};
use nusamai_plateau::models::TopLevelCityObject;
use tauri_plugin_log::{LogTarget, RotationStrategy, TimezoneStrategy};
use thiserror::Error;

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

struct ConversionTasksState {
    canceller: Arc<Mutex<Canceller>>,
}

fn main() {
    // System log plugin
    let tauri_loggger = tauri_plugin_log::Builder::default()
        .targets([LogTarget::Stdout, LogTarget::LogDir, LogTarget::Webview])
        .max_file_size(1_000_000) // in bytes
        .rotation_strategy(RotationStrategy::KeepOne)
        .timezone_strategy(TimezoneStrategy::UseLocal)
        .level(LOG_LEVEL)
        .level_for("sqlx", LevelFilter::Info) // suppress sqlx logs, as it's too verbose in DEBUG level
        .build();

    // Build and run the Tauri app
    tauri::Builder::default()
        .plugin(tauri_loggger)
        .manage(ConversionTasksState {
            canceller: Arc::new(Mutex::new(Canceller::default())),
        })
        .invoke_handler(tauri::generate_handler![
            run_conversion,
            cancel_conversion,
            get_parameter,
            get_transform
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, serde::Serialize)]
struct LogMessage {
    message: String,
    level: String,
    error_message: Option<String>,
    source: String,
}

impl From<&feedback::Message> for LogMessage {
    fn from(msg: &feedback::Message) -> Self {
        LogMessage {
            message: msg.message.to_string(),
            level: msg.level.to_string(),
            error_message: msg.error.as_ref().map(|e| e.to_string()),
            source: msg.source_component.to_string(),
        }
    }
}

// Everything returned from Tauri commands must implement serde::Serialize
#[derive(Error, Debug, serde::Serialize)]
#[serde(tag = "type", content = "message")]
enum Error {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Invalid setting: {0}")]
    InvalidSetting(String),
    #[error("Invalid mapping rules: {0}")]
    InvalidMappingRules(String),
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
    #[error("Conversion canceled")]
    Canceled,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

fn select_sink_provider(filetype: &str) -> Option<Box<dyn DataSinkProvider>> {
    // TODO: share possible options with the frontend types (src/lib/settings.ts)
    match filetype {
        "noop" => Some(Box::new(nusamai::sink::noop::NoopSinkProvider {})),
        "serde" => Some(Box::new(SerdeSinkProvider {})),
        "geojson" => Some(Box::new(GeoJsonSinkProvider {})),
        "gpkg" => Some(Box::new(GpkgSinkProvider {})),
        "mvt" => Some(Box::new(MvtSinkProvider {})),
        "shapefile" => Some(Box::new(ShapefileSinkProvider {})),
        "czml" => Some(Box::new(CzmlSinkProvider {})),
        "kml" => Some(Box::new(KmlSinkProvider {})),
        "gltf" => Some(Box::new(GltfSinkProvider {})),
        "cesiumtiles" => Some(Box::new(CesiumTilesSinkProvider {})),
        "minecraft" => Some(Box::new(MinecraftSinkProvider {})),
        "obj" => Some(Box::new(ObjSinkProvider {})),
        _ => None,
    }
}

#[tauri::command(async)]
#[allow(clippy::too_many_arguments)]
fn run_conversion(
    input_paths: Vec<String>,
    output_path: String,
    filetype: String,
    epsg: u16,
    rules_path: String,
    transformer_options: Vec<TransformerOption>,
    sink_parameters: Parameters,
    tasks_state: tauri::State<ConversionTasksState>,
    window: tauri::Window,
) -> Result<(), Error> {
    // Request cancellation of previous task if still running
    tasks_state.canceller.lock().unwrap().cancel();

    // Check the existence of the input paths
    for path in input_paths.iter() {
        if !PathBuf::from_str(path).unwrap().exists() {
            let msg = format!("Input file does not exist: {}", path);
            log::error!("{}", msg);
            return Err(Error::InvalidPath(msg));
        }
    }
    // Check if the mapping rules file is set, and if it exists
    if !rules_path.is_empty() && !PathBuf::from_str(&rules_path).unwrap().exists() {
        let msg = format!("Mapping rules file does not exist: {}", rules_path);
        log::error!("{}", msg);
        return Err(Error::InvalidPath(msg));
    };

    // If the directory for the output path does not exist, create it
    let output_path_buf = PathBuf::from_str(&output_path).unwrap();
    let output_parent_dir = output_path_buf.parent().unwrap();
    if !output_parent_dir.exists() {
        std::fs::create_dir_all(output_parent_dir)?;
        log::info!("Created output directory: {:?}", output_parent_dir);
    }

    let sinkopt: Vec<(String, String)> = vec![("@output".into(), output_path)];

    log::info!("Running pipeline with input: {:?}", input_paths);

    let mut sink = {
        let sink_provider = select_sink_provider(&filetype).ok_or_else(|| {
            let msg = format!("Invalid sink type: {}", filetype);
            log::error!("{}", msg);
            Error::InvalidSetting(msg)
        })?;

        let mut sink_params = sink_parameters;
        if let Err(err) = sink_params.update_values_with_str(&sinkopt) {
            let msg = format!("Error parsing sink options: {:?}", err);
            log::error!("{}", msg);
            return Err(Error::InvalidSetting(msg));
        };
        if let Err(err) = sink_params.validate() {
            let msg = format!("Error validating sink parameters: {:?}", err);
            log::error!("{}", msg);
            return Err(Error::InvalidSetting(msg));
        }
        sink_provider.create(&sink_params)
    };

    let mut requirements = sink.make_requirements(transformer_options);
    requirements.set_output_epsg(epsg);

    let source = {
        let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGmlSourceProvider {
            filenames: input_paths
                .iter()
                .map(|s| PathBuf::from_str(s).unwrap())
                .collect(),
        });
        let mut source_params = source_provider.parameters();
        if let Err(err) = source_params.validate() {
            let msg = format!("Error validating source parameters: {:?}", err);
            log::error!("{}", msg);
            return Err(Error::InvalidSetting(msg));
        }
        let mut source = source_provider.create(&source_params);
        source.set_appearance_parsing(requirements.use_appearance);
        source
    };

    let (transformer, schema) = {
        use nusamai_citygml::CityGmlElement;

        let mapping_rules = if rules_path.is_empty() {
            None
        } else {
            let file_contents = std::fs::read_to_string(rules_path).map_err(|e| {
                let msg = format!("Error reading mapping rules file: {}", e);
                log::error!("{}", msg);
                Error::InvalidMappingRules(msg)
            })?;
            let mapping_rules: MappingRules =
                serde_json::from_str(&file_contents).map_err(|e| {
                    let msg = format!("Error parsing mapping rules: {}", e);
                    log::error!("{}", msg);
                    Error::InvalidMappingRules(msg)
                })?;
            Some(mapping_rules)
        };

        let request = {
            let mut request = transformer::Request::from(requirements);
            request.set_mapping_rules(mapping_rules);
            request
        };
        let transform_builder = NusamaiTransformBuilder::new(request);
        let mut schema = nusamai_citygml::schema::Schema::default();
        TopLevelCityObject::collect_schema(&mut schema);
        transform_builder.transform_schema(&mut schema);
        let transformer = Box::new(MultiThreadTransformer::new(transform_builder));
        (transformer, schema)
    };

    // start the pipeline
    let (handle, watcher, inner_canceller) =
        nusamai::pipeline::run(source, transformer, sink, schema.into());

    // Store the canceller to the application state
    *tasks_state.canceller.lock().unwrap() = inner_canceller;

    let first_error = std::thread::scope(|scope| {
        // log watcher
        scope
            .spawn(move || {
                for msg in watcher {
                    window
                        .emit("conversion-log", LogMessage::from(&msg))
                        .unwrap();
                    if let Some(err) = &msg.error {
                        return Some(Error::ConversionFailed(err.to_string()));
                    }
                }
                None
            })
            .join()
    })
    .unwrap();

    // Wait for the pipeline to finish
    if let Err(msg) = handle.join() {
        return Err(Error::ConversionFailed(format!(
            "Pipeline thread panicked: {msg}"
        )));
    }

    // Return error if an error occurred in the pipeline
    if let Some(err) = first_error {
        return Err(err);
    }

    // Return the 'Canceled' error if the pipeline is canceled
    if tasks_state.canceller.lock().unwrap().is_canceled() {
        log::info!("Pipeline canceled");
        return Err(Error::Canceled);
    };

    Ok(())
}

/// Request cancellation of the current conversion task
#[tauri::command]
fn cancel_conversion(tasks_state: tauri::State<ConversionTasksState>) {
    tasks_state.canceller.lock().unwrap().cancel();
}

/// Get the transform options for a given sink type
#[tauri::command]
fn get_transform(filetype: String) -> Result<TransformerRegistry, Error> {
    let sink_provider = select_sink_provider(&filetype).ok_or_else(|| {
        let msg = format!("Invalid sink type: {}", filetype);
        log::error!("{}", msg);
        Error::InvalidSetting(msg)
    })?;

    let transformer_registry = sink_provider.available_transformer();

    Ok(transformer_registry)
}

/// Get the configurable parameters of the sink
#[tauri::command]
fn get_parameter(filetype: String) -> Result<Parameters, Error> {
    let sink_provider = select_sink_provider(&filetype).ok_or_else(|| {
        let msg = format!("Invalid sink type: {}", filetype);
        log::error!("{}", msg);
        Error::InvalidSetting(msg)
    })?;
    let sink_params = sink_provider.parameters();

    Ok(sink_params)
}
