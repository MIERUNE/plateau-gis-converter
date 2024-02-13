//! gltf sink poc

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use ahash::RandomState;
use byteorder::{ByteOrder, LittleEndian};
use earcut_rs::utils_3d::project3d_to_2d;
use earcut_rs::Earcut;
use indexmap::IndexSet;
use nusamai_gltf_json::extensions::mesh::ext_mesh_features::FeatureId;
use nusamai_projection::cartesian::geographic_to_geocentric;
use rayon::prelude::*;

use nusamai_citygml::object::ObjectStereotype;
use nusamai_citygml::schema::Schema;
use nusamai_citygml::{GeometryType, Value};
use nusamai_gltf_json::*;

use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vertex {
    pub position: [u32; 3],  // f32.to_bits()
    pub tex_coord: [u32; 2], // f32.to_bits()
    pub feature_id: u32,
}

pub struct BoundingVolume {
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

pub struct GltfPocSinkProvider {}

impl DataSinkProvider for GltfPocSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "glTF".to_string(),
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

        Box::<GltfPocSink>::new(GltfPocSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct GltfPocSink {
    output_path: PathBuf,
}

impl DataSink for GltfPocSink {
    fn make_transform_requirements(&self) -> transformer::Requirements {
        transformer::Requirements {
            resolve_appearance: true,
            ..Default::default()
        }
    }

    fn run(
        &mut self,
        upstream: Receiver,
        feedback: &Feedback,
        _schema: &Schema,
    ) -> Result<(), PipelineError> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        rayon::join(
            || {
                let ellipsoid = nusamai_projection::ellipsoid::wgs84();

                // Convert Entity to CzmlPolygon objects
                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_canceled() {
                            return Err(PipelineError::Canceled);
                        }

                        let mut bounding_volume = BoundingVolume {
                            min_lng: f64::MAX,
                            max_lng: f64::MIN,
                            min_lat: f64::MAX,
                            max_lat: f64::MIN,
                            min_height: f64::MAX,
                            max_height: f64::MIN,
                        };

                        // todo: transformerからschemaを受け取る必要がある

                        let entity = parcel.entity;
                        let geom_store = entity.geometry_store.read().unwrap();

                        let Value::Object(obj) = &entity.root else {
                            unimplemented!()
                        };
                        let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype
                        else {
                            unimplemented!()
                        };

                        // Divide polygons into triangles
                        let mut earcutter = Earcut::new();
                        let mut buf3d: Vec<f64> = Vec::new();
                        let mut buf2d: Vec<f64> = Vec::new();
                        let mut triangles_buf: Vec<u32> = Vec::new();
                        let mut triangles = Vec::new();

                        // extract triangles from entity
                        geometries.iter().for_each(|entry| match entry.ty {
                            GeometryType::Solid
                            | GeometryType::Surface
                            | GeometryType::Triangle => {
                                for idx_poly in geom_store.multipolygon.iter_range(
                                    entry.pos as usize..(entry.pos + entry.len) as usize,
                                ) {
                                    let mut poly = idx_poly.transform(|idx| {
                                        let [lng, lat, height] =
                                            geom_store.vertices[idx[0] as usize];
                                        [lng, lat, height]
                                    });

                                    poly.transform_inplace(|[lng, lat, height]| {
                                        bounding_volume.min_lng = bounding_volume.min_lng.min(*lng);
                                        bounding_volume.max_lng = bounding_volume.max_lng.max(*lng);
                                        bounding_volume.min_lat = bounding_volume.min_lat.min(*lat);
                                        bounding_volume.max_lat = bounding_volume.max_lat.max(*lat);
                                        bounding_volume.min_height =
                                            bounding_volume.min_height.min(*height);
                                        bounding_volume.max_height =
                                            bounding_volume.max_height.max(*height);

                                        // Convert to geocentric (x, y, z) coordinate.
                                        // (Earcut do not work in geographic space)

                                        // WGS84 to Geocentric
                                        let (x, y, z) = geographic_to_geocentric(
                                            &ellipsoid, *lng, *lat, *height,
                                        );
                                        // OpenGL is a right-handed y-up
                                        [x, z, -y]
                                    });

                                    let num_outer = match poly.hole_indices().first() {
                                        Some(&v) => v as usize,
                                        None => poly.coords().len() / 3,
                                    };

                                    buf3d.clear();
                                    buf3d.extend(poly.coords());

                                    if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                                        // earcut
                                        earcutter.earcut(
                                            &buf2d,
                                            poly.hole_indices(),
                                            2,
                                            &mut triangles_buf,
                                        );
                                        triangles.extend(triangles_buf.iter().map(|idx| {
                                            [
                                                buf3d[*idx as usize * 3],
                                                buf3d[*idx as usize * 3 + 1],
                                                buf3d[*idx as usize * 3 + 2],
                                            ]
                                        }));
                                    }
                                }
                            }
                            GeometryType::Curve => unimplemented!(),
                            GeometryType::Point => unimplemented!(),
                        });

                        // extract attributes from entity
                        let Value::Object(obj) = &entity.root else {
                            unimplemented!()
                        };
                        let attributes = obj.attributes.clone();

                        // send triangles and attributes to sender
                        if sender
                            .send((triangles, attributes, bounding_volume))
                            .is_err()
                        {
                            return Err(PipelineError::Canceled);
                        }

                        Ok(())
                    },
                );
            },
            || {
                // Write glTF to a file
                // todo: schemaから属性定義を行う必要がある

                let mut buffers: Vec<[f64; 4]> = Vec::new();

                let mut all_max: [f64; 3] = [f64::MIN; 3];
                let mut all_min: [f64; 3] = [f64::MAX; 3];

                let mut bounding_volume = BoundingVolume {
                    min_lng: f64::MAX,
                    max_lng: f64::MIN,
                    min_lat: f64::MAX,
                    max_lat: f64::MIN,
                    min_height: f64::MAX,
                    max_height: f64::MIN,
                };

                for (feature_id, (triangles, _attributes, _bounding_volume)) in
                    receiver.into_iter().enumerate()
                {
                    let mut pos_max = [f64::MIN; 3];
                    let mut pos_min = [f64::MAX; 3];

                    // calculate the centroid and min/max
                    for &[x, y, z] in &triangles {
                        pos_min = [
                            f64::min(pos_min[0], x),
                            f64::min(pos_min[1], y),
                            f64::min(pos_min[2], z),
                        ];
                        pos_max = [
                            f64::max(pos_max[0], x),
                            f64::max(pos_max[1], y),
                            f64::max(pos_max[2], z),
                        ];
                        buffers.push([x, y, z, feature_id as f64]);
                    }

                    all_min = [
                        f64::min(all_min[0], pos_min[0]),
                        f64::min(all_min[1], pos_min[1]),
                        f64::min(all_min[2], pos_min[2]),
                    ];
                    all_max = [
                        f64::max(all_max[0], pos_max[0]),
                        f64::max(all_max[1], pos_max[1]),
                        f64::max(all_max[2], pos_max[2]),
                    ];

                    bounding_volume.min_lng =
                        f64::min(bounding_volume.min_lng, _bounding_volume.min_lng);
                    bounding_volume.max_lng =
                        f64::max(bounding_volume.max_lng, _bounding_volume.max_lng);
                    bounding_volume.min_lat =
                        f64::min(bounding_volume.min_lat, _bounding_volume.min_lat);
                    bounding_volume.max_lat =
                        f64::max(bounding_volume.max_lat, _bounding_volume.max_lat);
                    bounding_volume.min_height =
                        f64::min(bounding_volume.min_height, _bounding_volume.min_height);
                    bounding_volume.max_height =
                        f64::max(bounding_volume.max_height, _bounding_volume.max_height);
                }

                // calculate the centroid
                let mut all_translation = [0.; 3];
                all_translation[0] = (all_max[0] + all_min[0]) / 2.;
                all_translation[1] = (all_max[1] + all_min[1]) / 2.;
                all_translation[2] = (all_max[2] + all_min[2]) / 2.;

                all_max[0] -= all_translation[0];
                all_min[0] -= all_translation[0];
                all_max[1] -= all_translation[1];
                all_min[1] -= all_translation[1];
                all_max[2] -= all_translation[2];
                all_min[2] -= all_translation[2];

                // make vertices and indices
                let mut vertices: IndexSet<Vertex, RandomState> = IndexSet::default();

                let indices: Vec<u32> = buffers
                    .iter()
                    .map(|&[x, y, z, feature_id]| {
                        let (x, y, z) = (
                            x - all_translation[0],
                            y - all_translation[1],
                            z - all_translation[2],
                        );
                        let vbits = [
                            (x as f32).to_bits(),
                            (y as f32).to_bits(),
                            (z as f32).to_bits(),
                        ];

                        let vertex = Vertex {
                            position: vbits,
                            feature_id: feature_id as u32,
                            ..Default::default()
                        };

                        let (index, _) = vertices.insert_full(vertex);

                        index as u32
                    })
                    .collect();

                let indices: Vec<u32> = indices
                    .chunks_exact(3)
                    .filter(|idx| (idx[0] != idx[1] && idx[1] != idx[2] && idx[2] != idx[0]))
                    .flatten()
                    .copied()
                    .collect();

                let mut file = File::create(&self.output_path).unwrap();
                let writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                write_gltf(writer, all_min, all_max, all_translation, vertices, indices);

                let region: [f64; 6] = [
                    bounding_volume.min_lng.to_radians(),
                    bounding_volume.min_lat.to_radians(),
                    bounding_volume.max_lng.to_radians(),
                    bounding_volume.max_lat.to_radians(),
                    bounding_volume.min_height,
                    bounding_volume.max_height,
                ];

                write_3dtiles(region, &self.output_path);

                // todo: 属性部分をbufferにするコードを書く
            },
        );
        Ok(())
    }
}

