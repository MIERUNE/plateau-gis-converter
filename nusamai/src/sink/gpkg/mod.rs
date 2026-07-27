//! GeoPackage sink

mod attributes;
mod bbox;
mod geometry;
mod table;

use std::{collections::HashMap, path::PathBuf};

use attributes::prepare_object_attributes;
use bbox::Bbox;
use geometry::encode_feature_geometry;
use indexmap::IndexMap;
use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
};
use nusamai_gpkg::{table::GpkgGeometryType, GpkgHandler};
use rayon::prelude::*;
use table::schema_to_table_infos;
use url::Url;

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
    transformer::{join_attribute_arrays_config, use_lod_config, TransformerSettings},
};

use super::option::output_parameter;

pub struct GpkgSinkProvider {}

impl DataSinkProvider for GpkgSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "gpkg".to_string(),
            name: "GeoPackage".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());

        params
    }

    fn transformer_options(&self) -> TransformerSettings {
        let mut settings: TransformerSettings = TransformerSettings::new();
        settings.insert(use_lod_config("max_lod", None));
        settings.insert(join_attribute_arrays_config(false));

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_settings = self.transformer_options();

        Box::<GpkgSink>::new(GpkgSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings,
        })
    }
}

pub struct GpkgSink {
    output_path: PathBuf,
    transform_settings: TransformerSettings,
}

// An ephimeral container to wrap and pass the data in the pipeline
// Corresponds to a record in the features/attributes table of GeoPackage
enum Record {
    Feature {
        obj_id: String,
        geometry_type: GpkgGeometryType,
        geometry: Vec<u8>,
        bbox: Bbox,
        attributes: IndexMap<String, String>,
    },
    Attribute {
        attributes: IndexMap<String, String>,
    },
}

