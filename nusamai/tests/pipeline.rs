use nusamai::configuration::Config;
use nusamai::pipeline::{self, TransformError};
use nusamai::pipeline::{
    feedback, Feedback, FeedbackMessage, Percel, Sender, Sink, SinkInfo, SinkProvider, Source,
    SourceInfo, SourceProvider, Transformer,
};
use nusamai_plateau::TopLevelCityObject;
use rand::prelude::*;

pub struct DummySourceProvider {}

impl SourceProvider for DummySourceProvider {
    fn create(&self, _config: &Config) -> Box<dyn Source> {
        Box::new(DummySource {})
    }

    fn info(&self) -> SourceInfo {
        SourceInfo {
            name: "Dummy Source".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

pub struct DummySource {}

impl Source for DummySource {
    fn run(&mut self, sink: Sender, feedback: &Feedback) {
        for _i in 0..100 {
            if feedback.is_cancelled() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(
                (5.0 + random::<f32>() * 10.0) as u64,
            ));
            let obj = Percel {
                cityobj: TopLevelCityObject {
                    root: citygml::ObjectValue::Double(0.),
                    geometries: Default::default(),
                },
            };
            feedback.send(FeedbackMessage {
                message: format!("generating: {:?}", obj),
            });
            if sink.send(obj).is_err() {
                break;
            }
        }
    }
}

struct NoopTransformer {}

impl Transformer for NoopTransformer {
    fn transform(
        &self,
        percel: Percel,
        sender: &Sender,
        _feedback: &feedback::Feedback,
    ) -> Result<(), TransformError> {
        // no-op
        sender.send(percel)?;
        Ok(())
    }
}

struct DummySinkProvider {}

impl SinkProvider for DummySinkProvider {
    fn create(&self, _config: &Config) -> Box<dyn Sink> {
        Box::new(DummySink {})
    }

    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "Dummy Sink".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

struct DummySink {}

impl Sink for DummySink {
    fn receive(&mut self, percel: Percel, feedback: &mut Feedback) {
        std::thread::sleep(std::time::Duration::from_millis(
            (5.0 + random::<f32>() * 20.0) as u64,
        ));
        feedback.send(FeedbackMessage {
            message: format!("dummy sink received: {:?}", percel),
        })
    }

    fn finalize(&mut self, _feedback: &mut Feedback) {
        // no-op
    }
}

#[test]
fn test_run_pipeline() {
    let input_driver_factory: Box<dyn SourceProvider> = Box::new(DummySourceProvider {});
    let output_driver_factory: Box<dyn SinkProvider> = Box::new(DummySinkProvider {});

    let input_driver = input_driver_factory.create(&input_driver_factory.config());
    let transformer = Box::new(NoopTransformer {});
    let output_driver = output_driver_factory.create(&input_driver_factory.config());

    // start the pipeline
    let (handle, watcher, canceller) = pipeline::run(input_driver, transformer, output_driver);

    std::thread::scope(|scope| {
        // cancel the pipeline
        scope.spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(300));
            canceller.cancel();
        });
        // log watcher
        let watcher_handle = scope.spawn(move || {
            let mut sink_counter = 0;
            for msg in watcher {
                println!("Feedback message from the pipeline: {:?}", msg);
                if msg.message.contains("dummy sink") {
                    sink_counter += 1;
                }
            }
            sink_counter
        });
        let sink_counter = watcher_handle.join().unwrap();
        assert!(sink_counter > 10); // sink should receive more than 10 objects
        assert!(sink_counter < 80); // pipeline should be cancelled before 50
    });

    // wait for the pipeline to finish
    handle.join();
}
