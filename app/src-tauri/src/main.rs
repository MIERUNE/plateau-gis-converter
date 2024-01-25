// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::sync::{Arc, Mutex};

use nusamai::pipeline::Canceller;
use nusamai::sink::DataSinkProvider;
use nusamai::sink::{
    geojson::GeoJsonSinkProvider, gpkg::GpkgSinkProvider, mvt::MVTSinkProvider,
    serde::SerdeSinkProvider,
};
use nusamai::source::citygml::CityGmlSourceProvider;
use nusamai::source::DataSourceProvider;
use nusamai::transformer::builder::{NusamaiTransformBuilder, TransformBuilder};
use nusamai::transformer::runner::MultiThreadTransformer;
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

#[tauri::command]
fn run(input_paths: Vec<String>, output_path: String, filetype: String) {
    let sinkopt: Vec<(String, String)> = vec![("@output".into(), output_path)];

    log::info!("Running pipeline with input: {:?}", input_paths);

    // TODO: set cancellation handler
    let canceller = Arc::new(Mutex::new(Canceller::default()));

    let source = {
        let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGmlSourceProvider {
            filenames: input_paths,
        });
        let mut source_params = source_provider.parameters();
        if let Err(err) = source_params.validate() {
            log::error!("Error validating source parameters: {:?}", err);
            return;
        }
        source_provider.create(&source_params)
    };

    let sink = {
        // TODO: share with the frontend types (src/lib/settings.ts)
        let sink_provider: Box<dyn DataSinkProvider> = match &*filetype {
            "GeoJSON" => Box::new(GeoJsonSinkProvider {}),
            "GeoPackage" => Box::new(GpkgSinkProvider {}),
            "Serde" => Box::new(SerdeSinkProvider {}),
            "Vector Tiles" => Box::new(MVTSinkProvider {}),
            _ => {
                log::error!("Unknown filetype: {}", filetype);
                return;
            }
        };

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
        let transform_builder = NusamaiTransformBuilder::default();
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
    if canceller.lock().unwrap().is_cancelled() {
        log::info!("Pipeline cancelled");
    }
}
