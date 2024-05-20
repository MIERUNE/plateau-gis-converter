//! Minecraft sink

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use hashbrown::HashMap;
use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};

use nusamai_plateau::Entity;
use rayon::prelude::*;

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
};

pub struct MinecraftSinkProvider {}

impl DataSinkProvider for MinecraftSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "minecraft".to_string(),
            name: "Minecraft".to_string(),
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

        Box::<MinecraftSink>::new(MinecraftSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct MinecraftSink {
    output_path: PathBuf,
}

impl DataSink for MinecraftSink {
    fn make_requirements(&self) -> DataRequirements {
        DataRequirements {
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        let (ra, rb) = rayon::join(
            || {
                upstream
                    .into_iter()
                    .par_bridge()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        // 地心座標系に変更
                        // Y軸とZ軸を入れ替え
                        // 地物のボクセライズ
                        // カラーの割り当て
                        // 地物ごとのvoxelを次のステージに送信

                        if sender.send(parcel.entity).is_err() {
                            return Err(PipelineError::Canceled);
                        };

                        Ok(())
                    })
            },
            || {
                let mut voxels = Vec::new();

                receiver.into_iter().for_each(|feature| {
                    // voxelを受け取る
                    // どのリージョン・チャンク・セクション・座標なのかを特定する
                    // ファイル出力に都合の良い形式に変換
                    voxels.push(feature);
                });
出力
                // ファイル群を出力
                std::fs::create_dir_all(&self.output_path)?;
                let _ = voxels.iter().try_for_each(|features| -> Result<()> {
                    feedback.ensure_not_canceled()?;

                    Ok(())
                });

                Ok(())
            },
        );

        match ra {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.fatal_error(error),
        }
        match rb {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.fatal_error(error),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
