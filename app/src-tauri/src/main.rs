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
use reqwest::StatusCode;
use std::collections::{HashMap, HashSet};
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
use tokio::time::{sleep, Duration};

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

struct ConversionTasksState {
    canceller: Arc<Mutex<Canceller>>,
}

const DEFAULT_PLATEAU_API_BASE: &str = "https://api.plateauview.mlit.go.jp";

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
            fetch_citygml_metadata,
            download_citygml_pack,
            pack_and_run_conversion,
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
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Timeout: {0}")]
    Timeout(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err.to_string())
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

#[derive(Debug, serde::Deserialize)]
struct RawFeatureTypeInfo {
    #[serde(default)]
    name: String,
}

#[derive(Debug, serde::Deserialize)]
struct RawCityGmlFile {
    #[serde(default)]
    code: String,
    #[serde(default)]
    url: String,
    #[serde(rename = "maxLod")]
    max_lod: Option<i32>,
    #[serde(rename = "fileSize")]
    file_size: Option<u64>,
    #[serde(default)]
    features: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
struct RawCityGmlMetadataResponse {
    #[serde(default)]
    cities: Vec<RawCityEntry>,
    #[serde(rename = "featureTypes", default)]
    feature_types: HashMap<String, RawFeatureTypeInfo>,
}

#[derive(Debug, serde::Deserialize)]
struct RawCityEntry {
    #[serde(default)]
    files: HashMap<String, Vec<RawCityGmlFile>>,
}

#[derive(Debug, serde::Serialize, Clone)]
struct CityGmlRemoteFile {
    meshcode: String,
    #[serde(rename = "featureType")]
    feature_type: String,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxLod")]
    max_lod: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fileSize")]
    file_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
struct FeatureTypeInfo {
    label: String,
    #[serde(rename = "fileCount")]
    file_count: usize,
    files: Vec<CityGmlRemoteFile>,
}

#[derive(Debug, serde::Serialize)]
struct FetchCityGmlMetadataResult {
    #[serde(rename = "featureTypes")]
    feature_types: HashMap<String, FeatureTypeInfo>,
    meshes: HashMap<String, HashMap<String, Vec<CityGmlRemoteFile>>>,
}

#[derive(Debug, serde::Deserialize)]
struct PackResponse {
    id: String,
}

#[derive(Debug, serde::Deserialize)]
struct PackStatusResponse {
    status: String,
    #[serde(default)]
    progress: Option<f32>,
}

#[derive(Debug, serde::Serialize)]
struct DownloadCityGmlPackResult {
    #[serde(rename = "packId")]
    pack_id: String,
    #[serde(rename = "zipPath")]
    zip_path: String,
}

fn plateau_api_base_url() -> String {
    env::var("PLATEAU_API_BASE").unwrap_or_else(|_| DEFAULT_PLATEAU_API_BASE.to_string())
}

fn emit_pack_progress(app: &tauri::AppHandle, stage: &str, status: &str, progress: f32) {
    // conversion-log へ進捗を出力
    let percent = (progress * 100.0).clamp(0.0, 100.0);
    let level = match status {
        "failed" => "ERROR",
        "succeeded" => "INFO",
        "accepted" | "processing" | "checking" | "started" => "INFO",
        _ => "INFO",
    };

    let message = match stage {
        "request" => format!("パック作成リクエスト受理 status={status}"),
        "status" => format!("パック作成ステータス確認 status={status} progress={percent:.1}%"),
        "download" => match status {
            "started" => "パックZIPダウンロード開始".to_string(),
            "completed" => "パックZIPダウンロード完了".to_string(),
            other => format!("パックZIPダウンロード進行 status={other} progress={percent:.1}%"),
        },
        _ => format!("パック進行 stage={stage} status={status} progress={percent:.1}%"),
    };

    let _ = app.emit(
        "conversion-log",
        LogMessage {
            message,
            level: level.to_string(),
            error_message: None,
            source: "download_citygml_pack".to_string(),
        },
    );
}

fn normalize_meshcode(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let len = trimmed.len();
    if !(6..=10).contains(&len) {
        return None;
    }
    if !trimmed.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(trimmed.to_string())
}

fn is_included_meshcode(meshcode: &str, meshcodes: &[String], strict: bool) -> bool {
    if strict {
        meshcodes.iter().any(|c| c == meshcode)
    } else {
        meshcodes.iter().any(|c| meshcode.starts_with(c))
    }
}

pub fn compress_meshcodes(meshcodes: &[String], limit: usize) -> Vec<String> {
    if meshcodes.len() <= limit {
        return meshcodes.to_vec();
    }

    // まず 6桁に縮約
    let six = collapse_to_prefix(meshcodes, 6);
    if six.len() <= limit {
        return six;
    }

    // 次に 4桁へ縮約
    collapse_to_prefix(meshcodes, 4)
}

fn collapse_to_prefix(codes: &[String], prefix_len: usize) -> Vec<String> {
    let mut set: HashSet<String> = HashSet::with_capacity(codes.len());
    for code in codes {
        if code.len() >= prefix_len {
            set.insert(code[..prefix_len].to_string());
        } else {
            // 既に短いコードはそのまま (例えば4桁丸め時に4桁が入ってきたら維持)
            set.insert(code.clone());
        }
    }
    set.into_iter().collect()
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
        } else if !PathBuf::from_str(path).unwrap().exists() {
            let msg = format!("Input file does not exist: {path}");
            log::error!("{msg}");
            return Err(Error::InvalidPath(msg));
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
                        let msg = format!("Unsupported file in ZIP: {internal_path}");
                        log::error!("{msg}");
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
            let full_path = format!("{zip_path}/{file_path}");
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
                    let full_path = format!("{input_path}/{file_path}");
                    type_entry.push(full_path);
                }
            }
        }
    }
    Ok(result)
}

