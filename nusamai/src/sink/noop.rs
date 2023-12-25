//! No-op Sink (demo)
//!
//! This is a demo sync that only counts the number of features and vertices and does not generate any output.

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

pub struct NoopSinkProvider {}

impl DataSinkProvider for NoopSinkProvider {
    fn create(&self, _config: &Config) -> Box<dyn DataSink> {
        Box::new(NoopSink {
            num_features: 0,
            num_vertices: 0,
        })
    }

    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "Noop Sink".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

pub struct NoopSink {
    num_features: usize,
    num_vertices: usize,
}

impl DataSink for NoopSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        for parcel in upstream {
            if feedback.is_cancelled() {
                println!("sink cancelled");
                return;
            }
            self.num_features += 1;
            self.num_vertices += parcel.cityobj.geometries.vertices.len();

            println!("feature: {:?}", parcel.cityobj.root);
        }

        if feedback.is_cancelled() {
            return;
        }
        println!("total number of features: {:#?}", self.num_features);
        println!("total vertices: {}", self.num_vertices);
    }
}
