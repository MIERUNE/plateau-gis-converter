use std::path::Path;

use hashbrown::HashMap;
use rayon::prelude::*;

use crate::disjoint_set::DisjointSet;
use crate::export::AtlasExporter;
use crate::place::{PlacedTextureGeometry, PlacedUVPolygon, TexturePlacer};
use crate::texture::cache::TextureCache;
use crate::texture::{ChildUVPolygon, ClusterBoundingTexture, PolygonMappedTexture};
use crate::{AtlasID, ClusterID, PolygonID};
pub type Atlas = Vec<PlacedTextureGeometry>;

pub struct AtlasPacker {
    textures: HashMap<PolygonID, PolygonMappedTexture>,
}

impl Default for AtlasPacker {
    fn default() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub(super) struct Cluster {
    pub bounding_texture: ClusterBoundingTexture,
    pub uv_polygons: Vec<(PolygonID, ChildUVPolygon)>,
}

impl AtlasPacker {
    pub fn add_texture(&mut self, polygon_id: PolygonID, texture: PolygonMappedTexture) {
        self.textures.insert(polygon_id, texture);
    }

    fn create_clusters(&self) -> HashMap<ClusterID, Cluster> {
        let polygon_ids: Vec<PolygonID> = self.textures.keys().cloned().collect();

        let disjoint_set = {
            println!("Creating disjoint set");
            let start = std::time::Instant::now();
            let mut disjoint_set = DisjointSet::new(polygon_ids.len());

            for i in 0..polygon_ids.len() {
                for j in (i + 1)..polygon_ids.len() {
                    let texture_i = self.textures.get(&polygon_ids[i]).unwrap();
                    let texture_j = self.textures.get(&polygon_ids[j]).unwrap();

                    if texture_i.bbox_overlaps(texture_j) {
                        disjoint_set.unite(i, j);
                    }
                    println!("{} / {}", i, polygon_ids.len());
                }
            }
            disjoint_set.compress();
            println!("Disjoint set created in {:?}", start.elapsed());
            disjoint_set
        };

        let clustered_polygon_ids: HashMap<ClusterID, Vec<PolygonID>> = {
            let mut clustered_polygon_ids = HashMap::new();
            for (i, polygon_id) in polygon_ids.iter().enumerate() {
                let cluster_id = disjoint_set.root(i).to_string();
                clustered_polygon_ids
                    .entry(cluster_id)
                    .or_insert_with(Vec::new)
                    .push(polygon_id.clone());
            }
            clustered_polygon_ids
        };

        let cluster_map: HashMap<ClusterID, Cluster> = clustered_polygon_ids
            .iter()
            .filter_map(|(cluster_id, polygon_ids)| {
                let bounding_texture = polygon_ids.iter().fold(
                    None,
                    |acc: Option<ClusterBoundingTexture>, polygon_id| {
                        let texture = self.textures.get(polygon_id).unwrap();
                        match acc {
                            Some(bounding_texture) => bounding_texture.expand(texture),
                            None => Some(ClusterBoundingTexture::new(texture)),
                        }
                    },
                )?;

                let uv_polygons = polygon_ids
                    .iter()
                    .map(|polygon_id| {
                        let texture = self.textures.get(polygon_id).unwrap();
                        (polygon_id.clone(), bounding_texture.get_child(texture))
                    })
                    .collect::<Vec<(PolygonID, ChildUVPolygon)>>();

                Some((
                    cluster_id.clone(),
                    Cluster {
                        bounding_texture,
                        uv_polygons,
                    },
                ))
            })
            .collect::<HashMap<_, _>>();

        cluster_map
    }

    pub fn pack<P: TexturePlacer>(self, mut placer: P) -> PackedAtlasProvider {
        let mut current_atlas: Atlas = Vec::new();
        let mut atlases: HashMap<AtlasID, Atlas> = HashMap::new();

        let clusters = self.create_clusters();
        let mut placed_uv_polygon_map: HashMap<PolygonID, PlacedUVPolygon> = HashMap::new();
        for (cluster_id, cluster) in clusters.iter() {
            if !placer.can_place(&cluster.bounding_texture) {
                let current_atlas_id = atlases.len();
                atlases.insert(current_atlas_id, current_atlas.clone());
                current_atlas.clear();
                placer.reset_param();
            }

            let current_atlas_id = atlases.len();

            let (placed_texture, placed_uv_polygons) = placer.place_texture(
                cluster.bounding_texture.clone(),
                cluster.uv_polygons.clone(),
                cluster_id.clone(),
                current_atlas_id,
            );

            current_atlas.push(placed_texture.clone());

            let polygon_ids = cluster
                .uv_polygons
                .iter()
                .map(|(id, _)| id)
                .collect::<Vec<_>>();

            for (polygon_id, placed_uv_polygon) in polygon_ids.iter().zip(placed_uv_polygons) {
                if let Some(placed_uv_polygon) = placed_uv_polygon {
                    placed_uv_polygon_map.insert((*polygon_id).clone(), placed_uv_polygon.clone());
                }
            }
        }

        // treat the last atlas
        if !current_atlas.is_empty() {
            let current_atlas_id = atlases.len();

            atlases.insert(current_atlas_id, current_atlas.clone());
            current_atlas.clear();
        }

        PackedAtlasProvider {
            clusters,
            atlases,
            placed_uv_polygon_map,
        }
    }
}

pub struct PackedAtlasProvider {
    atlases: HashMap<AtlasID, Atlas>,
    clusters: HashMap<ClusterID, Cluster>,
    placed_uv_polygon_map: HashMap<PolygonID, PlacedUVPolygon>,
}

impl PackedAtlasProvider {
    pub fn export<E: AtlasExporter>(
        &self,
        exporter: E,
        output_dir: &Path,
        texture_cache: &TextureCache,
        width: u32,
        height: u32,
    ) {
        self.atlases.par_iter().for_each(|(id, atlas)| {
            let output_path = output_dir.join(id.to_string());
            exporter.export(
                atlas,
                &self
                    .clusters
                    .iter()
                    .map(|(id, cluster)| (id.clone(), cluster.bounding_texture.clone()))
                    .collect::<HashMap<ClusterID, ClusterBoundingTexture>>(),
                &output_path,
                texture_cache,
                width,
                height,
            );
        });
    }

    pub fn get_texture_info(&self, polygon_id: &PolygonID) -> Option<&PlacedUVPolygon> {
        self.placed_uv_polygon_map.get(polygon_id)
    }
}
