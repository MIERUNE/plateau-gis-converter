//! GeoPackage sink

use rayon::prelude::*;
use tokio;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use nusamai_gpkg::geometry::multipolygon_to_bytes;
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
pub struct GpkgSink {}

impl GpkgSink {
    pub async fn run_async(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        let mut handler = GpkgHandler::init("output.gpkg").await.unwrap();

        let (sender, mut receiver) = tokio::sync::mpsc::channel(100);

        let producers = {
            let feedback = feedback.clone();
            tokio::task::spawn_blocking(move || {
                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }
                        let cityobj = parcel.cityobj;
                        if !cityobj.geometries.multipolygon.is_empty() {
                            let bytes = multipolygon_to_bytes(
                                &cityobj.geometries.vertices,
                                &cityobj.geometries.multipolygon,
                                4326,
                            );
                            if sender.blocking_send(bytes).is_err() {
                                return Err(());
                            };
                        }
                        Ok(())
                    },
                );
            })
        };

        let mut tx = handler.begin().await.unwrap();
        while let Some(gpkg_bin) = receiver.recv().await {
            if feedback.is_cancelled() {
                return;
            }
            tx.insert_feature(&gpkg_bin).await;
        }
        tx.commit().await.unwrap();

        producers.await.unwrap();
    }
}

impl DataSink for GpkgSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(self.run_async(upstream, feedback));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
