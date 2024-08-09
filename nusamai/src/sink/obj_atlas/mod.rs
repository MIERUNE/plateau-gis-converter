//! obj sink
mod material;
mod obj_writer;

use std::{f64::consts::FRAC_PI_2, path::PathBuf, sync::Mutex};

use ahash::HashMap;
use earcut::{utils3d::project3d_to_2d, Earcut};
use flatgeom::MultiPolygon;
use glam::{DMat4, DVec3, DVec4};
use indexmap::IndexSet;
use itertools::Itertools;
use material::{Material, Texture};
use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};
use nusamai_plateau::appearance;
use nusamai_projection::cartesian::geodetic_to_geocentric;
use obj_writer::write_obj;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
    transformer::{TransformerConfig, TransformerOption, TransformerRegistry},
};

pub struct ObjAtlasSinkProvider {}

impl DataSinkProvider for ObjAtlasSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "obj_atlas".to_string(),
            name: "OBJ_ATLAS".to_string(),
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
                label: None,
            },
        );

        params.define(
            "transform".into(),
            ParameterEntry {
                description: "transform option".into(),
                required: false,
                parameter: ParameterType::String(StringParameter { value: None }),
                label: None,
            },
        );

        params.define(
            "split".into(),
            ParameterEntry {
                description: "Splitting objects".into(),
                required: true,
                parameter: ParameterType::Boolean(BooleanParameter { value: Some(false) }),
                label: Some("オブジェクトを分割する".into()),
            },
        );

        params
    }

    fn available_transformer(&self) -> TransformerRegistry {
        let mut settings: TransformerRegistry = TransformerRegistry::new();

        settings.insert(TransformerConfig {
            key: "use_texture".to_string(),
            label: "テクスチャの使用".to_string(),
            is_enabled: false,
            requirements: vec![transformer::Requirement::UseAppearance],
        });

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_options = self.available_transformer();
        let is_split = get_parameter_value!(params, "split", Boolean).unwrap();

        Box::<ObjAtlasSink>::new(ObjAtlasSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings: transform_options,
            obj_options: ObjParams { is_split },
        })
    }
}

pub struct ObjAtlasSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
    obj_options: ObjParams,
}

struct ObjParams {
    is_split: bool,
}

