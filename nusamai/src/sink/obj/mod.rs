//! obj sink
use std::{any::Any, f64::consts::FRAC_PI_2, io::Write, path::PathBuf, sync::Mutex};
mod material;

use ahash::{HashMap, HashSet, RandomState};
use earcut::{utils3d::project3d_to_2d, Earcut};
use flatgeom::{MultiPolygon, Polygon};
use glam::{DMat4, DVec3, DVec4};
use indexmap::IndexSet;
use itertools::Itertools;
use material::{load_image, Material, Texture};
use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};
use nusamai_plateau::appearance;
use nusamai_projection::cartesian::geodetic_to_geocentric;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{cesiumtiles::metadata, DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
    transformer::{TransformerConfig, TransformerOption, TransformerRegistry},
};

pub struct ObjSinkProvider {}

impl DataSinkProvider for ObjSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "obj".to_string(),
            name: "OBJ".to_string(),
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

        Box::<ObjSink>::new(ObjSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings: transform_options,
        })
    }
}

pub struct ObjSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
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
    // attribute values
    pub attributes: nusamai_citygml::object::Value,
    // feature_id
    pub feature_id: Option<u32>,
    // type_id
    pub type_id: Option<String>,
}

type ClassifiedFeatures = HashMap<String, ClassFeatures>;

#[derive(Default)]
struct ClassFeatures {
    features: Vec<Feature>,
    bounding_volume: BoundingVolume,
}
#[derive(Default)]
pub struct PrimitiveInfo {
    pub indices: Vec<u32>,
    pub feature_ids: HashSet<u32>,
}

pub type Primitives = HashMap<material::Material, PrimitiveInfo>;

// 頂点とテクスチャ座標を一緒に保持する構造体
#[derive(Clone, Debug)]
// VertexDataの構造体を拡張
struct VertexData {
    position: [f64; 3],
    tex_coord: [f64; 2],
    material_id: usize,
    type_id: Option<String>,
    tex_count: usize,
}

impl DataSink for ObjSink {
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
        // Features have polygons, attributes and materials
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
                attributes: entity.root.clone(),
                polygon_material_ids: Default::default(),
                materials: Default::default(),
                feature_id: None, // feature_id is set later
                type_id: obj.stereotype.id().map(|id| id.to_string()),
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

                            // let poly_uv = poly_uv.raw_coords();
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

                    let type_id = feature.type_id.as_ref().unwrap_or(&typename);

                    feature
                        .polygons
                        .transform_inplace(|&[lng, lat, height, u, v]| {
                            let (x, y, z) = geodetic_to_geocentric(&ellipsoid, lng, lat, height);
                            let v_xyz = DVec4::new(x, z, -y, 1.0);
                            let v_enu = transform_matrix * v_xyz;
                            [v_enu[0], v_enu[1], v_enu[2], u, 1.0 - v]
                        });

                    let mut feature_data = Vec::new();

                    let tex_count = &feature
                        .materials
                        .iter()
                        .filter(|m| m.base_texture.is_some())
                        .count();

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
                                type_id: Some(type_id.clone()),
                                tex_count: *tex_count,
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

                // file_path.push(format!("{}_OBJ", typename.replace(':', "_")));
                file_path.push(format!("{}_OBJ", "")); // NOTE: debug

                std::fs::create_dir_all(&file_path)?;

                let dir_name = file_path.to_str().unwrap();
                let mut obj_writer = std::fs::File::create(format!(
                    "{}/{}.obj",
                    dir_name,
                    typename.replace(':', "_")
                ))?;

                // Write MTL file
                let mut mtl_writer = std::fs::File::create(format!(
                    "{}/{}.mtl",
                    dir_name,
                    typename.replace(':', "_")
                ))?;

                // Material
                writeln!(obj_writer, "mtllib {}.mtl", typename.replace(':', "_"))?;

                let mut global_vertex_offset = 0;
                let mut global_texture_offset = 0;

                // Default material flag
                let mut default_material_written = false;

