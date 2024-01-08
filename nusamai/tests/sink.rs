use nusamai::sink::DataSinkProvider;
use nusamai::source::citygml::CityGMLSourceProvider;
use nusamai::source::DataSourceProvider;
use nusamai::transform::DummyTransformer;

use nusamai::sink;

pub(crate) fn simple_run_sink<S: DataSinkProvider>(sink_provider: S, output: Option<&str>) {
    let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGMLSourceProvider {
        filenames: vec![
            "../nusamai-plateau/tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml"
                .to_string(),
        ],
    });
    assert_eq!(source_provider.info().name, "CityGML");

    let source = source_provider.create(&source_provider.parameters());

    let transformer = Box::<DummyTransformer>::default();

    assert!(!sink_provider.info().name.is_empty());
    let mut sink_params = sink_provider.parameters();
    if let Some(output) = output {
        sink_params
            .update_values_with_str(std::iter::once(&("@output".into(), output.into())))
            .unwrap();
    }
    sink_params.validate().unwrap();
    let sink = sink_provider.create(&sink_params);

    let (handle, _watcher, canceller) = nusamai::pipeline::run(source, transformer, sink);
    handle.join();

    // should not be cancelled
    assert!(!canceller.is_cancelled());
}

#[test]
fn run_serde_sink() {
    simple_run_sink(sink::serde::SerdeSinkProvider {}, "/dev/null".into());
}

#[test]
fn run_noop_sink() {
    simple_run_sink(sink::noop::NoopSinkProvider {}, None);
}

#[test]
fn run_geojson_sink() {
    simple_run_sink(sink::geojson::GeoJsonSinkProvider {}, "/dev/null".into());
}

#[test]
fn run_gpkg_sink() {
    simple_run_sink(sink::gpkg::GpkgSinkProvider {}, "sqlite::memory:".into());
}

#[test]
fn run_tiling2d_sink() {
    simple_run_sink(sink::tiling2d::Tiling2DSinkProvider {}, "/dev/null".into());
}
