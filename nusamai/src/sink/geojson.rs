//! GeoJSON sink

use std::fs::File;
use std::io::BufWriter;

use rayon::prelude::*;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use geojson::{Feature, FeatureCollection, GeoJson};
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
                    (sender, Vec::<Feature>::new()),
                    |(sender, buf), parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        buf.clear();

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

                let all_features: Vec<Feature> = receiver.into_iter().flatten().collect();
                self.n_features = all_features.len();

                let feature_collection = FeatureCollection {
                    bbox: None,
                    features: all_features,
                    foreign_members: None,
                };
                let geojson_obj = GeoJson::from(feature_collection);

                // TODO: Handle output file path
                let mut file = File::create("output.geojson").unwrap();
                let mut writer = BufWriter::new(&mut file);
                serde_json::to_writer(&mut writer, &geojson_obj).unwrap();

                println!("Wrote {} features", self.n_features);
            },
        );
    }
}
