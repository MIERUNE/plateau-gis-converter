//! Shapefile sink

use std::path::PathBuf;

use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryType;
use rayon::prelude::*;

use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};

use nusamai_citygml::object::{Entity, ObjectStereotype, Value};
use nusamai_shapefile::conversion::indexed_polygon_to_shape;
use shapefile;

pub struct ShapefileSinkProvider {}

impl DataSinkProvider for ShapefileSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
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
    fn make_transform_requirements(&self) -> transformer::Requirements {
        use transformer::RequirementItem;

        transformer::Requirements {
            mergedown: RequirementItem::Required(transformer::Mergedown::Geometry),
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        rayon::join(
            || {
                // Convert CityObjects to Shapefile objects

                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        let features = toplevel_cityobj_to_geojson_features(&parcel.entity);
                        for feature in features {
                            if sender.send(feature).is_err() {
                                log::info!("sink cancelled");
                                return Err(());
                            };
                        }
                        Ok(())
                    },
                );
            },
            || {
                // Write Shapefile to a file

                // Attribute fields for the features
                // FieldName byte representation cannot exceed 11 bytes
                let table_builder = shapefile::dbase::TableWriterBuilder::new();

                // Create all the files needed for the shapefile to be complete (.shp, .shx, .dbf)
                let mut writer =
                    shapefile::Writer::from_path(&self.output_path, table_builder).unwrap();

                // Write each feature
                receiver.into_iter().for_each(|feature| {
                    let record = shapefile::dbase::Record::default(); // for attributes
                    writer.write_shape_and_record(&feature, &record).unwrap();
                });
            },
        );
    }
}

fn extract_properties(tree: &nusamai_citygml::object::Value) -> Option<geojson::JsonObject> {
    match &tree {
        obj @ nusamai_citygml::Value::Object(_) => match obj.to_attribute_json() {
            serde_json::Value::Object(map) => Some(map),
            _ => unreachable!(),
        },
        _ => panic!("Root value type must be Feature, but found {:?}", tree),
    }
}

/// Create Shapefile features from a TopLevelCityObject
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
/// TODO: Implement MultiLineString and MultiPoint handling
pub fn toplevel_cityobj_to_shape_features(entity: &Entity) -> Vec<shapefile::PolygonZ> {
    let _properties = extract_properties(&entity.root);
    let geom_store = entity.geometry_store.read().unwrap();

    let Value::Object(obj) = &entity.root else {
        return Vec::default();
    };
    let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype else {
        return Vec::default();
    };

    let mut polygons = Vec::new();

    geometries.iter().for_each(|entry| match entry.ty {
        GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
            for idx_poly in geom_store
                .multipolygon
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                let shape = indexed_polygon_to_shape(&geom_store.vertices, &idx_poly);
                polygons.push(shape);
            }
        }
        GeometryType::Curve => {
            for _idx_ls in geom_store
                .multilinestring
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                unimplemented!();
            }
        }
        GeometryType::Point => {
            for _idx_point in geom_store
                .multipoint
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                unimplemented!();
            }
        }
    });

    // TODO: polygons to a multipolygon
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use nusamai_citygml::{object::Object, GeometryRefEntry, Value};
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
            multilinestring: Default::default(),
            multipoint: Default::default(),
        };

        let obj = Entity {
            root: Value::Object(Object {
                typename: "dummy".into(),
                attributes: Default::default(),
                stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                    id: "dummy".into(),
                    geometries: vec![GeometryRefEntry {
                        ty: GeometryType::Solid,
                        pos: 0,
                        len: 1,
                        lod: 1,
                    }],
                },
            }),
            geometry_store: RwLock::new(geometries).into(),
        };

        let shapes = toplevel_cityobj_to_geojson_features(&obj);
        assert_eq!(shapes.len(), 1);

        let mpoly_shape = shapes.first().unwrap();
        assert_eq!(mpoly_shape.rings().len(), 1);
        assert_eq!(
            mpoly_shape.ring(0).unwrap(),
            &shapefile::PolygonRing::Outer(vec![
                // Outer ring: re-ordered to clockwise
                shapefile::PointZ::new(0., 0., 111., NO_DATA),
                shapefile::PointZ::new(0., 5., 111., NO_DATA),
                shapefile::PointZ::new(5., 5., 111., NO_DATA),
                shapefile::PointZ::new(5., 0., 111., NO_DATA),
                shapefile::PointZ::new(0., 0., 111., NO_DATA), // automatically closed
            ])
        )
    }
}
