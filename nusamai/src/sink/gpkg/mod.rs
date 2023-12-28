//! GeoPackage sink

use rayon::prelude::*;
use tokio;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use nusamai_gpkg::GpkgHandler;

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
        let (sender, receiver) = std::sync::mpsc::sync_channel(100);

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let handler = runtime.block_on(GpkgHandler::init("output.gpkg")).unwrap();

        let _ = upstream
            .into_iter()
            .par_bridge()
            .try_for_each_with(sender, |sender, parcel| {
                if feedback.is_cancelled() {
                    return Err(());
                }

                if !parcel.cityobj.geometries.multipolygon.is_empty() {
                    runtime.block_on(handler.add_multi_polygon_feature(
                        &parcel.cityobj.geometries.vertices,
                        &parcel.cityobj.geometries.multipolygon,
                    ));
                }

                if sender.send(parcel.cityobj).is_err() {
                    println!("sink cancelled");
                    return Err(());
                };

                Ok(())
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
