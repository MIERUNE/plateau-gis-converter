use nusamai::configuration::Config;
use nusamai::pipeline::{self, Parcel, Receiver, TransformError};
use nusamai::pipeline::{feedback, Feedback, FeedbackMessage, Sender, Transformer};
use nusamai::sink::{DataSink, DataSinkProvider, SinkInfo};
use nusamai::source::{DataSource, DataSourceProvider, SourceInfo};
use nusamai_plateau::TopLevelCityObject;
use rand::prelude::*;

pub struct DummySourceProvider {}

impl DataSourceProvider for DummySourceProvider {
    fn create(&self, _config: &Config) -> Box<dyn DataSource> {
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

impl DataSource for DummySource {
    fn run(&mut self, sink: Sender, feedback: &Feedback) {
        for _i in 0..100 {
            if feedback.is_cancelled() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(
                (5.0 + random::<f32>() * 10.0) as u64,
            ));
            let obj = Parcel {
                cityobj: TopLevelCityObject {
                    root: citygml::ObjectValue::Double(0.),
                    geometries: Default::default(),
                },
            };
            feedback.feedback(FeedbackMessage {
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
        parcel: Parcel,
        sender: &Sender,
        _feedback: &feedback::Feedback,
    ) -> Result<(), TransformError> {
        // no-op
        sender.send(parcel)?;
        Ok(())
    }
}

struct DummySinkProvider {}

impl DataSinkProvider for DummySinkProvider {
    fn create(&self, _config: &Config) -> Box<dyn DataSink> {
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

impl DataSink for DummySink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        for parcel in upstream {
            if feedback.is_cancelled() {
                return;
            }

            std::thread::sleep(std::time::Duration::from_millis(
                (5.0 + random::<f32>() * 20.0) as u64,
            ));
            feedback.feedback(FeedbackMessage {
                message: format!("dummy sink received: {:?}", parcel),
            })
        }
    }
}

#[test]
fn test_run_pipeline() {
    let input_driver_factory: Box<dyn DataSourceProvider> = Box::new(DummySourceProvider {});
    let output_driver_factory: Box<dyn DataSinkProvider> = Box::new(DummySinkProvider {});

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