#[tauri::command(async)]
async fn fetch_citygml_metadata(
    meshcodes: Vec<String>,
    strict: bool,
) -> Result<FetchCityGmlMetadataResult, Error> {
    if meshcodes.len() == 0 {
        return Err(Error::InvalidSetting(
            "メッシュコードが指定されていません。".to_string(),
        ));
    }

    // MeshcodeをAPIが受け入れ可能な数に圧縮
    let compressed_meshcodes = compress_meshcodes(&meshcodes, 30);

    let conditions = format!("m:{}", compressed_meshcodes.join(","));

    let base_url = plateau_api_base_url();
    let request_url = format!(
        "{}/datacatalog/citygml/{}",
        base_url.trim_end_matches('/'),
        conditions
    );

    log::info!("Fetching CityGML metadata: {request_url}");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|err| Error::Http(err.to_string()))?;

    let response = client
        .get(&request_url)
        .send()
        .await
        .map_err(|err| Error::Http(err.to_string()))?;

    match response.status() {
        StatusCode::NOT_FOUND => {
            return Err(Error::NotFound(format!(
                "条件に一致するCityGMLメタデータが見つかりません: {conditions}"
            )));
        }
        status if !status.is_success() => {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api(format!(
                "CityGMLメタデータの取得に失敗しました (status: {status}): {body}"
            )));
        }
        _ => {}
    }

    let raw: RawCityGmlMetadataResponse = response
        .json()
        .await
        .map_err(|err| Error::Http(err.to_string()))?;

    let mut meshes: HashMap<String, HashMap<String, Vec<CityGmlRemoteFile>>> = HashMap::new();
    let mut feature_type_files: HashMap<String, Vec<CityGmlRemoteFile>> = HashMap::new();
    for city in raw.cities {
        for (feature_type, files) in city.files {
            for file in files {
                if file.url.is_empty() {
                    continue;
                }
                if let Some(meshcode) = normalize_meshcode(&file.code) {
                    if is_included_meshcode(&meshcode, &meshcodes, strict) {
                        let entry = meshes.entry(meshcode.clone()).or_default();
                        let remote_file = CityGmlRemoteFile {
                            meshcode: meshcode.clone(),
                            feature_type: feature_type.clone(),
                            url: file.url.clone(),
                            max_lod: file.max_lod,
                            file_size: file.file_size,
                            features: file.features,
                        };
                        let feature_type_files_entry =
                            feature_type_files.entry(feature_type.clone()).or_default();
                        if !feature_type_files_entry
                            .iter()
                            .any(|existing| existing.url == remote_file.url)
                        {
                            feature_type_files_entry.push(remote_file.clone());
                        }
                        let type_entry = entry.entry(feature_type.clone()).or_default();
                        if !type_entry
                            .iter()
                            .any(|existing| existing.url == remote_file.url)
                        {
                            type_entry.push(remote_file);
                        }
                    }
                }
            }
        }
    }

    let feature_types = raw
        .feature_types
        .into_iter()
        .filter(|(code, _)| feature_type_files.contains_key(code))
        .map(|(code, info)| {
            let feature_type_file = feature_type_files.get(&code).unwrap();
            let feature_type_info = FeatureTypeInfo {
                label: info.name,
                file_count: feature_type_file.len(),
                files: feature_type_file.clone(),
            };
            (code, feature_type_info)
        })
        .collect();

    Ok(FetchCityGmlMetadataResult {
        feature_types,
        meshes,
    })
}

