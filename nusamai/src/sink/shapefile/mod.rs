//! Shapefile sink

use std::hash::Hash;
use std::path::PathBuf;
use std::sync::Mutex;

use chrono::Datelike;
use hashbrown::HashMap;
use indexmap::IndexMap;
use quick_xml::events::attributes;
use rayon::prelude::*;
use shapefile::dbase::{self, Date, FieldName, FieldValue, Record};

use nusamai_citygml::object::{self, ObjectStereotype, Value};
use nusamai_citygml::schema::{Schema, TypeDef, TypeRef};
use nusamai_citygml::GeometryType;
use nusamai_plateau::Entity;
use nusamai_shapefile::conversion::indexed_multipolygon_to_shape;

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo};
use crate::transformer;

pub struct ShapefileSinkProvider {}

impl DataSinkProvider for ShapefileSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "shapefile".to_string(),
            name: "Shapefile".to_string(),
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

        Box::<ShapefileSink>::new(ShapefileSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct ShapefileSink {
    output_path: PathBuf,
}

impl DataSink for ShapefileSink {
    fn make_requirements(&self) -> DataRequirements {
        DataRequirements {
            shorten_names_for_shapefile: true,
            tree_flattening: transformer::TreeFlatteningSpec::Flatten {
                feature: transformer::FeatureFlatteningOption::AllExceptThematicSurfaces,
                data: transformer::DataFlatteningOption::None,
                object: transformer::ObjectFlatteningOption::None,
            },
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        let (ra, rb) = rayon::join(
            || {
                // Convert CityObjects to Shapefile objects

                upstream
                    .into_iter()
                    .par_bridge()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let Value::Object(object) = &parcel.entity.root else {
                            return Ok(());
                        };
                        let typename = object.typename.clone();

                        let (shape, attributes) = entity_to_shapes(&parcel.entity);

                        if sender.send((typename, shape, attributes)).is_err() {
                            return Err(PipelineError::Canceled);
                        };

                        Ok(())
                    })
            },
            || {
                // Write Shapefile to a file

                // Attribute fields for the features
                // FieldName byte representation cannot exceed 11 bytes
                let mut categorized_shapes =
                    IndexMap::<String, Vec<(shapefile::Shape, dbase::Record)>>::new();

                receiver
                    .into_iter()
                    .for_each(|(typename, shape, attributes)| {
                        categorized_shapes
                            .entry(typename.to_string())
                            .or_default()
                            .push((shape, attributes));
                    });

                for (typename, features) in categorized_shapes {
                    let table_builder = prepare_table_builder(&typename, schema, &features);

                    // Create all the files needed for the shapefile to be complete (.shp, .shx, .dbf)
                    std::fs::create_dir_all(&self.output_path)?;
                    let mut file_path = self.output_path.clone();
                    file_path.push(format!("{}.shp", typename));

                    let mut writer = shapefile::Writer::from_path(file_path, table_builder)?;

                    // Write each feature
                    features
                        .into_iter()
                        .for_each(|(shape, attributes)| match shape {
                            shapefile::Shape::PolygonZ(polygon) => {
                                writer
                                    .write_shape_and_record(&polygon, &attributes)
                                    .unwrap();
                            }
                            shapefile::Shape::NullShape => {}
                            _ => {
                                log::warn!("Unsupported shape type");
                            }
                        });
                }

                Ok::<(), shapefile::Error>(())
            },
        );

        match ra {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.report_fatal_error(error),
        }
        match rb {
            Ok(_) => {}
            Err(shapefile::Error::IoError(error)) => {
                feedback.report_fatal_error(PipelineError::IoError(error))
            }
            Err(error) => feedback.report_fatal_error(PipelineError::Other(error.to_string())),
        }

        Ok(())
    }
}

pub struct FieldInfo {
    field_type: TypeRef,
    size: u8,
}

pub type FieldInfoList = HashMap<String, FieldInfo>;

struct TableBuilder {
    fields: FieldInfoList,
    builder: dbase::TableWriterBuilder,
}

