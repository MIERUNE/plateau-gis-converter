//! GeoPackage sink

use std::path::PathBuf;

use url::Url;

use rayon::prelude::*;

use crate::parameters::Parameters;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use crate::get_parameter_value;
use crate::parameters::*;
use nusamai_gpkg::geometry::multipolygon_to_bytes;
use nusamai_gpkg::GpkgHandler;

pub struct GpkgSinkProvider {}

impl DataSinkProvider for GpkgSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "GeoPackage".to_string(),
        }
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(
            "@output".into(),
            ParameterEntry {
                description: "Output file path".into(),
                required: true,
                parameter: ParameterType::FileSystemPath(FileSystemPathParameter {
                    value: None,
                    must_exist: false,
                }),
            },
        );
        params
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath).unwrap();

        Box::<GpkgSink>::new(GpkgSink {
            output_path: output_path.clone(),
        })
    }
}

#[derive(Default)]
pub struct GpkgSink {
    output_path: PathBuf,
}

impl GpkgSink {
    pub async fn run_async(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        let mut handler = if self.output_path.to_string_lossy().starts_with("sqlite:") {
            GpkgHandler::from_url(&Url::parse(self.output_path.to_str().unwrap()).unwrap())
                .await
                .unwrap()
        } else {
            GpkgHandler::from_url(
                &Url::parse(&format!("sqlite://{}", self.output_path.to_str().unwrap())).unwrap(),
            )
            .await
            .unwrap()
        };

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