#[tauri::command(async)]
async fn download_citygml_pack(
    urls: Vec<String>,
    app: tauri::AppHandle,
) -> Result<DownloadCityGmlPackResult, Error> {
    if urls.is_empty() {
        return Err(Error::InvalidSetting(
            "CityGMLパックの対象URLが選択されていません。".to_string(),
        ));
    }

    let base_url = plateau_api_base_url();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|err| Error::Http(err.to_string()))?;

    emit_pack_progress(&app, "request", "accepted", 0.0);

    let request_body = serde_json::json!({ "urls": urls });
    let pack_response = client
        .post(format!("{}/citygml/pack", base_url.trim_end_matches('/')))
        .json(&request_body)
        .send()
        .await
        .map_err(|err| Error::Http(err.to_string()))?;

    if !pack_response.status().is_success() {
        let status = pack_response.status();
        let body = pack_response.text().await.unwrap_or_default();
        return Err(Error::Api(format!(
            "CityGMLパックの作成依頼に失敗しました (status: {status}): {body}"
        )));
    }

    let pack: PackResponse = pack_response
        .json()
        .await
        .map_err(|err| Error::Http(err.to_string()))?;
    let pack_id = pack.id;

    let status_url = format!(
        "{}/citygml/pack/{pack_id}/status",
        base_url.trim_end_matches('/')
    );

    let mut attempt = 0usize;
    let max_attempts = 120usize; // up to 10 minutes (120 * 5s)

    loop {
        attempt += 1;
        emit_pack_progress(&app, "status", "checking", 0.0);

        let status_response = client
            .get(&status_url)
            .send()
            .await
            .map_err(|err| Error::Http(err.to_string()))?;

        if !status_response.status().is_success() {
            let status = status_response.status();
            let body = status_response.text().await.unwrap_or_default();
            return Err(Error::Api(format!(
                "CityGMLパックのステータス取得に失敗しました (status: {status}): {body}"
            )));
        }

        let status_body: PackStatusResponse = status_response
            .json()
            .await
            .map_err(|err| Error::Http(err.to_string()))?;

        let progress = status_body.progress.unwrap_or(0.0);
        emit_pack_progress(
            &app,
            "status",
            status_body.status.as_str(),
            progress.clamp(0.0, 1.0),
        );

        match status_body.status.as_str() {
            "succeeded" => break,
            "failed" => {
                return Err(Error::Api(
                    "CityGMLパックの作成が失敗しました。".to_string(),
                ));
            }
            "accepted" | "processing" => {
                if attempt >= max_attempts {
                    return Err(Error::Timeout(
                        "CityGMLパックの作成がタイムアウトしました。".to_string(),
                    ));
                }
                sleep(Duration::from_secs(5)).await;
            }
            other => {
                return Err(Error::Api(format!(
                    "CityGMLパックのステータスが不正です: {other}"
                )));
            }
        }
    }

    emit_pack_progress(&app, "download", "started", 0.0);

    let download_url = format!(
        "{}/citygml/pack/{pack_id}.zip",
        base_url.trim_end_matches('/')
    );

    let mut download_response = client
        .get(&download_url)
        .send()
        .await
        .map_err(|err| Error::Http(err.to_string()))?;

    if download_response.status().is_redirection() {
        if let Some(location) = download_response
            .headers()
            .get(reqwest::header::LOCATION)
            .and_then(|v| v.to_str().ok())
        {
            download_response = client.get(location).send().await?;
        }
    }

    if !download_response.status().is_success() {
        let status = download_response.status();
        let body = download_response.text().await.unwrap_or_default();
        return Err(Error::Api(format!(
            "CityGMLパックのダウンロードに失敗しました (status: {status}): {body}"
        )));
    }

    let bytes = download_response
        .bytes()
        .await
        .map_err(|err| Error::Http(err.to_string()))?;

    let mut target_path = env::temp_dir();
    target_path.push(format!("plateau-pack-{pack_id}.zip"));

    std::fs::write(&target_path, bytes.as_ref()).map_err(|err| Error::Io(err.to_string()))?;

    emit_pack_progress(&app, "download", "completed", 1.0);

    Ok(DownloadCityGmlPackResult {
        pack_id,
        zip_path: target_path.to_string_lossy().to_string(),
    })
}

