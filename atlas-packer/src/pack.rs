use std::path::Path;

use hashbrown::HashMap;
use rayon::prelude::*;
use rstar::{RTree, RTreeObject, AABB};

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

struct Rectangle {
    index: usize,
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

impl RTreeObject for Rectangle {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.min_x, self.min_y], [self.max_x, self.max_y])
    }
}

impl AtlasPacker {
    pub fn add_texture(&mut self, polygon_id: PolygonID, texture: PolygonMappedTexture) {
        self.textures.insert(polygon_id, texture);
    }

    fn create_clusters(&self) -> HashMap<ClusterID, Cluster> {
        let polygon_ids: Vec<PolygonID> = self.textures.keys().cloned().collect();

        let mut rtree = RTree::new();
        let mut disjoint_set = DisjointSet::new(polygon_ids.len());

        for (i, polygon_id) in polygon_ids.iter().enumerate() {
            let texture = self.textures.get(polygon_id).unwrap();
            let (min_x, min_y, max_x, max_y) = texture.bbox();
            let texture_with_index = Rectangle {
                index: i,
                min_x: min_x as f32,
                min_y: min_y as f32,
                max_x: max_x as f32,
                max_y: max_y as f32,
            };
            rtree.insert(texture_with_index);
        }
        for (i, polygon_id) in polygon_ids.iter().enumerate() {
            let texture = self.textures.get(polygon_id).unwrap();
            let bbox = AABB::from_corners(
                [texture.bbox().0 as f32, texture.bbox().1 as f32],
                [texture.bbox().2 as f32, texture.bbox().3 as f32],
            );

            // Only items with the same texture and overlapping areas will be searched
            let hit = rtree
                .locate_in_envelope_intersecting(&bbox)
                .filter(|target| {
                    let target_texture = self.textures.get(&polygon_ids[target.index]).unwrap();
                    texture.image_path == target_texture.image_path
                });

            for j in hit {
                if i < j.index {
                    disjoint_set.unite(i, j.index);
                }
            }
        }
        disjoint_set.compress();

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
