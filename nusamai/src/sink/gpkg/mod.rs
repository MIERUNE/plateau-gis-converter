//! GeoPackage sink

use std::path::PathBuf;

use nusamai_citygml::schema::Schema;
use url::Url;

use rayon::prelude::*;

use crate::parameters::Parameters;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use crate::get_parameter_value;
use crate::parameters::*;
use nusamai_gpkg::geometry::write_indexed_multipolygon;
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

pub struct GpkgSink {
    output_path: PathBuf,
}

impl GpkgSink {
    pub async fn run_async(&mut self, upstream: Receiver, feedback: &Feedback) {
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
                        let entity = parcel.entity;
                        let geom_store = entity.geometry_store.read().unwrap();
                        if !geom_store.multipolygon.is_empty() {
                            let mut bytes = Vec::new();
                            if write_indexed_multipolygon(
                                &mut bytes,
                                &geom_store.vertices,
                                &geom_store.multipolygon,
                                4326,
                            )
                            .is_err()
                            {
                                // TODO: fatal error
                            }

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
    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(self.run_async(upstream, feedback));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
