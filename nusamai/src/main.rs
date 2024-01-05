use std::sync::{Arc, Mutex};

use clap::Parser;

use nusamai::pipeline::Canceller;
use nusamai::sink::{
    geojson::GeoJsonSinkProvider, gpkg::GpkgSinkProvider, noop::NoopSinkProvider,
    serde::SerdeSinkProvider, tiling2d::Tiling2DSinkProvider,
};
use nusamai::sink::{DataSink, DataSinkProvider};
use nusamai::source::citygml::CityGMLSourceProvider;
use nusamai::source::{DataSource, DataSourceProvider};
use nusamai::transform::NoopTransformer;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, long)]
    sink: SinkChoice,

    #[arg()]
    filenames: Vec<String>,
}

#[derive(clap::ValueEnum, Clone)]
enum SinkChoice {
    Noop,
    Serde,
    Geojson,
    Gpkg,
    Tiling2d,
}

impl SinkChoice {
    fn create(&self) -> Box<dyn DataSinkProvider> {
        match self {
            SinkChoice::Noop => Box::new(NoopSinkProvider {}),
            SinkChoice::Serde => Box::new(SerdeSinkProvider {}),
            SinkChoice::Geojson => Box::new(GeoJsonSinkProvider {}),
            SinkChoice::Gpkg => Box::new(GpkgSinkProvider {}),
            SinkChoice::Tiling2d => Box::new(Tiling2DSinkProvider {}),
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut canceller = Arc::new(Mutex::new(Canceller::default()));
    {
        let canceller = canceller.clone();
        ctrlc::set_handler(move || {
            println!("request cancellation");
            canceller.lock().unwrap().cancel();
        })
        .expect("Error setting Ctrl-C handler");
    }

    let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGMLSourceProvider {
        filenames: args.filenames,
    });
    let sink_provider = args.sink.create();

    let source = source_provider.create(&source_provider.config());
    let sink = sink_provider.create(&sink_provider.config());

    run(source, sink, &mut canceller);
}

fn run(
    source: Box<dyn DataSource>,
    sink: Box<dyn DataSink>,
    canceller: &mut Arc<Mutex<Canceller>>,
) {
    let transformer = Box::new(NoopTransformer {});

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
