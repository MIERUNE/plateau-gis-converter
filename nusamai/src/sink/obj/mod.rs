//! Minecraft sink

use std::{fs::File, io::Write, path::PathBuf, sync::Mutex};

use ahash::RandomState;
use earcut::{utils3d::project3d_to_2d, Earcut};
use indexmap::IndexSet;
use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
    transformer::{TransformerOption, TransformerRegistry},
};

use nusamai_projection::cartesian::geodetic_to_geocentric;
use rayon::prelude::*;

pub struct ObjSinkProvider {}

impl DataSinkProvider for ObjSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "obj".to_string(),
            name: "OBJ".to_string(),
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
                label: None,
            },
        );
        params
    }

    fn available_transformer(&self) -> TransformerRegistry {
        let settings: TransformerRegistry = TransformerRegistry::new();

        settings
    }
    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_options = self.available_transformer();

        Box::<ObjSink>::new(ObjSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings: transform_options,
        })
    }
}

pub struct ObjSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
}

impl DataSink for ObjSink {
    fn make_requirements(&mut self, properties: Vec<TransformerOption>) -> DataRequirements {
        let default_requirements = DataRequirements {
            tree_flattening: transformer::TreeFlatteningSpec::Flatten {
                feature: transformer::FeatureFlatteningOption::AllExceptThematicSurfaces,
                data: transformer::DataFlatteningOption::None,
                object: transformer::ObjectFlatteningOption::None,
            },
            ..Default::default()
        };

        for prop in properties {
            let _ = &self
                .transform_settings
                .update_transformer(&prop.key, prop.is_enabled);
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        let (ra, rb) = rayon::join(
            || {
                let ellipsoid = nusamai_projection::ellipsoid::wgs84();

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
                        let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype else {
                            return Ok(());
                        };

                        let mut earcutter = Earcut::new();
                        let mut buf3d: Vec<[f64; 3]> = Vec::new();
                        let mut buf2d: Vec<[f64; 2]> = Vec::new();
                        let mut index_buf: Vec<u32> = Vec::new();
                        let mut triangles = Vec::new();

                        geometries.iter().for_each(|entry| match entry.ty {
                            GeometryType::Solid
                            | GeometryType::Surface
                            | GeometryType::Triangle => {
                                for idx_poly in geom_store.multipolygon.iter_range(
                                    entry.pos as usize..(entry.pos + entry.len) as usize,
                                ) {
                                    let poly = idx_poly.transform(|idx| {
                                        let [lng, lat, height] = geom_store.vertices[*idx as usize];
                                        // Convert to geocentric (x, y, z) coordinate.
                                        // (Earcut do not work in geographic space)
                                        let (x, y, z) =
                                            geodetic_to_geocentric(&ellipsoid, lng, lat, height);
                                        [x, y, z]
                                    });
                                    let num_outer = match poly.hole_indices().first() {
                                        Some(&v) => v as usize,
                                        None => poly.raw_coords().len(),
                                    };

                                    buf3d.clear();
                                    buf3d.extend(poly.raw_coords().iter());

                                    if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                                        // earcut
                                        earcutter.earcut(
                                            buf2d.iter().cloned(),
                                            poly.hole_indices(),
                                            &mut index_buf,
                                        );
                                        triangles.extend(
                                            index_buf.iter().map(|&idx| buf3d[idx as usize]),
                                        );
                                    }
                                }
                            }
                            GeometryType::Curve | GeometryType::Point => {
                                // not supported in PLY sink
                            }
                        });

                        if sender.send(triangles).is_err() {
                            return Err(PipelineError::Canceled);
                        };

                        Ok(())
                    })
            },
            || {
                // calculate the centroid
                let mut mu_x = 0.;
                let mut mu_y = 0.;
                let mut mu_z = 0.;
                let mut all_vertices = Vec::new();
                for (i, triangles) in receiver.into_iter().enumerate() {
                    if i % 10000 == 0 {
                        feedback.ensure_not_canceled()?;
                    }

                    for [x, y, z] in triangles {
                        mu_x += x;
                        mu_y += y;
                        mu_z += z;
                        all_vertices.push([x, y, z]);
                    }
                }
                mu_x /= all_vertices.len() as f64;
                mu_y /= all_vertices.len() as f64;
                mu_z /= all_vertices.len() as f64;

                // make vertices and indices
                let mut vertices: IndexSet<[u64; 3], RandomState> = IndexSet::default();
                let indices: Vec<_> = all_vertices
                    .iter()
                    .map(|[x, y, z]| {
                        let vbits = [
                            (x - mu_x).to_bits(),
                            (y - mu_y).to_bits(),
                            (z - mu_z).to_bits(),
                        ];
                        let (index, _) = vertices.insert_full(vbits);
                        index as u32
                    })
                    .collect();

                feedback.ensure_not_canceled()?;

                // write to file
                println!("{:?} {:?}", vertices.len(), indices.len());
                // let mut file = std::fs::File::create(&self.output_path.clone())?;

                let mut file = File::create("output.obj")?;

                let mut writer = std::io::BufWriter::new(file);

                // Writing vertex data
                for vertex in &vertices {
                    let [vx, vy, vz] = vertex;
                    let vx = f64::from_bits(*vx);
                    let vy = f64::from_bits(*vy);
                    let vz = f64::from_bits(*vz);
                    writeln!(writer, "v {} {} {}", vx, vy, vz)?;
                }

                // Writing of surface data (index starts at 1, so +1 is used)
                for face in indices.chunks(3) {
                    if let [i1, i2, i3] = face {
                        writeln!(writer, "f {} {} {}", i1 + 1, i2 + 1, i3 + 1)?;
                    }
                }

                writer.flush()?;
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
