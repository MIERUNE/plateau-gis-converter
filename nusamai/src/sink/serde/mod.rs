//! Serde sink (demo)
//!
//! This is a demo sink that serializes the city objects with serde.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use bincode;
use nusamai_citygml::schema::Schema;
use rayon::prelude::*;

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

pub struct SerdeSinkProvider {}

impl DataSinkProvider for SerdeSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "Serde (bincode)".to_string(),
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
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);

        Box::<SerdeSink>::new(SerdeSink {
            output_path: output_path.unwrap().into(),
            ..Default::default()
        })
    }
}

#[derive(Default)]
pub struct SerdeSink {
    output_path: PathBuf,
    features_written: usize,
    bytes_written: usize,
}

impl DataSink for SerdeSink {
    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        rayon::join(
            || {
                // Compression
                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    (sender, Vec::new()),
                    |(sender, buf), parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        buf.clear();
                        bincode::serialize_into(buf as &mut Vec<u8>, &parcel.entity).unwrap();
                        if sender.send(lz4_flex::compress_prepend_size(buf)).is_err() {
                            log::info!("sink cancelled");
                            return Err(());
                        };
                        Ok(())
                    },
                );
            },
            || {
                // Write to file
                let mut writer =
                    BufWriter::with_capacity(1024 * 1024, File::create(&self.output_path).unwrap());
                for compressed in receiver {
                    // size
                    writer
                        .write_all(&(compressed.len() as u32).to_le_bytes())
                        .unwrap();
                    // compressed data
                    writer.write_all(&compressed).unwrap();

                    self.features_written += 1;
                    self.bytes_written += 4 + compressed.len();
                }

                log::info!(
                    "Wrote {} features ({} bytes)",
                    self.features_written,
                    self.bytes_written
                );
            },
        );
    }
}