impl<'a> TableBuilder {
    pub fn new(fields: FieldInfoList) -> Self {
        Self {
            fields,
            builder: dbase::TableWriterBuilder::new(),
        }
    }

    pub fn add_field(mut self, field_type: &TypeRef, name: &str, size: u8) -> Self {
        match field_type {
            TypeRef::String | TypeRef::Code | TypeRef::URI => {
                self.builder = self
                    .builder
                    .add_character_field(name.try_into().unwrap(), size);
            }
            TypeRef::Integer | TypeRef::NonNegativeInteger => {
                self.builder = self.builder.add_integer_field(name.try_into().unwrap());
            }
            TypeRef::Double | TypeRef::Measure => {
                self.builder = self.builder.add_double_field(name.try_into().unwrap());
            }
            TypeRef::Boolean => {
                self.builder = self.builder.add_logical_field(name.try_into().unwrap());
            }
            TypeRef::Date => {
                self.builder = self.builder.add_date_field(name.try_into().unwrap());
            }
            TypeRef::Point => {
                // todo
            }
            TypeRef::Unknown => {
                // todo
            }
            TypeRef::Named(_) => {
                // todo
            }
            TypeRef::JsonString(_) => {
                // todo
            }
            TypeRef::DateTime => {
                // todo
            }
        }
        self
    }

    fn build(mut self) -> dbase::TableWriterBuilder {
        for (field_name, field_info) in self.fields {
            match field_info.field_type {
                TypeRef::String | TypeRef::Code | TypeRef::URI => {
                    self.builder = self.builder.add_character_field(
                        field_name.as_str().try_into().unwrap(),
                        field_info.size,
                    );
                }
                TypeRef::Integer | TypeRef::NonNegativeInteger => {
                    self.builder = self
                        .builder
                        .add_integer_field(field_name.as_str().try_into().unwrap());
                }
                TypeRef::Double | TypeRef::Measure => {
                    self.builder = self
                        .builder
                        .add_double_field(field_name.as_str().try_into().unwrap());
                }
                TypeRef::Boolean => {
                    self.builder = self
                        .builder
                        .add_logical_field(field_name.as_str().try_into().unwrap());
                }
                TypeRef::Date => {
                    self.builder = self
                        .builder
                        .add_date_field(field_name.as_str().try_into().unwrap());
                }
                TypeRef::Point => {
                    // todo
                }
                TypeRef::Unknown => {
                    // todo
                }
                TypeRef::Named(_) => {
                    // todo
                }
                TypeRef::JsonString(_) => {
                    // todo
                }
                TypeRef::DateTime => {
                    // todo
                }
            }
        }
        self.builder
    }
}

fn prepare_table_builder(
    schema: &Schema,
    features: &Vec<(shapefile::Shape, dbase::Record)>,
) -> dbase::TableWriterBuilder {
    // let table_builder = TableWriterBuilderWrapper::new(schema);
    let mut fields: FieldInfoList = Default::default();

    for (_, attributes) in features {
        for (field_name, field_value) in attributes.clone().into_iter() {
            match field_value {
                FieldValue::Character(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::String,
                            size: 255,
                        },
                    );
                }
                FieldValue::Numeric(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                FieldValue::Logical(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Boolean,
                            size: 1,
                        },
                    );
                }
                FieldValue::Date(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Date,
                            size: 8,
                        },
                    );
                }
                FieldValue::Float(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                FieldValue::Integer(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Integer,
                            size: 4,
                        },
                    );
                }
                FieldValue::Currency(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                FieldValue::DateTime(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::DateTime,
                            size: 8,
                        },
                    );
                }
                FieldValue::Double(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::Double,
                            size: 8,
                        },
                    );
                }
                FieldValue::Memo(_) => {
                    fields.insert(
                        field_name.clone(),
                        FieldInfo {
                            field_type: TypeRef::String,
                            size: 255,
                        },
                    );
                }
            }
        }
    }

    let table_builder = TableBuilder::new(fields);
    table_builder.build()
}

