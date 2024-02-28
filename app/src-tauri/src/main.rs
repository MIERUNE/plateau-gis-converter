// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

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
use nusamai::transformer::{MappingRules, Requirements};
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

fn select_sink_provider(filetype: &str) -> Box<dyn DataSinkProvider> {
    // TODO: share possible options with the frontend types (src/lib/settings.ts)
    match filetype {
        "noop" => Box::new(nusamai::sink::noop::NoopSinkProvider {}),
        "serde" => Box::new(SerdeSinkProvider {}),
        "geojson" => Box::new(GeoJsonSinkProvider {}),
        "gpkg" => Box::new(GpkgSinkProvider {}),
        "mvt" => Box::new(MvtSinkProvider {}),
        "shapefile" => Box::new(ShapefileSinkProvider {}),
        "czml" => Box::new(CzmlSinkProvider {}),
        "kml" => Box::new(KmlSinkProvider {}),
        "gltf" => Box::new(GltfSinkProvider {}),
        "ply" => Box::new(StanfordPlySinkProvider {}),
        "cesiumtiles" => Box::new(CesiumTilesSinkProvider {}),
        _ => panic!("Unknown filetype: {}", filetype),
    }
}

#[tauri::command]
fn run(input_paths: Vec<String>, output_path: String, filetype: String, rules_path: String) {
    let sinkopt: Vec<(String, String)> = vec![("@output".into(), output_path)];

    log::info!("Running pipeline with input: {:?}", input_paths);

    // TODO: set cancellation handler
    let canceller = Arc::new(Mutex::new(Canceller::default()));

    let source = {
        let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGmlSourceProvider {
            filenames: input_paths
                .iter()
                .map(|s| PathBuf::from_str(s).unwrap())
                .collect(),
        });
        let mut source_params = source_provider.parameters();
        if let Err(err) = source_params.validate() {
            log::error!("Error validating source parameters: {:?}", err);
            return;
        }
        source_provider.create(&source_params)
    };

    let sink = {
        let sink_provider = select_sink_provider(&filetype);
        let mut sink_params = sink_provider.parameters();
        if let Err(err) = sink_params.update_values_with_str(&sinkopt) {
            log::error!("Error parsing sink options: {:?}", err);
            return;
        };
        if let Err(err) = sink_params.validate() {
            log::error!("Error validating source parameters: {:?}", err);
            return;
        }
        sink_provider.create(&sink_params)
    };

    let (transformer, schema) = {
        use nusamai_citygml::CityGmlElement;

        let requirements = if rules_path.is_empty() {
            sink.make_transform_requirements()
        } else {
            let file_contents = std::fs::read_to_string(&rules_path);
            if let Ok(contents) = file_contents {
                if let Ok(mapping_rules) = serde_json::from_str::<MappingRules>(&contents) {
                    Requirements {
                        mapping_rules: Some(mapping_rules),
                        ..sink.make_transform_requirements()
                    }
                } else {
                    log::error!("Error parsing rules file");
                    return;
                }
            } else {
                log::error!("Error reading rules file");
                return;
            }
        };

        let transform_builder = NusamaiTransformBuilder::new(requirements.into());
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
    }
}
