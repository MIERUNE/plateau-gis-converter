pub mod configuration;
pub mod pipeline;
use configuration::Config;
use pipeline::{
    feedback, Feedback, FeedbackMessage, Percel, Sender, Sink, SinkProvider, Source, SourceInfo,
    SourceProvider, Transformer,
};

pub struct DummySourceProvider {}

impl SourceProvider for DummySourceProvider {
    fn create(&self) -> Box<dyn Source> {
        Box::new(DummySource {})
    }

    fn info(&self) -> pipeline::SourceInfo {
        SourceInfo {
            name: "Dummy Source".to_string(),
        }
    }

    fn configuration(&self) -> Config {
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
            std::thread::sleep(std::time::Duration::from_millis(100));
            let obj = Percel { dummy_value: i };
            feedback.send(FeedbackMessage {
                message: format!("generating: {:?}", obj),
            });
            sink.send(obj).unwrap();
        }
    }
}

struct TransformerImpl {}

impl Transformer for TransformerImpl {
    fn transform(&self, obj: &mut Percel, feedback: &feedback::Feedback) {
        obj.dummy_value *= 5;
        feedback.send(FeedbackMessage {
            message: format!("transformed object: {:?}", obj),
        })
    }
}

struct DummySinkProvider {}

impl SinkProvider for DummySinkProvider {
    fn create(&self) -> Box<dyn Sink> {
        Box::new(DummySink {})
    }

    fn info(&self) -> pipeline::SinkInfo {
        pipeline::SinkInfo {
            name: "Dummy Sink".to_string(),
        }
    }

    fn configuration(&self) -> Config {
        Config::default()
    }
}

struct DummySink {}

impl Sink for DummySink {
    fn feed(&mut self, percel: Percel, feedback: &mut Feedback) {
        feedback.send(FeedbackMessage {
            message: format!("dummy sink received: {:?}", percel),
        })
    }
}

fn main() {
    let input_driver_factory: Box<dyn SourceProvider> = Box::new(DummySourceProvider {});
    let output_driver_factory: Box<dyn SinkProvider> = Box::new(DummySinkProvider {});

    let input_driver = input_driver_factory.create();
    let transformer = Box::new(TransformerImpl {});
    let output_driver = output_driver_factory.create();

    let (handle, watcher, canceller) = pipeline::run(input_driver, transformer, output_driver);

    std::thread::scope(|scope| {
        scope.spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(2000));
            canceller.cancel();
        });
        scope.spawn(move || {
            for msg in watcher {
                println!("Feedback message from the pipeline: {:?}", msg);
            }
        });
    });

    handle.join();
}
