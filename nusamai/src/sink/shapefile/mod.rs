//! Shapefile sink

mod attributes;

use std::path::PathBuf;

use indexmap::IndexMap;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use nusamai_citygml::object::{Map, ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryType;
use nusamai_plateau::Entity;
use nusamai_shapefile::conversion::indexed_multipolygon_to_shape;

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo};
use crate::transformer;

use self::attributes::{
    fill_missing_fields, make_field_list, prepare_shapefile_attributes, TableBuilder,
};

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
                data: transformer::DataFlatteningOption::TopLevelOnly,
                object: transformer::ObjectFlatteningOption::None,
            },

            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
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

                        let (shape, attributes) = entity_to_shapes(&parcel.entity);

                        if sender
                            .send((object.typename.clone(), shape, attributes))
                            .is_err()
                        {
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
                    IndexMap::<String, Vec<(shapefile::Shape, Map)>>::new();

                receiver
                    .into_iter()
                    .for_each(|(typename, shape, attributes)| {
                        categorized_shapes
                            .entry(typename.to_string())
                            .or_default()
                            .push((shape, attributes));
                    });

                // output file per typename
                for (typename, mut features) in categorized_shapes {
                    let table_info = make_field_list(&features);
                    fill_missing_fields(&mut features, &table_info);

                    let table_builder = TableBuilder::new(table_info);

                    let records = prepare_shapefile_attributes(&features);

                    // Create all the files needed for the shapefile to be complete (.shp, .shx, .dbf)
                    std::fs::create_dir_all(&self.output_path)?;
                    let mut file_path = self.output_path.clone();
                    file_path.push(format!("{}.shp", typename.replace(':', "_")));

                    let mut writer =
                        shapefile::Writer::from_path(file_path, table_builder.build())?;

                    // Write each feature
                    features.into_iter().zip_eq(records).for_each(
                        |((shape, _), record)| match shape {
                            shapefile::Shape::PolygonZ(polygon) => {
                                writer.write_shape_and_record(&polygon, &record).unwrap();
                            }
                            shapefile::Shape::NullShape => {
                                use shapefile::Point;
                                let point = Point::default();
                                writer.write_shape_and_record(&point, &record).unwrap();
                            }
                            _ => {
                                log::warn!("Unsupported shape type");
                            }
                        },
                    );
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

/// Create Shapefile features from a Entity
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
/// TODO: Implement MultiLineString and MultiPoint handling
pub fn entity_to_shapes(entity: &Entity) -> (shapefile::Shape, Map) {
    let mut attributes: IndexMap<String, Value, ahash::RandomState> = Map::default();

    let Value::Object(obj) = &entity.root else {
        return (shapefile::Shape::NullShape, attributes);
    };
    for (k, v) in &obj.attributes {
        attributes.insert(k.clone(), v.clone());
    }

    let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype else {
        return (shapefile::Shape::NullShape, attributes);
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

        return (shape, attributes);
    }

    (shapefile::Shape::NullShape, attributes)
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

        let (shapes, attributes) = entity_to_shapes(&obj);
        assert_eq!(attributes.len(), 0);

        if let shapefile::Shape::PolygonZ(polygon) = &shapes {
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
