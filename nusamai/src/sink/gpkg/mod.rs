//! GeoPackage sink

use std::fs::File;
use std::io::{BufWriter, Write};

use rayon::prelude::*;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use nusamai_gpkg::GpkgHandler;
use nusamai_plateau::TopLevelCityObject;
use serde_json::Value;

pub struct GpkgSinkProvider {}

impl DataSinkProvider for GpkgSinkProvider {
    fn create(&self, _config: &Config) -> Box<dyn DataSink> {
        Box::<GpkgSink>::default()
    }

    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "GeoPackage Sink".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

#[derive(Default)]
pub struct GpkgSink {
    n_features: usize,
}

impl DataSink for GpkgSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        //     let (sender, receiver) = std::sync::mpsc::sync_channel(100);

        //     rayon::join(
        //         || {
        //             // Convert TopLevelCityObjects to GeoJSON objects

        //             let output_path = "output.gpkg";
        //             // let handler = nusamai_gpkg::GpkgHandler::init(output_path).await.unwrap();

        //             let _ = upstream.into_iter().par_bridge().try_for_each_with(
        //                 sender,
        //                 |sender, parcel| {
        //                     if feedback.is_cancelled() {
        //                         return Err(());
        //                     }

        //                     // TODO
        //                     //
        //                     // let geometries = &parcel.cityobj.geometries;
        //                     //
        //                     // if !geometries.multipolygon.is_empty() {
        //                     //     handler
        //                     //         .add_multi_polygon_feature(
        //                     //             &geometries.vertices,
        //                     //             &geometries.multipolygon,
        //                     //         )
        //                     //         .await;
        //                     // }

        //                     // if sender.send(geometries).is_err() {
        //                     //     println!("sink cancelled");
        //                     //     return Err(());
        //                     // };

        //                     Ok(())
        //                 },
        //             );
        //         },
        //         || {
        //             receiver;
        //         },
        //     );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
