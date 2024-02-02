//! czml sink

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use rayon::prelude::*;

use nusamai_citygml::object::{Entity, ObjectStereotype, Value};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::GeometryType;
use nusamai_czml::conversion::indexed_multilinestring_to_czml_polygon;
use nusamai_czml::Packet;

use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
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

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        rayon::join(
            || {
                // Convert Entity to CzmlPolygon objects
                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        let packets = entity_to_packet(&parcel.entity);
                        for packet in packets {
                            let Ok(bytes) = serde_json::to_vec(&packet) else {
                                // TODO: fatal error
                                return Err(());
                            };
                            if sender.send(bytes).is_err() {
                                log::info!("sink cancelled");
                                return Err(());
                            };
                        }

                        Ok(())
                    },
                );
            },
            || {
                // Write CZML to a file
                let mut file = File::create(&self.output_path).unwrap();
                let mut writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                // Cesium requires a Packet called Document
                writer
                    .write_all(b"[{\"id\":\"document\",\"version\":\"1.0\"},")
                    .unwrap();

                // Write each Packet
                let mut iter = receiver.into_iter().peekable();
                while let Some(bytes) = iter.next() {
                    writer.write_all(&bytes).unwrap();
                    if iter.peek().is_some() {
                        writer.write_all(b",").unwrap();
                    };
                }

                // Write the FeautureCollection footer and EOL
                writer.write_all(b"]\n").unwrap();
            },
        );
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
pub fn entity_to_packet(entity: &Entity) -> Vec<Packet> {
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

    let mut packets = vec![];
    if !mpoly.is_empty() {
        let czml_polygon = indexed_multilinestring_to_czml_polygon(&geom_store.vertices, &mpoly);

        let packet = Packet {
            polygon: Some(czml_polygon),
            ..Default::default()
        };

        packets.push(packet);
    }

    packets
}

#[cfg(test)]
mod tests {}
