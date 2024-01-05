use nusamai::sink::serde::SerdeSinkProvider;
use nusamai::sink::DataSinkProvider;
use nusamai::source::citygml::CityGMLSourceProvider;
use nusamai::source::DataSourceProvider;
use nusamai::transform::NoopTransformer;

#[test]
fn run_serde_sink() {
    let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGMLSourceProvider {
        filenames: vec![
            "../nusamai-plateau/tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml"
                .to_string(),
        ],
    });
    let sink_provider: Box<dyn DataSinkProvider> = Box::new(SerdeSinkProvider {});

    let source = source_provider.create(&source_provider.parameters());
    let transformer = Box::new(NoopTransformer {});
    let mut sink_params = sink_provider.parameters();
    sink_params
        .update_values_with_str(std::iter::once(&("@output".into(), "/dev/null".into())))
        .unwrap();
    let sink = sink_provider.create(&sink_params);

    // start the pipeline
    let (handle, _watcher, _canceller) = nusamai::pipeline::run(source, transformer, sink);

    // wait for the pipeline to finish
    handle.join();
}
