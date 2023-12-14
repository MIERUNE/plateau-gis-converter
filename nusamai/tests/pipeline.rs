use nusamai::configuration::Config;
use nusamai::pipeline;
use nusamai::pipeline::{
    feedback, Feedback, FeedbackMessage, Percel, Sender, Sink, SinkInfo, SinkProvider, Source,
    SourceInfo, SourceProvider, Transformer,
};
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
        for i in 0..100 {
            if feedback.is_cancelled() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(
                (5.0 + random::<f32>() * 10.0) as u64,
            ));
            let obj = Percel { dummy_value: i };
            feedback.send(FeedbackMessage {
                message: format!("generating: {:?}", obj),
            });
            sink.send(obj).unwrap();
        }
    }
}

struct DummyTransformer {}

impl Transformer for DummyTransformer {
    fn transform(&self, obj: &mut Percel, feedback: &feedback::Feedback) {
        std::thread::sleep(std::time::Duration::from_millis(
            (5.0 + random::<f32>() * 10.0) as u64,
        ));
        obj.dummy_value *= 5;
        feedback.send(FeedbackMessage {
            message: format!("transformed object: {:?}", obj),
        })
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
    fn feed(&mut self, percel: Percel, feedback: &mut Feedback) {
        std::thread::sleep(std::time::Duration::from_millis(
            (5.0 + random::<f32>() * 20.0) as u64,
        ));
        feedback.send(FeedbackMessage {
            message: format!("dummy sink received: {:?}", percel),
        })
    }
}

#[test]
fn test_run_pipeline() {
    let input_driver_factory: Box<dyn SourceProvider> = Box::new(DummySourceProvider {});
    let output_driver_factory: Box<dyn SinkProvider> = Box::new(DummySinkProvider {});

    let input_driver = input_driver_factory.create(&input_driver_factory.config());
    let transformer = Box::new(DummyTransformer {});
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
