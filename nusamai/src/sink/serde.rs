//! Serde sink (demo)
//!
//! This is a demo sink that serializes the city objects with serde.

use std::fs::File;
use std::io::{BufWriter, Write};

use bincode;
use rayon::prelude::*;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

pub struct SerdeSinkProvider {}

impl DataSinkProvider for SerdeSinkProvider {
    fn create(&self, _config: &Config) -> Box<dyn DataSink> {
        Box::<SerdeSink>::default()
    }

    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "Serde Sink".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

#[derive(Default)]
pub struct SerdeSink {
    features_written: usize,
    bytes_written: usize,
}

impl DataSink for SerdeSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(100);

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
                        bincode::serialize_into(buf as &mut Vec<u8>, &parcel.cityobj).unwrap();
                        if sender.send(lz4_flex::compress_prepend_size(buf)).is_err() {
                            println!("sink cancelled");
                            return Err(());
                        };
                        Ok(())
                    },
                );
            },
            || {
                // Write to file
                let mut writer = BufWriter::new(File::create("output.bin").unwrap());
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

                println!(
                    "Wrote {} features ({} bytes)",
                    self.features_written, self.bytes_written
                );
            },
        );
    }
}