#[tauri::command(async)]
#[allow(clippy::too_many_arguments)]
async fn pack_and_run_conversion(
    urls: Vec<String>,
    output_path: String,
    filetype: String,
    epsg: u16,
    rules_path: String,
    transformer_settings: TransformerSettings,
    sink_parameters: Parameters,
    tasks_state: tauri::State<'_, ConversionTasksState>,
    app: tauri::AppHandle,
) -> Result<(), Error> {
    // Cancel any previous task
    tasks_state.canceller.lock().unwrap().cancel();

    {
        let _ = app.emit(
            "conversion-log",
            LogMessage {
                message: "CityGMLパック処理を開始します".to_string(),
                level: "INFO".to_string(),
                error_message: None,
                source: "pack_and_run_conversion".to_string(),
            },
        );
    }

    // Start pack download
    let pack_result = match download_citygml_pack(urls.clone(), app.clone()).await {
        Ok(r) => {
            {
                let _ = app.emit(
                    "conversion-log",
                    LogMessage {
                        message: format!(
                            "CityGMLパックのダウンロードが完了しました packId={}",
                            r.pack_id
                        ),
                        level: "INFO".to_string(),
                        error_message: None,
                        source: "pack_and_run_conversion".to_string(),
                    },
                );
            }
            r
        }
        Err(e) => {
            {
                let _ = app.emit(
                    "conversion-log",
                    LogMessage {
                        message: format!("CityGMLパックの取得に失敗しました: {e}"),
                        level: "ERROR".to_string(),
                        error_message: None,
                        source: "pack_and_run_conversion".to_string(),
                    },
                );
            }
            return Err(e);
        }
    };

    // Check cancellation before conversion
    /*
    if tasks_state.canceller.lock().unwrap().is_canceled() {
        {
            let _ = app.emit(
                "conversion-log",
                LogMessage {
                    message: "キャンセル要求を検知しました（パック取得後）".to_string(),
                    level: "WARN".to_string(),
                    error_message: None,
                    source: "pack_and_run_conversion".to_string(),
                },
            );
        }
        return Err(Error::Canceled);
    }*/

    let packs = list_files_in_zip(&pack_result.zip_path.clone())?;
    println!("{:?}", packs);
    // Build input paths inside the downloaded pack zip
    let zip_path = pack_result.zip_path.clone(); // ends with .zip
    let mut input_paths: Vec<String> = Vec::new();

    for url in urls.iter() {
        if let Some(idx) = url.find("/udx/") {
            // "/udx/" の直前までを取得し、その最後のディレクトリ名を親ディレクトリとして付与する
            // 例: .../11203_kawaguchi-shi_pref_2024_citygml_1_op/udx/bldg/... -> 親: 11203_kawaguchi-shi_pref_2024_citygml_1_op
            let before_udx = &url[..idx]; // "/udx/" 開始位置より前
            let parent_dir = before_udx
                .trim_end_matches('/') // 末尾スラッシュ除去
                .rsplit('/')
                .next()
                .unwrap_or("");
            // internal は "udx/..." 部分
            let internal_udx = &url[idx + 1..]; // keep "udx/..."
            if internal_udx.is_empty() || !internal_udx.starts_with("udx/") || parent_dir.is_empty()
            {
                let _ = app.emit(
                    "conversion-log",
                    LogMessage {
                        message: format!(
                            "URL から内部パス (親/udx/...) を抽出できませんでした: {url}"
                        ),
                        level: "ERROR".to_string(),
                        error_message: None,
                        source: "pack_and_run_conversion".to_string(),
                    },
                );
                return Err(Error::InvalidPath(format!(
                    "内部パスが不正です (url={url})"
                )));
            }
            // Compose "<zip>.zip/<parent>/udx/..."
            let internal_with_parent = format!("{parent_dir}/{internal_udx}");
            let composite = format!("{zip_path}/{internal_with_parent}");
            input_paths.push(composite);
        } else {
            {
                let _ = app.emit(
                    "conversion-log",
                    LogMessage {
                        message: format!("URL に /udx/ が含まれていません: {url}"),
                        level: "ERROR".to_string(),
                        error_message: None,
                        source: "pack_and_run_conversion".to_string(),
                    },
                );
            }
            return Err(Error::InvalidPath(format!(
                "/udx/ が見つかりません (url={url})"
            )));
        }
    }

    {
        let _ = app.emit(
            "conversion-log",
            LogMessage {
                message: format!("変換対象ファイル数: {}", input_paths.len()),
                level: "INFO".to_string(),
                error_message: None,
                source: "pack_and_run_conversion".to_string(),
            },
        );
    }

    // Run conversion
    {
        let _ = app.emit(
            "conversion-log",
            LogMessage {
                message: "変換パイプラインを開始します".to_string(),
                level: "INFO".to_string(),
                error_message: None,
                source: "pack_and_run_conversion".to_string(),
            },
        );
    }
    run_conversion(
        input_paths,
        output_path,
        filetype,
        epsg,
        rules_path,
        transformer_settings,
        sink_parameters,
        tasks_state,
        app.clone(),
    )
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

    #[test]
    fn test_get_meshcodes_with_prefix() {
        // Use the test ZIP file from nusamai-plateau
        let zip_path = "../../nusamai-plateau/tests/data/kawasaki-shi.zip";

        // Skip test if the file doesn't exist (e.g., in CI environment)
        if !std::path::Path::new(zip_path).exists() {
            return;
        }

        let result = get_meshcodes_with_prefix(vec![zip_path.to_string()]).unwrap();

        // The result should contain some meshcodes and types
        assert!(
            !result.is_empty(),
            "Should find some meshcodes in the ZIP file"
        );

        // Verify the structure - each meshcode should map to types, and each type to file paths
        for (meshcode, type_map) in &result {
            assert!(!meshcode.is_empty(), "Meshcode should not be empty");
            assert!(!type_map.is_empty(), "Type map should not be empty");

            for (file_type, paths) in type_map {
                assert!(
                    PLATEAU_TYPES.contains(&file_type.as_str()),
                    "File type '{}' should be a valid PLATEAU type",
                    file_type
                );
                assert!(!paths.is_empty(), "Paths should not be empty");

                for path in paths {
                    assert!(
                        path.starts_with(zip_path),
                        "Path should start with the ZIP file path"
                    );
                    assert!(path.ends_with(".gml"), "Path should end with .gml");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_list_supported_files_with_zip() {
        // Use the test ZIP file from nusamai-plateau if it exists
        let zip_path = "../../nusamai-plateau/tests/data/kawasaki-shi.zip";

        if !std::path::Path::new(zip_path).exists() {
            return; // Skip test if file doesn't exist
        }

        let parent_dir = std::path::Path::new(zip_path).parent().unwrap();
        let directories = vec![parent_dir.to_str().unwrap().to_string()];
        let result = list_supported_files(directories).await.unwrap();

        // Should find files inside the ZIP
        assert!(
            result.iter().any(|f| f.contains(".zip/")),
            "Should find files inside ZIP archives"
        );

        // Verify that ZIP file paths are properly formatted
        for file_path in &result {
            if file_path.contains(".zip/") {
                assert!(
                    file_path.ends_with(".gml")
                        || file_path.ends_with(".geojson")
                        || file_path.ends_with(".json"),
                    "ZIP file should end with supported extension: {}",
                    file_path
                );
            }
        }

        // Verify that results are sorted
        let mut sorted_result = result.clone();
        sorted_result.sort();
        assert_eq!(result, sorted_result);
    }

    #[test]
    fn test_compress_meshcodes() {
        let meshcodes = vec![
            "53394529".to_string(),
            "53394530".to_string(),
            "53394600".to_string(),
            "53394700".to_string(),
            "53394800".to_string(),
            "53394900".to_string(),
            "53395000".to_string(),
            "53395100".to_string(),
            "53395200".to_string(),
            "53395300".to_string(),
            "53395400".to_string(),
        ];

        // Test with limit greater than number of meshcodes
        let compressed = compress_meshcodes(&meshcodes, 20);
        assert_eq!(compressed.len(), meshcodes.len());

        // Test with limit less than number of meshcodes
        let compressed = compress_meshcodes(&meshcodes, 5);
        assert!(compressed.iter().all(|code| code.len() == 4));
        assert!(compressed.len() <= 5);
    }
}
