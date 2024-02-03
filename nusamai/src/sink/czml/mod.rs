//! czml sink

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use rayon::prelude::*;

use nusamai_citygml::object::{Entity, ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryType;
use nusamai_czml::conversion::indexed_multipolygon_to_czml_polygon;
use nusamai_czml::{indexed_polygon_to_czml_polygon, Packet};

use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};

pub struct CzmlSinkProvider {}

impl DataSinkProvider for CzmlSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "CZML".to_string(),
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

        Box::<CzmlSink>::new(CzmlSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct CzmlSink {
    output_path: PathBuf,
}

impl DataSink for CzmlSink {
    fn make_transform_requirements(&self) -> transformer::Requirements {
        use transformer::RequirementItem;

        transformer::Requirements {
            mergedown: RequirementItem::Required(transformer::Mergedown::Geometry),
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        let (ra, rb) = rayon::join(
            || {
                // Convert Entity to CzmlPolygon objects
                upstream
                    .into_iter()
                    .par_bridge()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let packets = entity_to_packet(&parcel.entity, true);
                        for packet in packets {
                            let bytes = serde_json::to_vec(&packet).unwrap();
                            if sender.send(bytes).is_err() {
                                return Err(PipelineError::Canceled);
                            };
                        }

                        Ok(())
                    })
            },
            || {
                // Write CZML to a file
                let mut file = File::create(&self.output_path)?;
                let mut writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                // Write each Packet
                let mut iter = receiver.into_iter().peekable();
                while let Some(bytes) = iter.next() {
                    writer.write_all(&bytes)?;
                    if iter.peek().is_some() {
                        writer.write_all(b",")?;
                    };
                }

                // Write the FeautureCollection footer and EOL
                writer.write_all(b"]\n")?;
                Ok(())
            },
        );

        match ra {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.report_error(error),
        }
        match rb {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.report_error(error),
        }

        Ok(())
    }
}

// fn extract_properties(tree: &nusamai_citygml::object::Value) -> Option<geojson::JsonObject> {
//     match &tree {
//         obj @ nusamai_citygml::Value::Object(_) => match obj.to_attribute_json() {
//             serde_json::Value::Object(map) => Some(map),
//             _ => unreachable!(),
//         },
//         _ => panic!("Root value type must be Feature, but found {:?}", tree),
//     }
// }

