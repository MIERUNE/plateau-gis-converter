// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nusamai::pipeline::Canceller;
use nusamai::sink::geojson::GeoJsonSinkProvider;
use nusamai::sink::DataSinkProvider;
use nusamai::source::citygml::CityGMLSourceProvider;
use nusamai::source::{DataSource, DataSourceProvider};
use nusamai::transform::DummyTransformer;
use std::sync::{Arc, Mutex};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn run(input_path: String, output_path: String, filetype: String) {
    let filenames = vec![input_path];

    let sinkopt: Vec<(String, String)> = vec![("@output".into(), output_path)];

    let mut canceller = Arc::new(Mutex::new(Canceller::default()));
    // {
    //     let canceller = canceller.clone();
    //     ctrlc::set_handler(move || {
    //         println!("request cancellation");
    //         canceller.lock().unwrap().cancel();
    //     })
    //     .expect("Error setting Ctrl-C handler");
    // }

    let source = {
        let source_provider: Box<dyn DataSourceProvider> =
            Box::new(CityGMLSourceProvider { filenames });
        let mut source_params = source_provider.parameters();
        // if let Err(err) = source_params.update_values_with_str(&args.sourceopt) {
        //     eprintln!("Error parsing source parameters: {:?}", err);
        //     return;
        // };
        if let Err(err) = source_params.validate() {
            eprintln!("Error validating source parameters: {:?}", err);
            return;
        }
        source_provider.create(&source_params)
    };

    let sink = {
        let sink_provider = Box::new(GeoJsonSinkProvider {}); //args.sink.create();
        let mut sink_params = sink_provider.parameters();
        if let Err(err) = sink_params.update_values_with_str(&sinkopt) {
            eprintln!("Error parsing sink options: {:?}", err);
            return;
        };
        if let Err(err) = sink_params.validate() {
            eprintln!("Error validating source parameters: {:?}", err);
            return;
        }
        sink_provider.create(&sink_params)
    };

    let transformer = Box::<DummyTransformer>::default();

    // start the pipeline
    let (handle, watcher, inner_canceller) = nusamai::pipeline::run(source, transformer, sink);
    *canceller.lock().unwrap() = inner_canceller;

    std::thread::scope(|scope| {
        // log watcher
        scope.spawn(move || {
            for msg in watcher {
                println!("Feedback message from the pipeline {:?}", msg);
            }
        });
    });

    // wait for the pipeline to finish
    handle.join();
    if canceller.lock().unwrap().is_cancelled() {
        println!("Pipeline cancelled");
    }
}
