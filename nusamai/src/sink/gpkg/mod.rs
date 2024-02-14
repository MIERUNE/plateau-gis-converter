//! GeoPackage sink

mod attributes;
mod bbox;
mod table;

use std::collections::HashSet;
use std::path::PathBuf;
use url::Url;

use indexmap::IndexMap;

use rayon::prelude::*;

use crate::parameters::Parameters;
use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};
use attributes::prepare_object_attributes;
use bbox::{get_indexed_multipolygon_bbox, Bbox};
use table::schema_to_table_infos;

use nusamai_citygml::object::{ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryType;
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
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);

        Box::<GpkgSink>::new(GpkgSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct GpkgSink {
    output_path: PathBuf,
}

impl GpkgSink {
    pub async fn run_async(
        &mut self,
        upstream: Receiver,
        feedback: &Feedback,
        schema: &Schema,
    ) -> Result<()> {
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

        let table_infos = schema_to_table_infos(schema);
        let mut created_tables = HashSet::<String>::new();

        let mut table_bboxes = IndexMap::<String, Bbox>::new();
        for table_name in table_infos.keys() {
            table_bboxes.insert(table_name.clone(), Default::default());
        }

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
                                let mut mpoly = nusamai_geometry::MultiPolygon::new();

                                geometries.iter().for_each(|entry| match entry.ty {
                                    GeometryType::Solid
                                    | GeometryType::Surface
                                    | GeometryType::Triangle => {
                                        for idx_poly in geom_store.multipolygon.iter_range(
                                            entry.pos as usize..(entry.pos + entry.len) as usize,
                                        ) {
                                            mpoly.push(idx_poly);
                                        }
                                    }
                                    GeometryType::Curve => unimplemented!(),
                                    GeometryType::Point => unimplemented!(),
                                });

                                if mpoly.is_empty() {
                                    return Ok(());
                                }

                                let mut bytes = Vec::new();
                                if write_indexed_multipolygon(
                                    &mut bytes,
                                    &geom_store.vertices,
                                    &mpoly,
                                    4326,
                                )
                                .is_err()
                                {
                                    // TODO: fatal error
                                }

                                let attributes = prepare_object_attributes(obj);

                                let bbox =
                                    get_indexed_multipolygon_bbox(&geom_store.vertices, &mpoly);

                                let table_name = obj.typename.to_string();

                                if sender
                                    .blocking_send((
                                        table_name,
                                        obj_id.clone(),
                                        bytes,
                                        attributes,
                                        bbox,
                                    ))
                                    .is_err()
                                {
                                    return Err(PipelineError::Canceled);
                                };
                            }
                            ObjectStereotype::Object { id: _obj_id } => {
                                // TODO: implement
                            }
                            ObjectStereotype::Data => {
                                // TODO: implement
                            }
                        }

                        Ok(())
                    })
            })
        };

        let mut tx: nusamai_gpkg::GpkgTransaction<'_> = handler.begin().await.unwrap();
        while let Some((table_name, obj_id, gpkg_bin, attributes, bbox)) = receiver.recv().await {
            feedback.ensure_not_canceled()?;

            if !created_tables.contains(&table_name) {
                let tf = table_infos.get(&table_name).unwrap();
                tx.add_table(tf).await.unwrap();
                created_tables.insert(table_name.clone());
            }

            tx.insert_feature(&table_name, &obj_id, &gpkg_bin, &attributes)
                .await
                .unwrap();

            // update the bounding box values
            table_bboxes.get_mut(&table_name).unwrap().merge(&bbox);
        }

        for (table_name, _bbox) in &table_bboxes {
            tx.update_bbox(table_name, table_bboxes.get(table_name).unwrap().to_tuple())
                .await
                .unwrap();
        }

        tx.commit().await.unwrap();

        match producers.await.unwrap() {
            Ok(_) | Err(PipelineError::Canceled) => Ok(()),
            error @ Err(_) => error,
        }
    }
}

impl DataSink for GpkgSink {
    fn make_transform_requirements(&self) -> transformer::Requirements {
        transformer::Requirements {
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(self.run_async(upstream, feedback, schema))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nusamai_geometry::MultiPolygon;
    use nusamai_projection::crs::EPSG_JGD2011_GEOGRAPHIC_3D;

    #[test]
    fn test_get_indexed_multipolygon_bbox() {
        let vertices: Vec<[f64; 3]> = vec![
            [10., 100., 111.],
            [10., 200., 111.],
            [20., 200., 111.],
            [20., 100., 111.],
        ];
        let mut mpoly = MultiPolygon::<1, u32>::new();
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        let geometries = nusamai_citygml::GeometryStore {
            epsg: EPSG_JGD2011_GEOGRAPHIC_3D,
            vertices,
            multipolygon: mpoly,
            ..Default::default()
        };

        let bbox = get_indexed_multipolygon_bbox(&geometries.vertices, &geometries.multipolygon);

        assert_eq!(bbox.to_tuple(), (10., 100., 20., 200.));
    }
}
