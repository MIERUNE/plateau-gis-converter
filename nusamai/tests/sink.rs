use std::{path::PathBuf, str::FromStr, sync::Once};

use nusamai::{
    sink,
    sink::DataSinkProvider,
    source::{citygml::CityGmlSourceProvider, DataSourceProvider},
    transformer::{MultiThreadTransformer, NusamaiTransformBuilder, TransformBuilder},
};
use nusamai_citygml::CityGmlElement;
use nusamai_plateau::models::TopLevelCityObject;

static INIT: Once = Once::new();

pub(crate) fn simple_run_sink<S: DataSinkProvider>(sink_provider: S, output: Option<&str>) {
    INIT.call_once(|| {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "error")
        }
        pretty_env_logger::init();
    });

    let filenames = [
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/rwy/53395527_rwy_6697.gml",
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/brid/dorokyo_51324378_brid_6697.gml",
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/trk/53361601_trk_6697.gml",
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/tun/53361613_tun_6697.gml",
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/veg/52385628_veg_6697_op.gml",
        "../nusamai-plateau/tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml",
        "../nusamai-plateau/tests/data/yokosuka-shi/udx/bldg/52397519_bldg_6697_op.gml",
        "../nusamai-plateau/tests/data/numazu-shi/udx/tran/52385608_tran_6697_op.gml",
    ];

    let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGmlSourceProvider {
        filenames: filenames
            .iter()
            .map(|name| PathBuf::from_str(name).unwrap())
            .collect(),
    });
    assert_eq!(source_provider.info().name, "CityGML");

    let source = source_provider.create(&source_provider.parameters());

    let sink = {
        assert!(!sink_provider.info().name.is_empty());
        let mut sink_params = sink_provider.parameters();
        if let Some(output) = output {
            sink_params
                .update_values_with_str(std::iter::once(&("@output".into(), output.into())))
                .unwrap();
        }
        sink_params.validate().unwrap();
        sink_provider.create(&sink_params)
    };

    let (transformer, schema) = {
        let transform_req = sink.make_requirements();
        let transform_builder = NusamaiTransformBuilder::new(transform_req.into());
        let mut schema = nusamai_citygml::schema::Schema::default();
        TopLevelCityObject::collect_schema(&mut schema);
        transform_builder.transform_schema(&mut schema);
        let transformer = Box::new(MultiThreadTransformer::new(transform_builder));
        (transformer, schema)
    };

    let (handle, watcher, canceller) =
        nusamai::pipeline::run(source, transformer, sink, schema.into());
    handle.join();

    for msg in watcher {
        println!("Feedback message from the pipeline {:?}", msg);
    }

    // should not be canceled
    assert!(!canceller.is_canceled());
}

#[test]
fn run_serde_sink() {
    simple_run_sink(sink::serde::SerdeSinkProvider {}, "/dev/null".into());
}

#[test]
fn run_czml_sink() {
    simple_run_sink(sink::czml::CzmlSinkProvider {}, "/dev/null".into());
}

#[test]
fn run_gltf_sink() {
    simple_run_sink(sink::gltf::GltfSinkProvider {}, "/tmp/nusamai/gltf".into());
}

#[test]
fn run_noop_sink() {
    simple_run_sink(sink::noop::NoopSinkProvider {}, None);
}

#[test]
fn run_geojson_sink() {
    simple_run_sink(
        sink::geojson::GeoJsonSinkProvider {},
        "/tmp/nusamai/geojson".into(),
    );
}

#[test]
fn run_gpkg_sink() {
    simple_run_sink(sink::gpkg::GpkgSinkProvider {}, "sqlite::memory:".into());
}

#[test]
fn run_mvt_sink() {
    simple_run_sink(sink::mvt::MvtSinkProvider {}, "/tmp/nusamai/mvt/".into());
}

#[test]
fn run_shapefile_sink() {
    simple_run_sink(
        sink::shapefile::ShapefileSinkProvider {},
        "/tmp/nusamai/shapefile".into(),
    );
}

#[test]
fn run_ply_sink() {
    simple_run_sink(sink::ply::StanfordPlySinkProvider {}, "/dev/null".into());
}

#[test]
fn run_3dtiles_sink() {
    simple_run_sink(
        sink::cesiumtiles::CesiumTilesSinkProvider {},
        "/tmp/nusamai/3dtiles/".into(),
    );
}