fn prepare_shapefile_attributes(
    object: &nusamai_citygml::object::Object,
) -> IndexMap<String, FieldValue> {
    let mut attributes = IndexMap::<String, FieldValue>::new();

    // todo: 長いフィールドはどうなるのか。落ちる？勝手に切られる？
    for (attr_name, attr_value) in &object.attributes {
        match attr_value {
            Value::String(s) => {
                attributes.insert(attr_name.into(), FieldValue::Character(Some(s.to_owned())));
            }
            Value::Code(c) => {
                // value of the code
                attributes.insert(
                    attr_name.into(),
                    FieldValue::Character(Some(c.value().to_owned())),
                );
            }
            Value::Integer(i) => {
                attributes.insert(attr_name.into(), FieldValue::Integer(i.to_owned() as i32));
            }
            Value::NonNegativeInteger(i) => {
                attributes.insert(attr_name.into(), FieldValue::Integer(i.to_owned() as i32));
            }
            Value::Double(d) => {
                attributes.insert(attr_name.into(), FieldValue::Double(d.to_owned()));
            }
            Value::Measure(m) => {
                attributes.insert(attr_name.into(), FieldValue::Double(m.value().to_owned()));
            }
            Value::Boolean(b) => {
                attributes.insert(attr_name.into(), FieldValue::Logical(Some(b.to_owned())));
            }
            Value::Uri(u) => {
                attributes.insert(
                    attr_name.into(),
                    FieldValue::Character(Some(u.value().to_string())),
                );
            }
            Value::Date(d) => {
                // Date represented as an ISO8601 string
                attributes.insert(
                    attr_name.into(),
                    FieldValue::Date(Some(Date::new(d.day(), d.month(), d.year() as u32))),
                );
            }
            Value::Point(_p) => {
                // TODO: implement
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

/// Create Shapefile features from a Entity
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
/// TODO: Implement MultiLineString and MultiPoint handling
pub fn entity_to_shapes(entity: &Entity) -> (shapefile::Shape, dbase::Record) {
    let mut record = dbase::Record::default();

    let Value::Object(obj) = &entity.root else {
        return (shapefile::Shape::NullShape, record);
    };
    let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype else {
        return (shapefile::Shape::NullShape, record);
    };

    let geom_store = entity.geometry_store.read().unwrap();

    let mut mpoly = nusamai_geometry::MultiPolygon::<1, u32>::new();

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

    if !mpoly.is_empty() {
        let shape =
            shapefile::Shape::PolygonZ(indexed_multipolygon_to_shape(&geom_store.vertices, &mpoly));

        let attributes = prepare_shapefile_attributes(obj);
        for (field_name, field_value) in attributes {
            record.insert(field_name, field_value);
        }
        return (shape, record);
    }

    (shapefile::Shape::NullShape, record)
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use nusamai_citygml::{object::Object, GeometryRef};
    use nusamai_geometry::MultiPolygon;
    use nusamai_projection::crs::EPSG_JGD2011_GEOGRAPHIC_3D;
    use shapefile::NO_DATA;

    #[test]
    fn test_toplevel_cityobj_multipolygon() {
        let vertices: Vec<[f64; 3]> = vec![
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
        ];
        let mut mpoly = MultiPolygon::<1, u32>::new();
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        let geometries = nusamai_citygml::GeometryStore {
            epsg: EPSG_JGD2011_GEOGRAPHIC_3D,
            vertices,
            multipolygon: mpoly,
            ..Default::default()
        };

        let obj = Entity {
            root: Value::Object(Object {
                typename: "dummy".into(),
                attributes: Default::default(),
                stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                    id: "dummy".into(),
                    geometries: vec![GeometryRef {
                        ty: GeometryType::Solid,
                        pos: 0,
                        len: 1,
                        lod: 1,
                    }],
                },
            }),
            base_url: url::Url::parse("file:///dummy").unwrap(),
            geometry_store: RwLock::new(geometries).into(),
            appearance_store: Default::default(),
        };

        let shapes = entity_to_shapes(&obj);
        assert_eq!(shapes.len(), 1);

        if let shapefile::Shape::PolygonZ(polygon) = &shapes[0] {
            assert_eq!(polygon.rings().len(), 1);
            assert_eq!(
                polygon.ring(0).unwrap(),
                &shapefile::PolygonRing::Outer(vec![
                    // Outer ring: re-ordered to clockwise
                    shapefile::PointZ::new(0., 0., 111., NO_DATA),
                    shapefile::PointZ::new(0., 5., 111., NO_DATA),
                    shapefile::PointZ::new(5., 5., 111., NO_DATA),
                    shapefile::PointZ::new(5., 0., 111., NO_DATA),
                    shapefile::PointZ::new(0., 0., 111., NO_DATA), // automatically closed
                ])
            )
        } else {
            panic!("Expected PolygonZ.");
        }
    }
}
