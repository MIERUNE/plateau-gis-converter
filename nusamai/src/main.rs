use std::env;
use std::sync::{Arc, Mutex};

use clap::Parser;

use nusamai::pipeline::Canceller;
use nusamai::sink::{
    geojson::GeoJsonSinkProvider, geojson_transform_exp::GeoJsonTransformExpSinkProvider,
    gpkg::GpkgSinkProvider, mvt::MVTSinkProvider, noop::NoopSinkProvider, serde::SerdeSinkProvider,
    shapefile::ShapefileSinkProvider,
};
use nusamai::sink::{DataSink, DataSinkProvider};
use nusamai::source::citygml::CityGmlSourceProvider;
use nusamai::source::{DataSource, DataSourceProvider};
use nusamai::transformer::MultiThreadTransformer;
use nusamai::transformer::{NusamaiTransformBuilder, TransformBuilder};
use nusamai_citygml::CityGmlElement;
use nusamai_plateau::models::TopLevelCityObject;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    filenames: Vec<String>,

    /// Sink choice
    #[arg(value_enum, long)]
    sink: SinkChoice,

    /// Output path
    #[arg(long)]
    output: String,

    /// Options for the source
    #[arg(short = 'i', value_parser = parse_key_val)]
    sourceopt: Vec<(String, String)>,

    /// Options for the sink
    #[arg(short = 'o', value_parser = parse_key_val)]
    sinkopt: Vec<(String, String)>,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].into(), s[pos + 1..].into()))
}

#[derive(clap::ValueEnum, Clone)]
enum SinkChoice {
    Noop,
    Serde,
    Geojson,
    Gpkg,
    Mvt,
    GeojsonTransformExp,
    Shapefile,
}

impl SinkChoice {
    fn create(&self) -> Box<dyn DataSinkProvider> {
        match self {
            SinkChoice::Noop => Box::new(NoopSinkProvider {}),
            SinkChoice::Serde => Box::new(SerdeSinkProvider {}),
            SinkChoice::Geojson => Box::new(GeoJsonSinkProvider {}),
            SinkChoice::GeojsonTransformExp => Box::new(GeoJsonTransformExpSinkProvider {}),
            SinkChoice::Gpkg => Box::new(GpkgSinkProvider {}),
            SinkChoice::Mvt => Box::new(MVTSinkProvider {}),
            SinkChoice::Shapefile => Box::new(ShapefileSinkProvider {}),
        }
    }
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    let args = {
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

    let source = {
        let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGmlSourceProvider {
            filenames: args.filenames,
        });
        let mut source_params = source_provider.parameters();
        if let Err(err) = source_params.update_values_with_str(&args.sourceopt) {
            log::error!("Error parsing source parameters: {:?}", err);
            return;
        };
        if let Err(err) = source_params.validate() {
            log::error!("Error validating source parameters: {:?}", err);
            return;
        }
        source_provider.create(&source_params)
    };

    let sink = {
        let sink_provider = args.sink.create();
        let mut sink_params = sink_provider.parameters();
        if let Err(err) = sink_params.update_values_with_str(&args.sinkopt) {
            log::error!("Error parsing sink options: {:?}", err);
            return;
        };
        if let Err(err) = sink_params.validate() {
            log::error!("Error validating source parameters: {:?}", err);
            return;
        }
        sink_provider.create(&sink_params)
    };

    run(source, sink, &mut canceller);
}

fn run(
    source: Box<dyn DataSource>,
    sink: Box<dyn DataSink>,
    canceller: &mut Arc<Mutex<Canceller>>,
) {
    let total_time = std::time::Instant::now();

    // Prepare the transformer for the pipeline and transform the schema
    let (transformer, schema) = {
        let requirements = sink.make_transform_requirements();
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
    if canceller.lock().unwrap().is_cancelled() {
        log::info!("Pipeline cancelled");
    }

    log::info!("Total processing time: {:?}", total_time.elapsed());
}
