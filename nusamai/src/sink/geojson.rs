//! GeoJSON sink

use std::fs::File;
use std::io::{BufWriter, Write};

use rayon::prelude::*;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use nusamai_geojson::toplevel_cityobj_to_geojson_features;

pub struct GeoJsonSinkProvider {}

impl DataSinkProvider for GeoJsonSinkProvider {
    fn create(&self, _config: &Config) -> Box<dyn DataSink> {
        Box::<GeoJsonSink>::default()
    }

    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "GeoJSON Sink".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

#[derive(Default)]
pub struct GeoJsonSink {
    n_features: usize,
}

impl DataSink for GeoJsonSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(100);

        rayon::join(
            || {
                // Convert TopLevelCityObjects to GeoJSON objects

                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        let features = toplevel_cityobj_to_geojson_features(&parcel.cityobj);

                        if sender.send(features).is_err() {
                            println!("sink cancelled");
                            return Err(());
                        };

                        Ok(())
                    },
                );
            },
            || {
                // Write GeoJSON to a file

                // TODO: Handle output file path
                let mut file = File::create("output.geojson").unwrap();
                let mut writer = BufWriter::new(&mut file);

                // Write the FeatureCollection header
                writer
                    .write_all(b"{\"type\":\"FeatureCollection\",\"features\":[")
                    .unwrap();

                // Write each Feature
                let mut iter = receiver.into_iter().flatten().peekable();
                while let Some(feat) = iter.next() {
                    serde_json::to_writer(&mut writer, &feat).unwrap();
                    if iter.peek().is_some() {
                        writer.write_all(b",").unwrap();
                    };
                }

                // Write the FeautureCollection footer and EOL
                writer.write_all(b"]}\n").unwrap();

                println!("Wrote {} features", self.n_features);
            },
        );
    }
}
