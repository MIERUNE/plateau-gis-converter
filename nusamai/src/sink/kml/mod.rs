use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};

use nusamai_citygml::object::{ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryType;
use nusamai_kml::conversion::indexed_multipolygon_to_kml;
use nusamai_plateau::Entity;
use rayon::prelude::*;

use kml::{
    types::{Kml, MultiGeometry, Placemark},
    KmlWriter,
};

pub struct KmlSinkProvider {}

impl DataSinkProvider for KmlSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "kml".to_string(),
            name: "KML".to_string(),
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

        Box::<KmlSink>::new(KmlSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct KmlSink {
    output_path: PathBuf,
}

impl DataSink for KmlSink {
    fn make_transform_requirements(&self) -> transformer::Requirements {
        transformer::Requirements {
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        let (ra, rb) = rayon::join(
            || {
                // Convert CityObjects to GeoJSON objects
                upstream
                    .into_iter()
                    .par_bridge()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let multi_geom = entity_to_kml_mutilgeom(&parcel.entity);

                        for geom in multi_geom.geometries {
                            if sender.send(geom).is_err() {
                                return Err(PipelineError::Canceled);
                            }
                        }
                        Ok(())
                    })
            },
            || {
                // Write GeoJSON to a file
                let mut placemarks: Vec<Kml> = Vec::new();

                for geom in receiver.into_iter() {
                    let placemark = Placemark {
                        geometry: Some(geom),
                        ..Default::default()
                    };

                    placemarks.push(Kml::Placemark(placemark));
                }
                // TODO: Handle output file path

                let folder = Kml::Folder {
                    attrs: HashMap::new(),
                    elements: placemarks,
                };

                let mut file = File::create(&self.output_path).unwrap();
                let mut buf_writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                writeln!(buf_writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
                writeln!(
                    buf_writer,
                    r#"<kml xmlns="http://www.opengis.net/kml/2.2">"#
                )
                .unwrap();
                let mut kml_writer = KmlWriter::from_writer(&mut buf_writer);
                let _ = kml_writer.write(&folder);
                writeln!(buf_writer, "</kml>").unwrap();

                Ok(())
            },
        );

        match ra {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.report_fatal_error(error),
        }
        match rb {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.report_fatal_error(error),
        }

        Ok(())
    }
}

fn extract_properties(value: &nusamai_citygml::object::Value) -> Option<geojson::JsonObject> {
    match &value {
        obj @ nusamai_citygml::Value::Object(_) => match obj.to_attribute_json() {
            serde_json::Value::Object(map) => Some(map),
            _ => unreachable!(),
        },
        _ => panic!("Root value type must be Feature, but found {:?}", value),
    }
}

pub fn entity_to_kml_mutilgeom(entity: &Entity) -> MultiGeometry {
    let _properties = extract_properties(&entity.root);
    let geom_store = entity.geometry_store.read().unwrap();

    let Value::Object(obj) = &entity.root else {
        return MultiGeometry::default();
    };
    let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype else {
        return MultiGeometry::default();
    };

    let mut mpoly = nusamai_geometry::MultiPolygon::<1, u32>::new();

    geometries.iter().for_each(|entry| match entry.ty {
        GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
            for idx_poly in geom_store
                .multipolygon
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                mpoly.push(idx_poly);
            }
        }

        GeometryType::Curve => unimplemented!(),
        GeometryType::Point => unimplemented!(),
    });

    indexed_multipolygon_to_kml(&geom_store.vertices, &mpoly)
}
