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
    types::{Element, Geometry, Kml, MultiGeometry, Placemark, Polygon as KmlPolygon, SimpleData},
    KmlWriter,
};

pub struct KmlSinkProvider {}

impl DataSinkProvider for KmlSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "kml".to_string(),
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
                // Convert CityObjects to KML objects
                upstream
                    .into_iter()
                    .par_bridge()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let polygons = entity_to_kml_polygons(&parcel.entity);

                        let _simple_data_items = property_to_schema_data_entries(&entity_to_properties(&parcel.entity).unwrap_or_default());
                        let _schema_data = Element {
                            name: "SchemaData".to_string(),
                            // attrs: {
                            //     let mut attrs = HashMap::new();
                            //     attrs.insert("schemaUrl".to_string(), "#Schema_1".to_string());
                            //     attrs
                            // },
                            attrs: HashMap::new(),
                            content: None,
                            children: _simple_data_items
                                .into_iter()
                                .map(|simple_data| Element {
                                    name: "SimpleData".to_string(),
                                    attrs: simple_data.attrs,
                                    content: Some(simple_data.value),
                                    children: Vec::new(),
                                })
                                .collect::<Vec<_>>(),
                        };
                        let extended_data_entry = Element {
                            name: "ExtendedData".to_string(),
                            attrs: HashMap::new(),
                            content: None,
                            children: vec![_schema_data],
                        };

                        let geoms = polygons.into_iter().map(Geometry::Polygon).collect();
                        let multi_geom = MultiGeometry {
                            geometries: geoms,
                            ..Default::default()
                        };

                        let placemark = Placemark {
                            geometry: Some(Geometry::MultiGeometry(multi_geom)),
                            children: vec![extended_data_entry],
                            name: None,
                            description: None,
                            attrs: HashMap::new(),
                        };

                        if sender.send(placemark).is_err() {
                            return Err(PipelineError::Canceled);
                        }
                        Ok(())
                    })
            },
            || {
                // TODO?:QGIS attribute
                let schema_element = Element {
                    name: "Schema".to_string(),
                    attrs: {
                        let mut attrs = HashMap::new();
                        attrs.insert("name".to_string(), "Schema_1".to_string());
                        attrs.insert("id".to_string(), "Schema_1".to_string());
                        attrs
                    },
                    content: None,
                    children: Vec::new(),
                    // children: {
                    //     let mut children = Vec::new();
                    //     children.push(Element {
                    //         name: "SimpleField".to_string(),
                    //         attrs: {
                    //             let mut attrs = HashMap::new();
                    //             attrs.insert("name".to_string(), "id".to_string());
                    //             attrs.insert("type".to_string(), "string".to_string());
                    //             attrs
                    //         },
                    //         content: None,
                    //         children: Vec::new(),
                    //     });
                    //     children.push(Element {
                    //         name: "SimpleField".to_string(),
                    //         attrs: {
                    //             let mut attrs = HashMap::new();
                    //             attrs.insert("name".to_string(), "type".to_string());
                    //             attrs.insert("type".to_string(), "string".to_string());
                    //             attrs
                    //         },
                    //         content: None,
                    //         children: Vec::new(),
                    //     });
                    //     children
                    // },
                };

                // let placemarks = receiver.into_iter();
                let placemarks = receiver.into_iter().collect::<Vec<_>>();

                let kml_doc = Kml::Document {
                    attrs: HashMap::new(),
                    elements: {
                        let mut elements = Vec::<Kml>::new();
                        elements.push(Kml::Element(schema_element));
                        elements.extend(placemarks.into_iter().map(Kml::Placemark));
                        elements
                    },
                };

                let mut file = File::create(&self.output_path).unwrap();
                let mut buf_writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                writeln!(buf_writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
                writeln!(
                    buf_writer,
                    r#"<kml xmlns="http://www.opengis.net/kml/2.2">"#
                )
                .unwrap();
                // let mut kml_writer = KmlWriter::from_writer(&mut buf_writer);
                // let _ = kml_writer.write(&folder);

                let mut kml_writer = KmlWriter::from_writer(&mut buf_writer);
                let _ = kml_writer.write(&kml_doc);
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
        // _ => panic!("Root value type must be Feature, but found {:?}", value),
        _ => panic!("{}", geojson::JsonObject::new())
    }
}

pub fn entity_to_properties(entity: &Entity) -> Option<geojson::JsonObject> {
    extract_properties(&entity.root)
}

pub fn property_to_schema_data_entries(properties: &geojson::JsonObject) -> Vec<SimpleData> {
    let mut simple_data_entries = Vec::new();

    for (key, value) in properties.iter() {
        let simpledata = SimpleData {
            name: key.to_string(),
            value: serde_json::to_string(value).unwrap(),
            attrs: {
                let mut attrs = HashMap::new();
                attrs.insert("name".to_string(), key.to_string());
                attrs
            },
        };
        simple_data_entries.push(simpledata);
    }
    simple_data_entries
}

pub fn entity_to_kml_polygons(entity: &Entity) -> Vec<KmlPolygon> {
    let geom_store = entity.geometry_store.read().unwrap();

    let Value::Object(obj) = &entity.root else {
        return Vec::new();
    };
    let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype else {
        return Vec::new();
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
