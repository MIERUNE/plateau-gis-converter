//! czml sink

use std::path::PathBuf;

use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryType;
use rayon::prelude::*;

use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};

use nusamai_citygml::object::{Entity, ObjectStereotype, Value};
use nusamai_czml::conversion::indexed_multipolygon_to_shape;

pub struct czmlSinkProvider {}

impl DataSinkProvider for czmlSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "czml".to_string(),
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

        Box::<czmlSink>::new(czmlSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct czmlSink {
    output_path: PathBuf,
}

impl DataSink for czmlSink {
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
                // Convert CityObjects to czml objects

                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        let shapes = toplevel_cityobj_to_shapes(&parcel.entity);
                        for shape in shapes {
                            if sender.send(shape).is_err() {
                                log::info!("sink cancelled");
                                return Err(());
                            };
                        }

                        Ok(())
                    },
                );
            },
            || {
                // Write czml to a file

                // Attribute fields for the features
                // FieldName byte representation cannot exceed 11 bytes
                let table_builder = czml::dbase::TableWriterBuilder::new();

                // Create all the files needed for the czml to be complete (.shp, .shx, .dbf)
                let mut writer = czml::Writer::from_path(&self.output_path, table_builder).unwrap();

                // Write each feature
                receiver.into_iter().for_each(|shape| match shape {
                    czml::Shape::PolygonZ(polygon) => {
                        let record = czml::dbase::Record::default(); // for attributes
                        writer.write_shape_and_record(&polygon, &record).unwrap();
                    }
                    czml::Shape::NullShape => {}
                    _ => {
                        log::warn!("Unsupported shape type");
                    }
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

/// Create czml features from a TopLevelCityObject
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
/// TODO: Implement MultiLineString and MultiPoint handling
pub fn toplevel_cityobj_to_shapes(entity: &Entity) -> Vec<czml::Shape> {
    let _properties = extract_properties(&entity.root);
    let geom_store = entity.geometry_store.read().unwrap();

    let Value::Object(obj) = &entity.root else {
        return vec![czml::Shape::NullShape];
    };
    let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype else {
        return vec![czml::Shape::NullShape];
    };

    let mut mpoly = nusamai_geometry::MultiPolygon::<1, u32>::new();

    geometries.iter().for_each(|entry| match entry.ty {
        GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
            for idx_poly in geom_store
                .multipolygon
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                mpoly.add(idx_poly);
            }
        }
        GeometryType::Curve => unimplemented!(),
        GeometryType::Point => unimplemented!(),
    });

    let mut shapes = vec![];
    if !mpoly.is_empty() {
        let shape = indexed_multipolygon_to_shape(&geom_store.vertices, &mpoly);
        shapes.push(czml::Shape::PolygonZ(shape));
    }
    if !shapes.is_empty() {
        return shapes;
    }

    vec![czml::Shape::NullShape]
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use czml::NO_DATA;
    use nusamai_citygml::{object::Object, GeometryRefEntry, Value};
    use nusamai_geometry::MultiPolygon;
    use nusamai_projection::crs::EPSG_JGD2011_GEOGRAPHIC_3D;

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

        let shapes = toplevel_cityobj_to_shapes(&obj);
        assert_eq!(shapes.len(), 1);

        if let czml::Shape::PolygonZ(polygon) = &shapes[0] {
            assert_eq!(polygon.rings().len(), 1);
            assert_eq!(
                polygon.ring(0).unwrap(),
                &czml::PolygonRing::Outer(vec![
                    // Outer ring: re-ordered to clockwise
                    czml::PointZ::new(0., 0., 111., NO_DATA),
                    czml::PointZ::new(0., 5., 111., NO_DATA),
                    czml::PointZ::new(5., 5., 111., NO_DATA),
                    czml::PointZ::new(5., 0., 111., NO_DATA),
                    czml::PointZ::new(0., 0., 111., NO_DATA), // automatically closed
                ])
            )
        } else {
            panic!("Expected PolygonZ.");
        }
    }
}