/// Create CZML Packet from a Entity
pub fn entity_to_packet(entity: &Entity, single_part: bool) -> Vec<Packet> {
    // TODO: extract properties
    // let _properties = extract_properties(&entity.root);
    let geom_store = entity.geometry_store.read().unwrap();

    let Value::Object(obj) = &entity.root else {
        return Vec::default();
    };
    let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype else {
        return Vec::default();
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

    // Cesium requires a Packet called Document
    let doc = Packet {
        id: Some("document".into()),
        version: Some("1.0".into()),
        ..Default::default()
    };
    let mut packets = vec![doc];

    if !mpoly.is_empty() {
        // CZML does not support multi-part polygons due to its specification, so create a Packet for each face.
        if single_part {
            for poly in mpoly.iter() {
                let czml_polygon = indexed_polygon_to_czml_polygon(&geom_store.vertices, &poly);
                let packet = Packet {
                    polygon: Some(czml_polygon),
                    ..Default::default()
                };
                packets.push(packet);
            }
        } else {
            // TODO: Multi-part polygons are used in the glTF model
            let czml_polygon = indexed_multipolygon_to_czml_polygon(&geom_store.vertices, &mpoly);
            let packet = Packet {
                polygon: Some(czml_polygon),
                ..Default::default()
            };
            packets.push(packet);
        }
    }

    packets
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use nusamai_citygml::{object::Object, GeometryRefEntry, Value};
    use nusamai_czml::{PositionListProperties, PositionListType};
    use nusamai_geometry::MultiPolygon;
    use nusamai_projection::crs::EPSG_JGD2011_GEOGRAPHIC_3D;

    #[test]
    fn test_entity_multipolygon() {
        let vertices: Vec<[f64; 3]> = vec![
            // 1st polygon, exterior (vertex 0~3)
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
            // 1st polygon, interior 1 (vertex 4~7)
            [1., 1., 111.],
            [2., 1., 111.],
            [2., 2., 111.],
            [1., 2., 111.],
            // 1st polygon, interior 2 (vertex 8~11)
            [3., 3., 111.],
            [4., 3., 111.],
            [4., 4., 111.],
            [3., 4., 111.],
            // 2nd polygon, exterior (vertex 12~15)
            [4., 0., 222.],
            [7., 0., 222.],
            [7., 3., 222.],
            [4., 3., 222.],
            // 2nd polygon, interior (vertex 16~19)
            [5., 1., 222.],
            [6., 1., 222.],
            [6., 2., 222.],
            [5., 2., 222.],
            // 3rd polygon, exterior (vertex 20~23)
            [4., 0., 333.],
            [7., 0., 333.],
            [7., 3., 333.],
            [4., 3., 333.],
        ];

        let mut mpoly = MultiPolygon::<1, u32>::new();
        // 1st polygon
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        mpoly.add_interior([[4], [5], [6], [7], [4]]);
        mpoly.add_interior([[8], [9], [10], [11], [8]]);
        // 2nd polygon
        mpoly.add_exterior([[12], [13], [14], [15], [12]]);
        mpoly.add_interior([[16], [17], [18], [19], [16]]);
        // 3rd polygon
        mpoly.add_exterior([[20], [21], [22], [23], [20]]);

        let geometries = nusamai_citygml::GeometryStore {
            epsg: EPSG_JGD2011_GEOGRAPHIC_3D,
            vertices,
            multipolygon: mpoly,
            multilinestring: Default::default(),
            multipoint: Default::default(),
        };

        let entity = Entity {
            root: Value::Object(Object {
                typename: "dummy".into(),
                attributes: Default::default(),
                stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                    id: "dummy".into(),
                    geometries: vec![
                        GeometryRefEntry {
                            ty: GeometryType::Solid,
                            pos: 0,
                            len: 1,
                            lod: 1,
                        },
                        GeometryRefEntry {
                            ty: GeometryType::Solid,
                            pos: 1,
                            len: 1,
                            lod: 1,
                        },
                        GeometryRefEntry {
                            ty: GeometryType::Solid,
                            pos: 2,
                            len: 1,
                            lod: 1,
                        },
                    ],
                },
            }),
            geometry_store: RwLock::new(geometries).into(),
        };

        // test document packet
        let packets = entity_to_packet(&entity, true);
        assert_eq!(packets.len(), 4);
        assert_eq!(packets[0].id, Some("document".into()));

        // test first polygon packet
        let first_polygon = &packets[1].polygon;
        assert!(first_polygon.is_some());

        let first_polygon = first_polygon.as_ref().unwrap();
        let first_polygon_positions = PositionListProperties {
            cartographic_degrees: Some(vec![
                0., 0., 111., 5., 0., 111., 5., 5., 111., 0., 5., 111.,
            ]),
            ..Default::default()
        };
        assert_eq!(
            first_polygon.positions,
            Some(PositionListType::Object(first_polygon_positions))
        );

        // test second polygon packet
        let second_polygon = &packets[2].polygon;
        assert!(second_polygon.is_some());

        let second_polygon = second_polygon.as_ref().unwrap();
        let second_polygon_positions = PositionListProperties {
            cartographic_degrees: Some(vec![
                4., 0., 222., 7., 0., 222., 7., 3., 222., 4., 3., 222.,
            ]),
            ..Default::default()
        };
        assert_eq!(
            second_polygon.positions,
            Some(PositionListType::Object(second_polygon_positions))
        );

        // test third polygon packet
        let third_polygon = &packets[3].polygon;
        assert!(third_polygon.is_some());

        let third_polygon = third_polygon.as_ref().unwrap();
        let third_polygon_positions = PositionListProperties {
            cartographic_degrees: Some(vec![
                4., 0., 333., 7., 0., 333., 7., 3., 333., 4., 3., 333.,
            ]),
            ..Default::default()
        };
        assert_eq!(
            third_polygon.positions,
            Some(PositionListType::Object(third_polygon_positions))
        );
    }
}
