use nusamai::sink::DataSinkProvider;
use nusamai::source::citygml::CityGmlSourceProvider;
use nusamai::source::DataSourceProvider;
use nusamai::transformer::{self, MultiThreadTransformer};
use nusamai::transformer::{NusamaiTransformBuilder, TransformBuilder};

use nusamai::sink;
use nusamai_citygml::CityGmlElement;
use nusamai_plateau::models::TopLevelCityObject;

pub(crate) fn simple_run_sink<S: DataSinkProvider>(sink_provider: S, output: Option<&str>) {
    let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGmlSourceProvider {
        filenames: vec![
            "../nusamai-plateau/tests/data/plateau-3_0/udx/rwy/53395518_rwy_6697.gml".to_string(),
            "../nusamai-plateau/tests/data/plateau-3_0/udx/brid/51324378_brid_6697.gml".to_string(),
            "../nusamai-plateau/tests/data/plateau-3_0/udx/trk/53361601_trk_6697.gml".to_string(),
            "../nusamai-plateau/tests/data/plateau-3_0/udx/tun/53361613_tun_6697.gml".to_string(),
        ],
    });
    assert_eq!(source_provider.info().name, "CityGML");

    let source = source_provider.create(&source_provider.parameters());

    let transform_builder =
        NusamaiTransformBuilder::new(transformer::Requirements::default().into());
    let mut schema = nusamai_citygml::schema::Schema::default();
    TopLevelCityObject::collect_schema(&mut schema);
    transform_builder.transform_schema(&mut schema);
    let transformer = Box::new(MultiThreadTransformer::new(transform_builder));

    assert!(!sink_provider.info().name.is_empty());
    let mut sink_params = sink_provider.parameters();
    if let Some(output) = output {
        sink_params
            .update_values_with_str(std::iter::once(&("@output".into(), output.into())))
            .unwrap();
    }
    sink_params.validate().unwrap();
    let sink = sink_provider.create(&sink_params);

    let (handle, _watcher, canceller) =
        nusamai::pipeline::run(source, transformer, sink, schema.into());
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
fn run_mvt_sink() {
    simple_run_sink(sink::mvt::MVTSinkProvider {}, "/tmp/nusamai/mvt/".into());
}
