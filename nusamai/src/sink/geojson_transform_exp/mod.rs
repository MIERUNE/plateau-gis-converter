//! GeoJSON sink

mod transform;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use nusamai_citygml::schema::Schema;
use rayon::prelude::*;

use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};
use transform::ObjectTransformer;

use nusamai_geojson::conversion::{
    indexed_multilinestring_to_value, indexed_multipoint_to_value, indexed_multipolygon_to_value,
};
use nusamai_plateau::Entity;

pub struct GeoJsonTransformExpSinkProvider {}

impl DataSinkProvider for GeoJsonTransformExpSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "GeoJSON".to_string(),
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

        Box::<GeoJsonTfExpSink>::new(GeoJsonTfExpSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct GeoJsonTfExpSink {
    output_path: PathBuf,
}

impl DataSink for GeoJsonTfExpSink {
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

                        // todo: parse attributes
                        let object_transformer = ObjectTransformer {};
                        let entity = object_transformer
                            .transform(parcel.entity, &PathBuf::from("./mappings.json"));

                        let features = entity_to_geojson_features(&entity[0]);
                        for feature in features {
                            let bytes = serde_json::to_vec(&feature).unwrap();
                            if sender.send(bytes).is_err() {
                                return Err(PipelineError::Canceled);
                            };
                        }
                        Ok(())
                    })
            },
            || {
                // Write GeoJSON to a file

                let mut file = File::create(&self.output_path)?;
                let mut writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                // Write the FeatureCollection header
                writer.write_all(b"{\"type\":\"FeatureCollection\",\"features\":[")?;

                // Write each Feature
                let mut iter = receiver.into_iter().peekable();
                while let Some(bytes) = iter.next() {
                    writer.write_all(&bytes)?;
                    if iter.peek().is_some() {
                        writer.write_all(b",")?;
                    };
                }

                // Write the FeautureCollection footer and EOL
                writer.write_all(b"]}\n")?;
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

fn extract_properties(tree: &nusamai_citygml::object::Value) -> Option<geojson::JsonObject> {
    match &tree {
        obj @ nusamai_citygml::Value::Object(_) => match obj.to_attribute_json() {
            serde_json::Value::Object(map) => Some(map),
            _ => unreachable!(),
        },
        _ => panic!(
            "Root value type must be Feature or Data, but found {:?}",
            tree
        ),
    }
}

/// Create GeoJSON features from a TopLevelCityObject
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
// TODO: Handle properties (`obj.root` -> `geojson::Feature.properties`)
// TODO: We may want to traverse the tree and create features for each semantic child in the future
pub fn entity_to_geojson_features(obj: &Entity) -> Vec<geojson::Feature> {
    let mut geojson_features: Vec<geojson::Feature> = Vec::with_capacity(1);
    let properties = extract_properties(&obj.root);
    let geom_store = obj.geometry_store.read().unwrap();

    if !geom_store.multipolygon.is_empty() {
        let mpoly_geojson_geom =
            indexed_multipolygon_to_value(&geom_store.vertices, &geom_store.multipolygon);

        let mpoly_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoly_geojson_geom.into()),
            id: None,
            properties: properties.clone(),
            foreign_members: None,
        };
        geojson_features.push(mpoly_geojson_feat);
    }

    if !geom_store.multilinestring.is_empty() {
        let mls_geojson_geom =
            indexed_multilinestring_to_value(&geom_store.vertices, &geom_store.multilinestring);
        let mls_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mls_geojson_geom.into()),
            id: None,
            properties: properties.clone(),
            foreign_members: None,
        };
        geojson_features.push(mls_geojson_feat);
    }

    if !geom_store.multipoint.is_empty() {
        let mpoint_geojson_geom =
            indexed_multipoint_to_value(&geom_store.vertices, &geom_store.multipoint);
        let mpoint_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoint_geojson_geom.into()),
            id: None,
            properties,
            foreign_members: None,
        };
        geojson_features.push(mpoint_geojson_feat);
    }

    geojson_features
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use nusamai_citygml::{
        object::{self, Object},
        Value,
    };
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
            ..Default::default()
        };

        let obj = Entity {
            root: Value::Object(Object {
                typename: "dummy".into(),
                stereotype: object::ObjectStereotype::Feature {
                    id: "dummy".into(),
                    geometries: Vec::default(),
                },
                attributes: Default::default(),
            }),
            geometry_store: RwLock::new(geometries).into(),
            appearance_store: Default::default(),
        };

        let geojson_features = entity_to_geojson_features(&obj);
        assert_eq!(geojson_features.len(), 1);

        let mpoly_geojson = geojson_features.first().unwrap();
        assert!(mpoly_geojson.bbox.is_none());
        assert!(mpoly_geojson.foreign_members.is_none());
        if let geojson::Value::MultiPolygon(rings_list) =
            mpoly_geojson.geometry.clone().unwrap().value
        {
            for (i, rings) in rings_list.iter().enumerate() {
                match i {
                    0 => {
                        assert_eq!(rings.len(), 1);
                        assert_eq!(
                            rings[0],
                            vec![
                                [0., 0., 111.],
                                [5., 0., 111.],
                                [5., 5., 111.],
                                [0., 5., 111.],
                                [0., 0., 111.]
                            ]
                        );
                    }
                    _ => unreachable!("Unexpected number of polygons"),
                }
            }
        } else {
            unreachable!("The result is not a GeoJSON MultiPolygon");
        };
    }
}