fn write_gltf<W: Write>(
    mut writer: W,
    min: [f64; 3],
    max: [f64; 3],
    translation: [f64; 3],
    vertices: IndexSet<Vertex, RandomState>,
    indices: impl IntoIterator<Item = u32>,
) {
    let mut bin_content: Vec<u8> = Vec::new();

    // write vertices
    let vertices_offset = bin_content.len();
    let mut buf = [0; 12];
    let mut vertices_count = 0;
    for vertex in vertices.clone() {
        LittleEndian::write_u32_into(&vertex.position, &mut buf);
        bin_content.write_all(&buf).unwrap();
        vertices_count += 1;
    }
    let vertices_len: usize = bin_content.len() - vertices_offset;

    // write indices
    let indices_offset = bin_content.len();
    let mut indices_count = 0;
    for idx in indices {
        bin_content.write_all(&idx.to_le_bytes()).unwrap();
        indices_count += 1;
    }
    let indices_len = bin_content.len() - indices_offset;

    // write feature_ids
    let feature_ids_offset = bin_content.len();
    let mut feature_ids_count = 0;
    for vertex in vertices.clone() {
        bin_content
            .write_all(&vertex.feature_id.to_le_bytes())
            .unwrap();
        feature_ids_count += 1;
    }
    let feature_ids_len = bin_content.len() - feature_ids_offset;

    let gltf = Gltf {
        extensions_used: vec!["EXT_mesh_features".to_string()],
        scenes: vec![Scene {
            nodes: Some(vec![0]),
            ..Default::default()
        }],
        nodes: vec![Node {
            mesh: Some(0),
            translation,
            ..Default::default()
        }],
        meshes: vec![Mesh {
            primitives: vec![MeshPrimitive {
                attributes: vec![
                    ("POSITION".to_string(), 0),
                    ("_FEATURE_ID_0".to_string(), 2),
                ]
                .into_iter()
                .collect(),
                indices: Some(1),
                mode: PrimitiveMode::Triangles,
                extensions: Some(extensions::mesh::MeshPrimitive {
                    ext_mesh_features: Some(extensions::mesh::ext_mesh_features::ExtMeshFeatures {
                        feature_ids: vec![FeatureId {
                            attribute: Some(0),
                            feature_count: feature_ids_count,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        }],
        accessors: vec![
            Accessor {
                buffer_view: Some(0),
                component_type: ComponentType::Float,
                count: vertices_count,
                min: Some(min.to_vec()),
                max: Some(max.to_vec()),
                type_: AccessorType::Vec3,
                ..Default::default()
            },
            Accessor {
                buffer_view: Some(1),
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            },
            Accessor {
                buffer_view: Some(2),
                component_type: ComponentType::UnsignedInt,
                count: feature_ids_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            },
        ],
        buffer_views: vec![
            BufferView {
                byte_offset: vertices_offset as u32,
                byte_length: vertices_len as u32,
                target: Some(BufferViewTarget::ArrayBuffer),
                ..Default::default()
            },
            BufferView {
                byte_offset: indices_offset as u32,
                byte_length: indices_len as u32,
                target: Some(BufferViewTarget::ElementArrayBuffer),
                ..Default::default()
            },
            BufferView {
                byte_offset: feature_ids_offset as u32,
                byte_length: feature_ids_len as u32,
                target: Some(BufferViewTarget::ArrayBuffer),
                ..Default::default()
            },
        ],
        buffers: vec![Buffer {
            byte_length: bin_content.len() as u32,
            ..Default::default()
        }],
        ..Default::default()
    };

    {
        let mut json_content = serde_json::to_vec(&gltf).unwrap();

        // append padding
        json_content.extend(vec![0x20; (4 - (json_content.len() % 4)) % 4].iter());
        bin_content.extend(vec![0x0; (4 - (bin_content.len() % 4)) % 4].iter());

        let total_size = 12 + 8 + json_content.len() + 8 + bin_content.len();

        writer.write_all(b"glTF").unwrap(); // magic
        writer.write_all(&2u32.to_le_bytes()).unwrap(); // version: 2
        writer
            .write_all(&(total_size as u32).to_le_bytes())
            .unwrap(); // total size

        writer
            .write_all(&(json_content.len() as u32).to_le_bytes())
            .unwrap(); // json content
        writer.write_all(b"JSON").unwrap(); // chunk type
        writer.write_all(&json_content).unwrap(); // json content

        writer
            .write_all(&(bin_content.len() as u32).to_le_bytes())
            .unwrap(); // json content
        writer.write_all(b"BIN\0").unwrap(); // chunk type
        writer.write_all(&bin_content).unwrap(); // bin content

        writer.flush().unwrap();
    }
}

// FIXME: This is the code to verify the operation with Cesium
fn write_3dtiles(bounding_volume: [f64; 6], output_path: &Path) {
    // write 3DTiles
    let tileset_path = output_path.with_file_name("tileset.json");
    let content_uri = output_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let tileset = nusamai_3dtiles_json::tileset::Tileset {
        geometric_error: 1e+100,
        asset: nusamai_3dtiles_json::tileset::Asset {
            version: "1.1".to_string(),
            ..Default::default()
        },
        root: nusamai_3dtiles_json::tileset::Tile {
            bounding_volume: nusamai_3dtiles_json::tileset::BoundingVolume {
                region: Some(bounding_volume),
                ..Default::default()
            },
            content: Some(nusamai_3dtiles_json::tileset::Content {
                uri: content_uri,
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tileset_file = File::create(tileset_path).unwrap();
    let tileset_writer = BufWriter::with_capacity(1024 * 1024, &mut tileset_file);
    serde_json::to_writer_pretty(tileset_writer, &tileset).unwrap();
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use nusamai_citygml::{object::Object, GeometryRef, Value};
    use nusamai_geometry::MultiPolygon;
    use nusamai_plateau::Entity;
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
            ..Default::default()
        };

        let _entity = Entity {
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
            geometry_store: RwLock::new(geometries).into(),
            appearance_store: Default::default(),
        };
    }
}
