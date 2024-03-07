// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use thiserror::Error;

use nusamai::pipeline::Canceller;
use nusamai::sink::DataSinkProvider;
use nusamai::sink::{
    cesiumtiles::CesiumTilesSinkProvider, czml::CzmlSinkProvider, geojson::GeoJsonSinkProvider,
    gltf::GltfSinkProvider, gpkg::GpkgSinkProvider, kml::KmlSinkProvider, mvt::MvtSinkProvider,
    ply::StanfordPlySinkProvider, serde::SerdeSinkProvider, shapefile::ShapefileSinkProvider,
};
use nusamai::source::citygml::CityGmlSourceProvider;
use nusamai::source::DataSourceProvider;
use nusamai::transformer::MultiThreadTransformer;
use nusamai::transformer::{self, MappingRules};
use nusamai::transformer::{NusamaiTransformBuilder, TransformBuilder};
use nusamai_plateau::models::TopLevelCityObject;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid setting: {0}")]
    InvalidSetting(String),
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
        "ply" => Some(Box::new(StanfordPlySinkProvider {})),
        "cesiumtiles" => Some(Box::new(CesiumTilesSinkProvider {})),
        _ => None,
    }
}

// Everything returned from Tauri commands must implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn run(
    input_paths: Vec<String>,
    output_path: String,
    filetype: String,
    epsg: u16,
    rules_path: String,
) -> Result<(), Error> {
    let sinkopt: Vec<(String, String)> = vec![("@output".into(), output_path)];

    log::info!("Running pipeline with input: {:?}", input_paths);

    // TODO: set cancellation handler
    let canceller = Arc::new(Mutex::new(Canceller::default()));

    let sink = {
        let sink_provider = select_sink_provider(&filetype).ok_or_else(|| {
            let msg = format!("Invalid sink type: {}", filetype);
            log::error!("{}", msg);
            Error::InvalidSetting(msg)
        })?;

        let mut sink_params = sink_provider.parameters();
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

    let mut requirements = sink.make_requirements();
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
            // FIXME: error handling
            let file_contents =
                std::fs::read_to_string(rules_path).expect("Error reading rules file");
            let mapping_rules: MappingRules =
                serde_json::from_str(&file_contents).expect("Error parsing rules file");
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
    *canceller.lock().unwrap() = inner_canceller;

    std::thread::scope(|scope| {
        // log watcher
        scope.spawn(move || {
            for msg in watcher {
                log::info!("Feedback message from the pipeline {:?}", msg);
            }
        });
    });

    // wait for the pipeline to finish
    handle.join();
    if canceller.lock().unwrap().is_canceled() {
        log::info!("Pipeline canceled");
    };

    Ok(())
}
