// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    source::{citygml::CityGmlSourceProvider, geojson::GeoJsonSourceProvider, DataSourceProvider},
    transformer::{
        self, MappingRules, MultiThreadTransformer, NusamaiTransformBuilder, TransformBuilder,
        TransformerSettings,
    },
};
use nusamai_plateau::models::TopLevelCityObject;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::{
    env,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};
use tauri::Emitter;
use tauri_plugin_log::{RotationStrategy, TimezoneStrategy};
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
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Stdout,
        ))
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir { file_name: None },
        ))
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Webview,
        ))
        .max_file_size(1_000_000) // in bytes
        .rotation_strategy(RotationStrategy::KeepOne)
        .timezone_strategy(TimezoneStrategy::UseLocal)
        .level(LOG_LEVEL)
        .level_for("sqlx", LevelFilter::Info) // suppress sqlx logs, as it's too verbose in DEBUG level
        .build();

    // Build and run the Tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_loggger)
        .manage(ConversionTasksState {
            canceller: Arc::new(Mutex::new(Canceller::default())),
        })
        .invoke_handler(tauri::generate_handler![
            run_conversion,
            cancel_conversion,
            get_parameter,
            get_transform,
            list_supported_files,
            list_zip_contents,
            get_meshcodes_with_prefix,
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
    transformer_settings: TransformerSettings,
    sink_parameters: Parameters,
    tasks_state: tauri::State<ConversionTasksState>,
    app: tauri::AppHandle,
) -> Result<(), Error> {
    println!("Running conversion");

    // Request cancellation of previous task if still running
    tasks_state.canceller.lock().unwrap().cancel();

    // Check the existence of the input paths
    for path in input_paths.iter() {
        if path.contains(".zip/") {
            let parts: Vec<&str> = path.splitn(2, ".zip/").collect();
            if parts.len() == 2 {
                let zip_file = format!("{}.zip", parts[0]);
                if !PathBuf::from_str(&zip_file).unwrap().exists() {
                    let msg = format!("ZIP file does not exist: {zip_file}");
                    log::error!("{msg}");
                    return Err(Error::InvalidPath(msg));
                }
            }
        } else {
            if !PathBuf::from_str(path).unwrap().exists() {
                let msg = format!("Input file does not exist: {path}");
                log::error!("{msg}");
                return Err(Error::InvalidPath(msg));
            }
        }
    }

    if !rules_path.is_empty() && !PathBuf::from_str(&rules_path).unwrap().exists() {
        let msg = format!("Mapping rules file does not exist: {rules_path}");
        log::error!("{msg}");
        return Err(Error::InvalidPath(msg));
    };

    let output_path_buf = PathBuf::from_str(&output_path).unwrap();
    let output_parent_dir = output_path_buf.parent().unwrap();
    if !output_parent_dir.exists() {
        std::fs::create_dir_all(output_parent_dir)?;
        log::info!("Created output directory: {output_parent_dir:?}");
    }

    let sinkopt: Vec<(String, String)> = vec![("@output".into(), output_path)];

    log::info!("Running pipeline with input: {input_paths:?}");

    let mut sink = {
        let sink_provider = select_sink_provider(&filetype).ok_or_else(|| {
            let msg = format!("Invalid sink type: {filetype}");
            log::error!("{msg}");
            Error::InvalidSetting(msg)
        })?;

        let mut sink_params = sink_parameters;
        if let Err(err) = sink_params.update_values_with_str(&sinkopt) {
            let msg = format!("Error parsing sink options: {err:?}");
            log::error!("{msg}");
            return Err(Error::InvalidSetting(msg));
        };
        if let Err(err) = sink_params.validate() {
            let msg = format!("Error validating sink parameters: {err:?}");
            log::error!("{msg}");
            return Err(Error::InvalidSetting(msg));
        }
        sink_provider.create(&sink_params)
    };

    let mut requirements = sink.make_requirements(transformer_settings);
    requirements.set_output_epsg(epsg);

    let source = {
        // ファイルを拡張子で分類
        let mut gml_files = Vec::new();
        let mut geojson_files = Vec::new();
        let mut zip_gml_files = Vec::new();
        let mut zip_geojson_files = Vec::new();

        for path in input_paths.iter() {
            // Check if this is a ZIP file path (format: "zipfile.zip/internal/path.ext")
            if path.contains(".zip/") {
                let parts: Vec<&str> = path.splitn(2, ".zip/").collect();
                if parts.len() == 2 {
                    let internal_path = parts[1];
                    if internal_path.ends_with(".gml") {
                        zip_gml_files.push(path.clone());
                    } else if internal_path.ends_with(".geojson")
                        || internal_path.ends_with(".json")
                    {
                        zip_geojson_files.push(path.clone());
                    } else {
                        let msg = format!("Unsupported file in ZIP: {}", internal_path);
                        log::error!("{}", msg);
                        return Err(Error::InvalidPath(msg));
                    }
                }
            } else {
                // Regular file path
                let path_buf = PathBuf::from_str(path).unwrap();
                if let Some(ext) = path_buf.extension() {
                    match ext.to_str() {
                        Some("gml") => gml_files.push(path_buf),
                        Some("geojson") | Some("json") => geojson_files.push(path_buf),
                        _ => {
                            let msg = format!("Unsupported file extension: {ext:?}");
                            log::error!("{msg}");
                            return Err(Error::InvalidPath(msg));
                        }
                    }
                }
            }
        }

        // 混在チェック
        let has_gml = !gml_files.is_empty() || !zip_gml_files.is_empty();
        let has_geojson = !geojson_files.is_empty() || !zip_geojson_files.is_empty();

        if has_gml && has_geojson {
            let msg = "Cannot mix GML and GeoJSON files in a single conversion";
            log::error!("{msg}");
            return Err(Error::InvalidSetting(msg.to_string()));
        }

        // 適切なソースプロバイダーを選択
        let source_provider: Box<dyn DataSourceProvider> = if has_geojson {
            // Combine regular and ZIP GeoJSON files
            let mut all_geojson_files = geojson_files;
            for zip_path in zip_geojson_files {
                all_geojson_files.push(PathBuf::from(zip_path));
            }
            Box::new(GeoJsonSourceProvider {
                filenames: all_geojson_files,
            })
        } else {
            // Combine regular and ZIP GML files
            let mut all_gml_files = gml_files;
            for zip_path in zip_gml_files {
                all_gml_files.push(PathBuf::from(zip_path));
            }
            Box::new(CityGmlSourceProvider {
                filenames: all_gml_files,
            })
        };

        let mut source_params = source_provider.sink_options();
        if let Err(err) = source_params.validate() {
            let msg = format!("Error validating source parameters: {err:?}");
            log::error!("{msg}");
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
                let msg = format!("Error reading mapping rules file: {e}");
                log::error!("{msg}");
                Error::InvalidMappingRules(msg)
            })?;
            let mapping_rules: MappingRules =
                serde_json::from_str(&file_contents).map_err(|e| {
                    let msg = format!("Error parsing mapping rules: {e}");
                    log::error!("{msg}");
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
                    app.emit("conversion-log", LogMessage::from(&msg)).unwrap();
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
fn get_transform(filetype: String) -> Result<TransformerSettings, Error> {
    let sink_provider = select_sink_provider(&filetype).ok_or_else(|| {
        let msg = format!("Invalid sink type: {filetype}");
        log::error!("{msg}");
        Error::InvalidSetting(msg)
    })?;

    let transformer_settings = sink_provider.transformer_options();

    Ok(transformer_settings)
}

/// Get the configurable parameters of the sink
#[tauri::command]
fn get_parameter(filetype: String) -> Result<Parameters, Error> {
    let sink_provider = select_sink_provider(&filetype).ok_or_else(|| {
        let msg = format!("Invalid sink type: {filetype}");
        log::error!("{msg}");
        Error::InvalidSetting(msg)
    })?;
    let sink_params = sink_provider.sink_options();

    Ok(sink_params)
}

/// List supported files inside a ZIP archive
fn list_files_in_zip(zip_path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(zip_path)?;
    let reader = BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| Error::Io(e.to_string()))?;

    let mut files = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i).map_err(|e| Error::Io(e.to_string()))?;
        let file_path = file.name();

        // Check if the file has a supported extension
        if file_path.ends_with(".gml")
            || file_path.ends_with(".geojson")
            || file_path.ends_with(".json")
        {
            // Format: "/path/to/zipfile.zip/path/to/file.gml"
            let full_path = format!("{}/{}", zip_path, file_path);
            files.push(full_path);
        }
    }

    Ok(files)
}

/// List files in a ZIP archive (Tauri command)
#[tauri::command]
async fn list_zip_contents(zip_path: String) -> Result<Vec<String>, Error> {
    list_files_in_zip(&zip_path)
}

/// List supported files in the given directories
#[tauri::command]
async fn list_supported_files(directories: Vec<String>) -> Result<Vec<String>, Error> {
    let mut all_files = Vec::new();

    for directory in directories {
        let dir_path = PathBuf::from(&directory);

        if !dir_path.exists() {
            let msg = format!("Directory does not exist: {directory}");
            log::warn!("{msg}");
            continue;
        }

        if !dir_path.is_dir() {
            let msg = format!("Path is not a directory: {directory}");
            log::warn!("{msg}");
            continue;
        }

        // Read directory contents
        match std::fs::read_dir(&dir_path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();

                    // Check if it's a file and has supported extension
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            let ext_str = extension.to_string_lossy().to_lowercase();

                            // Check for directly supported files
                            if ext_str == "gml" || ext_str == "geojson" || ext_str == "json" {
                                if let Some(path_str) = path.to_str() {
                                    all_files.push(path_str.to_string());
                                }
                            }
                            // Check for ZIP files
                            else if ext_str == "zip" {
                                if let Some(path_str) = path.to_str() {
                                    // List files inside ZIP
                                    if let Ok(files) = list_files_in_zip(path_str) {
                                        all_files.extend(files);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                let msg = format!("Failed to read directory {directory}: {e}");
                log::error!("{msg}");
                return Err(Error::Io(msg));
            }
        }
    }

    all_files.sort();
    Ok(all_files)
}

const PLATEAU_TYPES: &[&str] = &[
    "bldg", "tran", "rwy", "trk", "squr", "wwy", "luse", "fld", "tnm", "htd", "ifld", "rfld",
    "lsld", "urf", "brid", "tun", "cons", "frn", "unf", "ubld", "veg", "dem", "wtr", "area", "gen",
    "app", "ext",
];

fn extract_meshcode_and_type(filename: &str) -> Option<(String, String)> {
    let basename = filename.split('/').next_back().unwrap_or(filename);

    // PLATEAU filename pattern: {meshcode}_{type}_{crs}_[..._optionals].gml
    // Example: 53394529_bldg_6697_op.gml
    let parts: Vec<&str> = basename.split('_').collect();

    if parts.len() <= 2 {
        return None;
    }

    let meshcode = parts[0];
    let meshcode_len = meshcode.len();

    if (meshcode_len != 6 && meshcode_len != 8) || !meshcode.chars().all(|c| c.is_numeric()) {
        // Meshcode must be 6 or 8 digits long and numeric
        return None;
    }

    let file_type = parts[1];

    if PLATEAU_TYPES.contains(&file_type) {
        Some((meshcode.to_string(), file_type.to_string()))
    } else {
        None
    }
}

#[tauri::command]
fn get_meshcodes_with_prefix(
    input_paths: Vec<String>,
) -> Result<HashMap<String, HashMap<String, Vec<String>>>, Error> {
    let mut result: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for input_path in input_paths {
        let file = File::open(input_path.clone())?;
        let reader = BufReader::new(file);
        let mut archive = zip::ZipArchive::new(reader).map_err(|e| Error::Io(e.to_string()))?;

        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| Error::Io(e.to_string()))?;
            let file_path = file.name().to_string();

            if file_path.ends_with(".gml") {
                if let Some((meshcode, file_type)) = extract_meshcode_and_type(&file_path) {
                    let meshcode_entry = result.entry(meshcode).or_default();
                    let type_entry = meshcode_entry.entry(file_type).or_default();
                    // Format: "/path/to/zipfile.zip/path/to/file.gml"
                    let full_path = format!("{}/{}", input_path, file_path);
                    type_entry.push(full_path);
                }
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_meshcode_and_type() {
        // Test valid PLATEAU filenames with 8-digit meshcodes
        assert_eq!(
            extract_meshcode_and_type("53393680_bldg_6697_lod4.2_op.gml"),
            Some(("53393680".to_string(), "bldg".to_string()))
        );

        assert_eq!(
            extract_meshcode_and_type("52385608_tran_6697_op.gml"),
            Some(("52385608".to_string(), "tran".to_string()))
        );

        assert_eq!(
            extract_meshcode_and_type("udx/fld/52385721_fld_6697_l1_op.gml"),
            Some(("52385721".to_string(), "fld".to_string()))
        );

        // Test valid PLATEAU filenames with 6-digit meshcodes (older format)
        assert_eq!(
            extract_meshcode_and_type("523855_tnm_6697_op.gml"),
            Some(("523855".to_string(), "tnm".to_string()))
        );

        assert_eq!(
            extract_meshcode_and_type("533915_urf_6668_kuiki_op.gml"),
            Some(("533915".to_string(), "urf".to_string()))
        );

        // Test invalid patterns
        assert_eq!(extract_meshcode_and_type("invalid_filename.gml"), None);
        assert_eq!(extract_meshcode_and_type("12345_bldg_6697.gml"), None); // Only 5 digits
        assert_eq!(extract_meshcode_and_type("1234567_bldg_6697.gml"), None); // 7 digits
        assert_eq!(extract_meshcode_and_type("123456789_bldg_6697.gml"), None); // 9 digits
        assert_eq!(extract_meshcode_and_type("12345678_unknown_6697.gml"), None); // Unknown type
        assert_eq!(extract_meshcode_and_type("building.gml"), None); // No underscore pattern
    }
}
