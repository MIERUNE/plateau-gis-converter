//! GeoPackage sink

mod bbox;

use std::path::PathBuf;
use url::Url;

use indexmap::IndexMap;

use rayon::prelude::*;

use crate::parameters::Parameters;
use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};
use bbox::Bbox;

use nusamai_citygml::object::{Object, ObjectStereotype, Value};
use nusamai_citygml::schema::{Schema, TypeDef, TypeRef};
use nusamai_citygml::GeometryType;
use nusamai_geometry::MultiPolygon;
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

        // add attribute columns
        let attribute_columns = schema_to_columns(schema);
        handler.add_columns(attribute_columns).await.unwrap();

        // global bounding box, to be updated per entity
        let mut global_bbox: Bbox = Default::default();

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
                        let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype
                        else {
                            return Ok(());
                        };

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

                        // Prepare attributes
                        let attributes = prepare_object_attributes(obj);

                        // Bounding box
                        let bbox = get_indexed_multipolygon_bbox(&geom_store.vertices, &mpoly);

                        if sender.blocking_send((bytes, attributes, bbox)).is_err() {
                            return Err(PipelineError::Canceled);
                        };

                        Ok(())
                    })
            })
        };

        let mut tx = handler.begin().await.unwrap();
        while let Some((gpkg_bin, attributes, bbox)) = receiver.recv().await {
            feedback.ensure_not_canceled()?;
            tx.insert_feature(&gpkg_bin, &attributes).await.unwrap();

            // track the global bounding box values
            global_bbox = global_bbox.merged_with(&bbox);
        }
        tx.commit().await.unwrap();

        handler
            .update_bbox("mpoly3d".into(), global_bbox.to_tuple())
            .await
            .unwrap();

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

/// Prepare the attribute values for the GeoPackage
fn prepare_object_attributes(obj: &Object) -> IndexMap<String, String> {
    let mut attributes = IndexMap::<String, String>::new();

    for (attr_name, attr_value) in &obj.attributes {
        match attr_value {
            Value::String(s) => {
                attributes.insert(attr_name.into(), s.into());
            }
            Value::Code(c) => {
                // value of the code
                attributes.insert(attr_name.into(), c.value().into());
            }
            Value::Integer(i) => {
                attributes.insert(attr_name.into(), i.to_string());
            }
            Value::NonNegativeInteger(i) => {
                attributes.insert(attr_name.into(), i.to_string());
            }
            Value::Double(d) => {
                attributes.insert(attr_name.into(), d.to_string());
            }
            Value::Measure(m) => {
                attributes.insert(attr_name.into(), m.value().to_string());
            }
            Value::Boolean(b) => {
                // 0 for false and 1 for true in SQLite
                attributes.insert(attr_name.into(), if *b { "1".into() } else { "0".into() });
            }
            Value::URI(u) => {
                // value of the URI
                attributes.insert(attr_name.into(), u.value().to_string());
            }
            Value::Date(d) => {
                // Date represented as an ISO8601 string
                attributes.insert(attr_name.into(), d.to_string());
            }
            Value::Point(_p) => {
                // TODO: implement
                // Point struct currently does not contain any data
            }
            Value::Array(_arr) => {
                // TODO: handle multiple values
            }
            Value::Object(_obj) => {
                // TODO: handle nested objects
            }
        };
    }

    attributes
}

/// Check the schema, and prepare attribute column information for the SQLite table
fn schema_to_columns(schema: &Schema) -> IndexMap<String, String> {
    let mut attribute_columns = IndexMap::<String, String>::new();
    schema.types.iter().for_each(|(_name, ty)| match ty {
        TypeDef::Feature(feat_td) => {
            // TODO: consider `feat_td.additional_attributes`
            feat_td.attributes.iter().for_each(|(attr_name, attr)|
                // TODO: consider `attr.{min_occurs,max_occurs}`
                match &attr.type_ref {
                    TypeRef::String => {
                        attribute_columns.insert(attr_name.into(), "TEXT".into());
                    }
                    TypeRef::Code => {
                        attribute_columns.insert(attr_name.into(), "TEXT".into());
                    }
                    TypeRef::Integer | TypeRef::NonNegativeInteger => {
                        attribute_columns.insert(attr_name.into(), "INTEGER".into());
                    }
                    TypeRef::Double => {
                        attribute_columns.insert(attr_name.into(), "REAL".into());
                    }
                    TypeRef::Boolean => {
                        attribute_columns.insert(attr_name.into(), "BOOLEAN".into());
                    }
                    TypeRef::JsonString => {
                        attribute_columns.insert(attr_name.into(), "TEXT".into());
                    }
                    TypeRef::URI => {
                        attribute_columns.insert(attr_name.into(), "TEXT".into());
                    }
                    TypeRef::Date => {
                        attribute_columns.insert(attr_name.into(), "DATE".into());
                    }
                    TypeRef::DateTime => {
                        attribute_columns.insert(attr_name.into(), "TEXT".into());
                    }
                    TypeRef::Measure => {
                        attribute_columns.insert(attr_name.into(), "REAL".into());
                    }
                    TypeRef::Point => {
                        // TODO: implement
                        // Point struct currently does not contain any data
                        log::warn!(
                            "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                            attr.type_ref,
                            attr_name
                        );
                    }
                    TypeRef::Named(_name) => {
                        // TODO: Implement
                        log::warn!(
                            "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                            attr.type_ref,
                            attr_name
                        );
                    },
                    TypeRef::Unknown => {
                        log::warn!(
                            "TypeDef::Feature - Unsupported attribute type: {:?} ('{}')",
                            attr.type_ref,
                            attr_name
                        );
                    }
                });
        }
        TypeDef::Data(data_td) => {
            // TODO: implement
            log::warn!(
                "TypeDef::Data - Not supported yet: {:?}",
                data_td.attributes.values()
            );
        }
        TypeDef::Property(prop_td) => {
            // TODO: implement
            log::warn!(
                "TypeDef::Property - Not supported yet: {} members ({:?}, etc.)",
                prop_td.members.len(),
                prop_td
                    .members
                    .iter()
                    .map(|m| &m.type_ref)
                    .take(3)
                    .collect::<Vec<_>>()
            );
        }
    });

    attribute_columns
}

// Get Bounding box of a MultiPolygon
fn get_indexed_multipolygon_bbox(vertices: &[[f64; 3]], mpoly: &MultiPolygon<1, u32>) -> Bbox {
    let mut bbox: Bbox = Default::default();

    for poly in mpoly {
        for linestring in &poly.exterior() {
            for point_idx in linestring.iter() {
                let [x, y, _z] = vertices[*point_idx as usize];
                bbox = bbox.updated_with(x, y);
            }
        }
    }
    bbox
}

#[cfg(test)]
mod tests {

    use super::*;
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
