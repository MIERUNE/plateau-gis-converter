//! czml sink

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};
use nusamai_czml::{
    conversion::indexed_multipolygon_to_czml_polygon, indexed_polygon_to_czml_polygon, CzmlBoolean,
    Packet, StringProperties, StringValueType,
};
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

pub struct CzmlSinkProvider {}

impl DataSinkProvider for CzmlSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "czml".to_string(),
            name: "CZML".to_string(),
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

        Box::<CzmlSink>::new(CzmlSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings,
        })
    }
}

pub struct CzmlSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
}

impl DataSink for CzmlSink {
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
                // Convert Entity to CzmlPolygon objects
                upstream
                    .into_iter()
                    .par_bridge()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let packets = entity_to_packets(parcel.entity, true);
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

                // Cesium requires a Packet called Document
                let doc = Packet {
                    id: Some("document".into()),
                    version: Some("1.0".into()),
                    ..Default::default()
                };
                write!(writer, r#"[{},"#, serde_json::to_string(&doc).unwrap())?;

                // Write each Packet
                let mut iter = receiver.into_iter().peekable();
                while let Some(bytes) = iter.next() {
                    feedback.ensure_not_canceled()?;

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
            Err(error) => feedback.fatal_error(error),
        }
        match rb {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.fatal_error(error),
        }

        Ok(())
    }
}

fn extract_properties(tree: &nusamai_citygml::object::Value) -> String {
    match &tree {
        obj @ nusamai_citygml::Value::Object(_) => match obj.to_attribute_json() {
            serde_json::Value::Object(map) => map_to_html_table(&map),
            _ => unreachable!(),
        },
        _ => panic!("Root value type must be Feature, but found {:?}", tree),
    }
}

fn map_to_html_table(map: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut html = String::new();
    html.push_str("<table>");
    for (key, value) in map {
        html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", key, value));
    }
    html.push_str("</table>");
    html
}

/// Create CZML Packet from a Entity
pub fn entity_to_packets(entity: Entity, single_part: bool) -> Vec<Packet> {
    let properties = extract_properties(&entity.root);
    let geom_store = entity.geometry_store.read().unwrap();

    let Value::Object(obj) = entity.root else {
        return Vec::default();
    };

    let ObjectStereotype::Feature {
        id: parent_id,
        geometries,
    } = obj.stereotype
    else {
        return Vec::default();
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

    // Create a Packet that retains attributes and references it from child features
    let properties_packet = Packet {
        id: Some(parent_id.clone()),
        description: Some(StringValueType::String(properties)),
        ..Default::default()
    };
    let mut packets: Vec<Packet> = vec![properties_packet];

    if !mpoly.is_empty() {
        // CZML does not support multi-part polygons due to its specification, so create a Packet for each face.
        if single_part {
            for poly in mpoly.iter() {
                let mut czml_polygon = indexed_polygon_to_czml_polygon(&geom_store.vertices, &poly);
                // In Cesium, if perPositionHeight is false, the polygon height is fixed
                czml_polygon.per_position_height = CzmlBoolean::Boolean(true);

                let packet = Packet {
                    polygon: Some(czml_polygon),
                    description: Some(StringValueType::Object(StringProperties {
                        reference: Some(format!("{parent_id}#description")),
                        ..Default::default()
                    })),
                    parent: Some(parent_id.clone()),
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

    use flatgeom::MultiPolygon;
    use nusamai_citygml::{object::Object, GeometryRef};
    use nusamai_czml::{PositionListProperties, PositionListType};
    use nusamai_projection::crs::EPSG_JGD2011_GEOGRAPHIC_3D;

    use super::*;

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

        let mut mpoly = MultiPolygon::<u32>::new();
        // 1st polygon
        mpoly.add_exterior([0, 1, 2, 3, 0]);
        mpoly.add_interior([4, 5, 6, 7, 4]);
        mpoly.add_interior([8, 9, 10, 11, 8]);
        // 2nd polygon
        mpoly.add_exterior([12, 13, 14, 15, 12]);
        mpoly.add_interior([16, 17, 18, 19, 16]);
        // 3rd polygon
        mpoly.add_exterior([20, 21, 22, 23, 20]);

        let geometries = nusamai_citygml::GeometryStore {
            epsg: EPSG_JGD2011_GEOGRAPHIC_3D,
            vertices,
            multipolygon: mpoly,
            ..Default::default()
        };

        let entity = Entity {
            root: Value::Object(Object {
                typename: "dummy".into(),
                attributes: Default::default(),
                stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                    id: "dummy".into(),
                    geometries: vec![
                        GeometryRef {
                            ty: GeometryType::Solid,
                            pos: 0,
                            len: 1,
                            lod: 1,
                        },
                        GeometryRef {
                            ty: GeometryType::Solid,
                            pos: 1,
                            len: 1,
                            lod: 1,
                        },
                        GeometryRef {
                            ty: GeometryType::Solid,
                            pos: 2,
                            len: 1,
                            lod: 1,
                        },
                    ],
                },
            }),
            base_url: url::Url::parse("file:///dummy").unwrap(),
            geometry_store: RwLock::new(geometries).into(),
            appearance_store: Default::default(),
        };

        let packets = entity_to_packets(entity, true);
        assert_eq!(packets.len(), 4);

        // test parent packet
        let parent = &packets[0];
        assert_eq!(parent.id, Some("dummy".into()));
        assert_eq!(
            parent.description,
            Some(StringValueType::String(
                r#"<table><tr><td>id</td><td>"dummy"</td></tr><tr><td>type</td><td>"dummy"</td></tr></table>"#.into()
            ))
        );

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
