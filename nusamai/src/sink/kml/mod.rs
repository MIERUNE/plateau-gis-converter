use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use kml::{
    types::{Element, Geometry, Kml, MultiGeometry, Placemark, Polygon as KmlPolygon, SimpleData},
    KmlWriter,
};
use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};
use nusamai_kml::conversion::indexed_multipolygon_to_kml;
use nusamai_plateau::Entity;
use rayon::prelude::*;

use crate::{
    get_parameter_value,
    option::use_lod_config,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer::TransformerRegistry,
};

use super::option::output_parameter;

pub struct KmlSinkProvider {}

impl DataSinkProvider for KmlSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "kml".to_string(),
            name: "KML".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());
        params
    }

    fn transformer_options(&self) -> TransformerRegistry {
        let mut settings: TransformerRegistry = TransformerRegistry::new();
        settings.insert(use_lod_config("max_lod"));

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_settings = self.transformer_options();

        Box::<KmlSink>::new(KmlSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings,
        })
    }
}

pub struct KmlSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
}

impl DataSink for KmlSink {
    fn make_requirements(&mut self, properties: TransformerRegistry) -> DataRequirements {
        let default_requirements = DataRequirements::default();

        for config in properties.configs.iter() {
            let _ = &self.transform_settings.update_transformer(config.clone());
        }

        self.transform_settings.build(default_requirements)
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

                        let simple_data_items =
                            property_to_schema_data_entries(&parcel.entity.root);

                        let schema_data = Element {
                            name: "SchemaData".to_string(),
                            attrs: {
                                let mut attrs = HashMap::new();
                                attrs.insert("schemaUrl".to_string(), "#Schema_1".to_string());
                                attrs
                            },
                            content: None,
                            children: simple_data_items
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
                            children: vec![schema_data],
                        };

                        let geoms = polygons.into_iter().map(Geometry::Polygon).collect();
                        let multi_geom = MultiGeometry {
                            geometries: geoms,
                            ..Default::default()
                        };

                        let placemark = Placemark {
                            geometry: Some(Geometry::MultiGeometry(multi_geom)),
                            children: vec![extended_data_entry],
                            ..Default::default()
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

                let mut file = File::create(&self.output_path)?;
                let mut buf_writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                writeln!(buf_writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
                writeln!(
                    buf_writer,
                    r#"<kml xmlns="http://www.opengis.net/kml/2.2">"#
                )?;
                writeln!(buf_writer, r#"<Document>"#)?;

                let mut kml_writer = KmlWriter::from_writer(&mut buf_writer);

                kml_writer
                    .write(&Kml::<f64>::Element(schema_element))
                    .map_err(|err| match err {
                        kml::Error::IoError(err) => PipelineError::IoError(err),
                        err => PipelineError::Other(err.to_string()),
                    })?;

                for placemark in receiver {
                    kml_writer
                        .write(&Kml::<f64>::Placemark(placemark))
                        .map_err(|err| match err {
                            kml::Error::IoError(err) => PipelineError::IoError(err),
                            err => PipelineError::Other(err.to_string()),
                        })?;
                }

                writeln!(buf_writer, "</Document>")?;
                writeln!(buf_writer, "</kml>")?;

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

pub fn property_to_schema_data_entries(root: &Value) -> Vec<SimpleData> {
    let Value::Object(obj) = root else {
        return vec![];
    };

    let simple_data_entries: Vec<_> = obj
        .attributes
        .iter()
        .map(|(key, value)| SimpleData {
            name: key.to_string(),
            value: match value {
                Value::String(s) => s.to_string(),
                Value::Double(d) => d.to_string(),
                Value::Measure(m) => m.value().to_string(),
                Value::Boolean(b) => b.to_string(),
                Value::Code(c) => c.value().to_string(),
                Value::Integer(i) => i.to_string(),
                Value::NonNegativeInteger(u) => u.to_string(),
                Value::Uri(u) => u.value().to_string(),
                Value::Date(d) => d.to_string(),
                Value::Point(_) => "".to_string(),
                Value::Array(_) => unreachable!(),
                Value::Object(_) => unreachable!(),
            },
            attrs: {
                let mut attrs = HashMap::new();
                attrs.insert("name".to_string(), key.to_string());
                attrs
            },
        })
        .collect();
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

    let mut mpoly = flatgeom::MultiPolygon::<u32>::new();

    geometries.iter().for_each(|entry| match entry.ty {
        GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
            for idx_poly in geom_store
                .multipolygon
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                mpoly.push(&idx_poly);
            }
        }
        GeometryType::Curve => unimplemented!(),
        GeometryType::Point => unimplemented!(),
    });

    indexed_multipolygon_to_kml(&geom_store.vertices, &mpoly)
}