impl GpkgSink {
    pub async fn run_async(
        &mut self,
        upstream: Receiver,
        feedback: &Feedback,
        schema: &Schema,
    ) -> Result<()> {
        let mut handler = if self.output_path.to_string_lossy().starts_with("sqlite:") {
            // note: unlike the case of the file system path, the database is not cleared even if it already exists
            // this is mainly expected to be used with `sqlite::memory:` for the testing purpose
            GpkgHandler::from_url(&Url::parse(self.output_path.to_str().unwrap()).unwrap())
                .await
                .map_err(|e| PipelineError::Other(e.to_string()))?
        } else {
            // delete the db file first if already exists
            if self.output_path.exists() {
                std::fs::remove_file(&self.output_path)?;
            };

            let conn_str = format!("file:{}", self.output_path.to_string_lossy());
            GpkgHandler::from_str(&conn_str)
                .await
                .map_err(|e| PipelineError::Other(e.to_string()))?
        };

        let table_infos = schema_to_table_infos(schema);
        let mut created_tables = HashMap::<String, Option<GpkgGeometryType>>::new();
        let has_feature_tables = table_infos.values().any(|table| table.has_geometry);
        let srs_id = match schema.epsg {
            Some(srs_id) => srs_id,
            None if !has_feature_tables => 0, // Undefined Geographic for attribute-only output
            None => {
                return Err(PipelineError::Other(
                    "GeoPackage feature output requires a schema EPSG code".into(),
                ));
            }
        };

        let mut table_bboxes = IndexMap::<String, Bbox>::new();

        let (sender, mut receiver) = tokio::sync::mpsc::channel(100);

        let producers = {
            let feedback = feedback.clone();
            tokio::task::spawn_blocking(move || {
                upstream
                    .into_iter()
                    .par_bridge()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let entity = parcel.entity;
                        let geom_store = entity.geometry_store.read().unwrap();

                        let Value::Object(obj) = &entity.root else {
                            return Ok(());
                        };

                        match &obj.stereotype {
                            ObjectStereotype::Feature {
                                id: obj_id,
                                geometries,
                            } => {
                                let table_name = obj.typename.to_string();
                                let Some(encoded) = encode_feature_geometry(
                                    &geom_store,
                                    geometries,
                                    srs_id,
                                    &table_name,
                                    obj_id,
                                )?
                                else {
                                    return Ok(());
                                };

                                let record = Record::Feature {
                                    obj_id: obj_id.clone(),
                                    geometry_type: encoded.geometry_type,
                                    geometry: encoded.bytes,
                                    bbox: encoded.bbox,
                                    attributes: prepare_object_attributes(obj),
                                };
                                if sender.blocking_send((table_name, record)).is_err() {
                                    return Err(PipelineError::Canceled);
                                };
                            }
                            ObjectStereotype::Data => {
                                let table_name = obj.typename.to_string();
                                let record = Record::Attribute {
                                    attributes: prepare_object_attributes(obj),
                                };
                                if sender.blocking_send((table_name, record)).is_err() {
                                    return Err(PipelineError::Canceled);
                                };
                            }
                            ObjectStereotype::Object { id: obj_id } => {
                                // TODO: implement (you will also need the corresponding TypeDef::Object in the schema)
                                feedback.warn(format!(
                                    "ObjectStereotype::Object is not supported yet: id = {obj_id}"
                                ));
                            }
                        }

                        Ok(())
                    })
            })
        };

        let mut tx = handler
            .begin()
            .await
            .map_err(|e| PipelineError::Other(e.to_string()))?;
        while let Some((table_name, record)) = receiver.recv().await {
            feedback.ensure_not_canceled()?;

            let record_geometry_type = match &record {
                Record::Feature { geometry_type, .. } => Some(*geometry_type),
                Record::Attribute { .. } => None,
            };
            if let Some(created_geometry_type) = created_tables.get(&table_name) {
                if *created_geometry_type != record_geometry_type {
                    return Err(PipelineError::Other(format!(
                        "mixed geometry types are not supported in table {table_name}: first record uses {created_geometry_type:?}, current record uses {record_geometry_type:?}"
                    )));
                }
            } else {
                let table_info = table_infos.get(&table_name).ok_or_else(|| {
                    PipelineError::Other(format!(
                        "GeoPackage table information is missing for {table_name}"
                    ))
                })?;
                tx.add_table(table_info, srs_id, record_geometry_type)
                    .await
                    .map_err(|e| PipelineError::Other(e.to_string()))?;
                created_tables.insert(table_name.clone(), record_geometry_type);
            }

            match record {
                Record::Feature {
                    obj_id,
                    geometry_type: _,
                    geometry,
                    bbox,
                    attributes,
                } => {
                    tx.insert_feature(&table_name, &obj_id, &geometry, &attributes)
                        .await
                        .map_err(|e| PipelineError::Other(e.to_string()))?;
                    table_bboxes.entry(table_name).or_default().merge(&bbox);
                }
                Record::Attribute { attributes } => {
                    tx.insert_attribute(&table_name, &attributes)
                        .await
                        .map_err(|e| PipelineError::Other(e.to_string()))?;
                }
            }
        }

        producers.await.map_err(|error| {
            PipelineError::Other(format!("GeoPackage producer task failed: {error}"))
        })??;

        for (table_name, bbox) in table_bboxes {
            feedback.ensure_not_canceled()?;

            tx.update_bbox(&table_name, bbox.to_tuple())
                .await
                .map_err(|e| PipelineError::Other(e.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|e| PipelineError::Other(e.to_string()))?;

        // Switch from WAL to DELETE journal mode so that the output is a single
        // self-contained .gpkg file without -wal/-shm sidecars.
        handler
            .finalize()
            .await
            .map_err(|e| PipelineError::Other(e.to_string()))?;
        Ok(())
    }
}

pub enum GpkgTransformOption {}

impl DataSink for GpkgSink {
    fn make_requirements(&mut self, properties: TransformerSettings) -> DataRequirements {
        let default_requirements = DataRequirements {
            tree_flattening: transformer::TreeFlatteningSpec::Flatten {
                feature: transformer::FeatureFlatteningOption::AllExceptThematicSurfaces,
                data: transformer::DataFlatteningOption::TopLevelOnly,
                object: transformer::ObjectFlatteningOption::None,
            },
            ..Default::default()
        };

        for config in properties.configs.iter() {
            let _ = &self.transform_settings.update_transformer(config.clone());
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(self.run_async(upstream, feedback, schema))
    }
}