                // OBJファイル書き込み部分
                for (feature_id, feature_data) in &feature_vertex_data {
                    let type_id = feature_data.first().unwrap().type_id.as_ref().unwrap();
                    // if "bldg_d05b0b65-eabf-473b-ab1c-cd9245aa3437" == type_id {
                    // if "bldg_51ed9798-bea2-4217-8389-e065e3586e61" == type_id {
                    if true {
                        writeln!(obj_writer, "o {}", type_id)?;

                        // 頂点とテクスチャ座標の書き込み
                        for vertex in feature_data {
                            writeln!(
                                obj_writer,
                                "v {} {} {}",
                                vertex.position[0], vertex.position[1], vertex.position[2]
                            )?;
                        }

                        for vertex in feature_data {
                            writeln!(
                                obj_writer,
                                "vt {} {}",
                                vertex.tex_coord[0],
                                1.0 - vertex.tex_coord[1]
                            )?;
                        }

                        // UV座標が[0.0, 1.0]でない場合のみテクスチャ座標の書き込み
                        let mut write_texture_indices = vec![true; feature_data.len()];
                        for (i, vertex) in feature_data.iter().enumerate() {
                            if vertex.tex_coord == [0.0, 1.0] {
                                write_texture_indices[i] = false;
                            } else {
                                // writeln!(
                                //     obj_writer,
                                //     "vt {} {}",
                                //     vertex.tex_coord[0],
                                //     1.0 - vertex.tex_coord[1]
                                // )?;
                            }
                        }

                        // テクスチャとマテリアル情報のキャッシュ
                        let mut texture_cache: std::collections::HashMap<String, Vec<u8>> =
                            std::collections::HashMap::new();
                        let mut material_written: std::collections::HashMap<(u32, usize), bool> =
                            std::collections::HashMap::new();

                        // マテリアルIDごとにフェイスをグループ化
                        let mut faces_by_material: std::collections::HashMap<
                            usize,
                            Vec<(usize, &VertexData)>,
                        > = std::collections::HashMap::new();
                        for (i, vertex) in feature_data.iter().enumerate() {
                            faces_by_material
                                .entry(vertex.material_id)
                                .or_insert_with(Vec::new)
                                .push((i, vertex));
                        }

                        for (material_id, faces) in faces_by_material.iter() {
                            let feature = features
                                .features
                                .iter()
                                .find(|f| f.feature_id == Some(*feature_id))
                                .unwrap();
                            let mat = &feature.materials[*material_id];

                            if let Some(Texture { uri }) = &mat.base_texture {
                                if let Ok(path) = uri.to_file_path() {
                                    let image_file_name = format!(
                                        "Feature_{}_Material_{}.jpg",
                                        feature_id, material_id
                                    );

                                    // テクスチャがキャッシュにない場合のみ読み込む
                                    if !texture_cache.contains_key(&image_file_name) {
                                        let content = load_image(feedback, &path)?;
                                        texture_cache.insert(image_file_name.clone(), content);

                                        let textures_dir = self
                                            .output_path
                                            .join(format!("{}_OBJ", ""))
                                            .join("textures");
                                        std::fs::create_dir_all(&textures_dir)?;

                                        let image_path = textures_dir.join(&image_file_name);
                                        std::fs::write(
                                            &image_path,
                                            texture_cache.get(&image_file_name).unwrap(),
                                        )?;
                                    }

                                    // マテリアル情報が未書き込みの場合のみMTLファイルに書き込む
                                    if !material_written.contains_key(&(*feature_id, *material_id))
                                    {
                                        writeln!(
                                            mtl_writer,
                                            "newmtl Material_{}_{}",
                                            feature_id, material_id
                                        )?;
                                        writeln!(
                                            mtl_writer,
                                            "map_Kd .\\textures\\{}",
                                            image_file_name
                                        )?;
                                        material_written.insert((*feature_id, *material_id), true);
                                    }

                                    writeln!(
                                        obj_writer,
                                        "usemtl Material_{}_{}",
                                        feature_id, material_id
                                    )?;
                                }
                            } else {
                                // デフォルトマテリアルが未書き込みの場合のみMTLファイルに書き込む
                                if !default_material_written {
                                    writeln!(mtl_writer, "newmtl Default_Material")?;
                                    writeln!(mtl_writer, "Ka 1.000 1.000 1.000")?;
                                    writeln!(mtl_writer, "Kd 1.000 1.000 1.000")?;
                                    writeln!(mtl_writer, "Ks 1.000 1.000 1.000")?;
                                    default_material_written = true;
                                }

                                writeln!(obj_writer, "usemtl Default_Material")?;
                            }

                            for (i, _) in faces {
                                if i % 3 == 0 {
                                    // 三角形の頂点のうち、テクスチャ座標を書き込んだかを確認
                                    if write_texture_indices[*i]
                                        && write_texture_indices[i + 1]
                                        && write_texture_indices[i + 2]
                                    {
                                        writeln!(
                                            obj_writer,
                                            "f {}/{} {}/{} {}/{}",
                                            global_vertex_offset + i + 1,
                                            global_texture_offset + i + 1,
                                            global_vertex_offset + i + 2,
                                            global_texture_offset + i + 2,
                                            global_vertex_offset + i + 3,
                                            global_texture_offset + i + 3
                                        )?;
                                    } else {
                                        println!("UV座標が[0.0, 1.0]の三角形があります");
                                        writeln!(
                                            obj_writer,
                                            "f {} {} {}",
                                            global_vertex_offset + i + 1,
                                            global_vertex_offset + i + 2,
                                            global_vertex_offset + i + 3
                                        )?;
                                    }
                                }
                            }
                        }

                        writeln!(
                            obj_writer,
                            "# ======================================================",
                        )?;

                        global_vertex_offset += feature_data.len();
                        global_texture_offset += feature_data.len();
                    }
                }

                obj_writer.flush()?;
                mtl_writer.flush()?;

                Ok::<(), PipelineError>(())
            })?;

        Ok(())
    }
}