pub struct BoundingVolume {
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl BoundingVolume {
    fn update(&mut self, other: &Self) {
        self.min_lng = self.min_lng.min(other.min_lng);
        self.max_lng = self.max_lng.max(other.max_lng);
        self.min_lat = self.min_lat.min(other.min_lat);
        self.max_lat = self.max_lat.max(other.max_lat);
        self.min_height = self.min_height.min(other.min_height);
        self.max_height = self.max_height.max(other.max_height);
    }
}

impl Default for BoundingVolume {
    fn default() -> Self {
        Self {
            min_lng: f64::MAX,
            max_lng: f64::MIN,
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_height: f64::MAX,
            max_height: f64::MIN,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    // polygons [x, y, z, u, v]
    pub polygons: MultiPolygon<'static, [f64; 5]>,
    // material ids for each polygon
    pub polygon_material_ids: Vec<u32>,
    // materials
    pub materials: IndexSet<Material>,
    // feature_id
    pub feature_id: Option<u32>,
}

type ClassifiedFeatures = HashMap<String, ClassFeatures>;

#[derive(Default)]
pub struct ClassFeatures {
    features: Vec<Feature>,
    bounding_volume: BoundingVolume,
}

#[derive(Clone, Debug)]
pub struct VertexData {
    position: [f64; 3],
    tex_coord: [f64; 2],
    material_id: usize,
}

impl DataSink for ObjAtlasSink {
    fn make_requirements(&mut self, properties: Vec<TransformerOption>) -> DataRequirements {
        let default_requirements: DataRequirements = DataRequirements {
            resolve_appearance: true,
            key_value: crate::transformer::KeyValueSpec::JsonifyObjectsAndArrays,
            ..Default::default()
        };

        for prop in properties {
            let _ = &self
                .transform_settings
                .update_transformer(&prop.key, prop.is_enabled);
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let ellipsoid = nusamai_projection::ellipsoid::wgs84();

        let classified_features: Mutex<ClassifiedFeatures> = Default::default();

        // Construct a Feature classified by typename from Entity
        // Feature has polygons, attributes, and materials.
        // The coordinates of polygon store the actual coordinate values (WGS84) and UV coordinates, not the index.
        let _ = upstream.into_iter().par_bridge().try_for_each(|parcel| {
            feedback.ensure_not_canceled()?;

            let entity = parcel.entity;

            // entity must be a Feature
            let Value::Object(obj) = &entity.root else {
                return Ok(());
            };
            let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype else {
                return Ok(());
            };

            let geom_store = entity.geometry_store.read().unwrap();
            if geom_store.multipolygon.is_empty() {
                return Ok(());
            }
            let appearance_store = entity.appearance_store.read().unwrap();

            let mut materials: IndexSet<Material> = IndexSet::new();
            let default_material = appearance::Material::default();

            let mut feature = Feature {
                polygons: MultiPolygon::new(),
                polygon_material_ids: Default::default(),
                materials: Default::default(),
                feature_id: None, // feature_id is set later
            };

            let mut local_bvol = BoundingVolume::default();

            geometries.iter().for_each(|entry| {
                match entry.ty {
                    GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
                        // extract the polygon, material, and texture
                        for (((idx_poly, poly_uv), poly_mat), poly_tex) in
                            geom_store
                                .multipolygon
                                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
                                .zip_eq(geom_store.polygon_uvs.iter_range(
                                    entry.pos as usize..(entry.pos + entry.len) as usize,
                                ))
                                .zip_eq(
                                    geom_store.polygon_materials
                                        [entry.pos as usize..(entry.pos + entry.len) as usize]
                                        .iter(),
                                )
                                .zip_eq(
                                    geom_store.polygon_textures
                                        [entry.pos as usize..(entry.pos + entry.len) as usize]
                                        .iter(),
                                )
                        {
                            // convert to idx_poly to polygon
                            let poly = idx_poly.transform(|c| geom_store.vertices[*c as usize]);
                            let orig_mat = poly_mat
                                .and_then(|idx| appearance_store.materials.get(idx as usize))
                                .unwrap_or(&default_material)
                                .clone();
                            let orig_tex = poly_tex
                                .and_then(|idx| appearance_store.textures.get(idx as usize));

                            let mat = Material {
                                base_color: orig_mat.diffuse_color.into(),
                                base_texture: orig_tex.map(|tex| Texture {
                                    uri: tex.image_url.clone(),
                                }),
                            };

                            let (mat_idx, _) = materials.insert_full(mat);

                            let mut ring_buffer: Vec<[f64; 5]> = Vec::new();

                            poly.rings().zip_eq(poly_uv.rings()).enumerate().for_each(
                                |(ri, (ring, uv_ring))| {
                                    ring.iter_closed().zip_eq(uv_ring.iter_closed()).for_each(
                                        |(c, uv)| {
                                            let [lng, lat, height] = c;
                                            ring_buffer.push([lng, lat, height, uv[0], uv[1]]);

                                            local_bvol.min_lng = local_bvol.min_lng.min(lng);
                                            local_bvol.max_lng = local_bvol.max_lng.max(lng);
                                            local_bvol.min_lat = local_bvol.min_lat.min(lat);
                                            local_bvol.max_lat = local_bvol.max_lat.max(lat);
                                            local_bvol.min_height =
                                                local_bvol.min_height.min(height);
                                            local_bvol.max_height =
                                                local_bvol.max_height.max(height);
                                        },
                                    );
                                    if ri == 0 {
                                        feature.polygons.add_exterior(ring_buffer.drain(..));
                                        feature.polygon_material_ids.push(mat_idx as u32);
                                    } else {
                                        feature.polygons.add_interior(ring_buffer.drain(..));
                                    }
                                },
                            );
                        }
                    }
                    GeometryType::Curve => {
                        // TODO: implement
                    }
                    GeometryType::Point => {
                        // TODO: implement
                    }
                }
            });

            feature.materials = materials;

            {
                let mut locked_features = classified_features.lock().unwrap();
                let feats = locked_features.entry(obj.typename.to_string()).or_default();
                feats.features.push(feature);
                feats.bounding_volume.update(&local_bvol);
            }

            Ok::<(), PipelineError>(())
        });

        let classified_features = classified_features.into_inner().unwrap();

        // Bounding volume for the entire dataset
        let global_bvol = {
            let mut global_bvol = BoundingVolume::default();
            for features in classified_features.values() {
                global_bvol.update(&features.bounding_volume);
            }
            global_bvol
        };

        // Transformation matrix to convert geodetic coordinates to geocentric and offset to the center
        let transform_matrix = {
            let bounds = &global_bvol;
            let center_lng = (bounds.min_lng + bounds.max_lng) / 2.0;
            let center_lat = (bounds.min_lat + bounds.max_lat) / 2.0;

            let psi = ((1. - ellipsoid.e_sq()) * center_lat.to_radians().tan()).atan();

            let (tx, ty, tz) = geodetic_to_geocentric(&ellipsoid, center_lng, center_lat, 0.);
            let h = (tx * tx + ty * ty + tz * tz).sqrt();

            DMat4::from_translation(DVec3::new(0., -h, 0.))
                * DMat4::from_rotation_x(-(FRAC_PI_2 - psi))
                * DMat4::from_rotation_y((-center_lng - 90.).to_radians())
        };
        let _ = transform_matrix.inverse();

        classified_features
            .into_par_iter()
            .try_for_each(|(typename, mut features)| {
                feedback.ensure_not_canceled()?;

                // Triangulation
                let mut earcutter = Earcut::new();
                let mut buf3d: Vec<[f64; 3]> = Vec::new();
                let mut buf2d: Vec<[f64; 2]> = Vec::new();
                let mut index_buf: Vec<u32> = Vec::new();

                let mut feature_id = 0;

                let mut feature_vertex_data: Vec<(u32, Vec<VertexData>)> = Vec::new();

                for feature in features.features.iter_mut() {
                    feedback.ensure_not_canceled()?;

                    let feature_id = feature.feature_id.unwrap_or_else(|| {
                        feature_id += 1;
                        feature.feature_id = Some(feature_id);
                        feature_id
                    });

                    feature
                        .polygons
                        .transform_inplace(|&[lng, lat, height, u, v]| {
                            let (x, y, z) = geodetic_to_geocentric(&ellipsoid, lng, lat, height);
                            let v_xyz = DVec4::new(x, z, -y, 1.0);
                            let v_enu = transform_matrix * v_xyz;
                            [v_enu[0], v_enu[1], v_enu[2], u, v]
                        });

                    let mut feature_data = Vec::new();

                    for (poly, &orig_mat_id) in feature
                        .polygons
                        .iter()
                        .zip_eq(feature.polygon_material_ids.iter())
                    {
                        let num_outer = match poly.hole_indices().first() {
                            Some(&v) => v as usize,
                            None => poly.raw_coords().len(),
                        };

                        let vertex_data: Vec<VertexData> = poly
                            .raw_coords()
                            .iter()
                            .map(|&[x, y, z, u, v]| VertexData {
                                position: [x, y, z],
                                tex_coord: [u, v],
                                material_id: orig_mat_id as usize,
                            })
                            .collect();

                        buf3d.clear();
                        buf3d.extend(vertex_data.iter().map(|v| v.position));

                        if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                            earcutter.earcut(
                                buf2d.iter().cloned(),
                                poly.hole_indices(),
                                &mut index_buf,
                            );
                            feature_data.extend(
                                index_buf
                                    .iter()
                                    .map(|&idx| vertex_data[idx as usize].clone()),
                            );
                        }
                    }

                    feature_vertex_data.push((feature_id, feature_data));
                }

                feedback.ensure_not_canceled()?;

                // Write OBJ file
                let mut file_path = self.output_path.clone();
                let file_name = typename.replace(':', "_").to_string();
                file_path.push(&file_name);

                std::fs::create_dir_all(&file_path)?;

                let dir_name = file_path.to_str().unwrap();
                let obj_writer = std::fs::File::create(format!("{}/{}.obj", dir_name, file_name))?;

                write_obj(
                    feedback,
                    obj_writer,
                    features.features,
                    feature_vertex_data,
                    file_name,
                    file_path,
                    self.obj_options.is_split,
                )?;

                Ok::<(), PipelineError>(())
            })?;

        Ok(())
    }
}
