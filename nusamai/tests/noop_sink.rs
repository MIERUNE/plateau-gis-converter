use nusamai::sink::noop::NoopSinkProvider;
use nusamai::sink::DataSinkProvider;
use nusamai::source::citygml::CityGMLSourceProvider;
use nusamai::source::DataSourceProvider;
use nusamai::transform::NoopTransformer;

#[test]
fn test_noop_sink() {
    let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGMLSourceProvider {
        filenames: vec![
            "../nusamai-plateau/tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml"
                .to_string(),
        ],
    });
    let sink_provider: Box<dyn DataSinkProvider> = Box::new(NoopSinkProvider {});

    let source = source_provider.create(&source_provider.config());
    let transformer = Box::new(NoopTransformer {});
    let sink = sink_provider.create(&sink_provider.config());

    // start the pipeline
    let (handle, _watcher, _canceller) = nusamai::pipeline::run(source, transformer, sink);

    // wait for the pipeline to finish
    handle.join();
}
