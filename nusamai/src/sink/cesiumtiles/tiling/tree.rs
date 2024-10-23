//! Tileset encoder

use cesiumtiles::tileset;
use tinymvt::TileZXY;

use super::scheme::{calc_parent_zxy, geometric_error};

#[derive(Debug)]
pub struct TileContent {
    pub zxy: TileZXY,
    pub content_path: String,
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Default for TileContent {
    fn default() -> Self {
        TileContent {
            zxy: (0, u32::MAX, u32::MAX),
            content_path: String::new(),
            min_lng: f64::MAX,
            max_lng: f64::MIN,
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_height: f64::MAX,
            max_height: f64::MIN,
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    zxy: TileZXY,
    contents: Vec<TileContent>,
    child00: Option<Box<Tile>>,
    child01: Option<Box<Tile>>,
    child10: Option<Box<Tile>>,
    child11: Option<Box<Tile>>,
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            zxy: (0, u32::MAX, u32::MAX),
            child00: None,
            child01: None,
            child10: None,
            child11: None,
            contents: vec![],
            min_lng: f64::MAX,
            max_lng: f64::MIN,
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_height: f64::MAX,
            max_height: f64::MIN,
        }
    }
}

impl Tile {
    fn update_boundary(&mut self) {
        for child in [
            &mut self.child00,
            &mut self.child01,
            &mut self.child10,
            &mut self.child11,
        ]
        .into_iter()
        .flatten()
        {
            child.update_boundary();
            self.min_lng = self.min_lng.min(child.min_lng);
            self.max_lng = self.max_lng.max(child.max_lng);
            self.min_lat = self.min_lat.min(child.min_lat);
            self.max_lat = self.max_lat.max(child.max_lat);
            self.min_height = self.min_height.min(child.min_height);
            self.max_height = self.max_height.max(child.max_height);
        }
        for content in &self.contents {
            self.min_lng = self.min_lng.min(content.min_lng);
            self.max_lng = self.max_lng.max(content.max_lng);
            self.min_lat = self.min_lat.min(content.min_lat);
            self.max_lat = self.max_lat.max(content.max_lat);
            self.min_height = self.min_height.min(content.min_height);
            self.max_height = self.max_height.max(content.max_height);
        }
    }

    fn into_tileset_tile(mut self) -> tileset::Tile {
        self.update_boundary();

        let children = {
            let children: Vec<_> = [self.child00, self.child01, self.child10, self.child11]
                .into_iter()
                .flatten()
                .map(|child| child.into_tileset_tile())
                .collect();
            if children.is_empty() {
                None
            } else {
                Some(children)
            }
        };

        let (content, contents) = {
            match self.contents.len() {
                0 => (None, None),
                1 => {
                    let content = tileset::Content {
                        uri: self.contents[0].content_path.clone(),
                        ..Default::default()
                    };
                    (Some(content), None)
                }
                _ => {
                    let contents: Vec<_> = self
                        .contents
                        .into_iter()
                        .map(|content| tileset::Content {
                            uri: content.content_path,
                            ..Default::default()
                        })
                        .collect();
                    (None, Some(contents))
                }
            }
        };

        let (z, _, y) = self.zxy;
        tileset::Tile {
            geometric_error: geometric_error(z, y),
            refine: Some(tileset::Refine::Replace),
            bounding_volume: tileset::BoundingVolume::new_region([
                self.min_lng.to_radians(),
                self.min_lat.to_radians(),
                self.max_lng.to_radians(),
                self.max_lat.to_radians(),
                self.min_height,
                self.max_height,
            ]),
            content,
            contents,
            children,
            ..Default::default()
        }
    }

    fn new(zxy: TileZXY) -> Self {
        Tile {
            zxy,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct TileTree {
    root: Tile,
}

impl Default for TileTree {
    fn default() -> Self {
        Self {
            root: Tile {
                zxy: (0, 0, 0),
                ..Default::default()
            },
        }
    }
}

impl TileTree {
    pub fn into_tileset_root(self) -> tileset::Tile {
        self.root.into_tileset_tile()
    }

    pub fn add_content(&mut self, content: TileContent) {
        let node = self.get_node(content.zxy);
        node.contents.push(content);
    }

    fn get_node(&mut self, zxy: TileZXY) -> &mut Tile {
        let (zoom, x, y) = zxy;
        if zoom == 0 {
            &mut self.root
        } else {
            let parent = self.get_node(calc_parent_zxy(zoom, x, y));
            let node = match (x % 2, y % 2) {
                (0, 0) => parent.child00.get_or_insert_with(|| Tile::new(zxy).into()),
                (0, 1) => parent.child01.get_or_insert_with(|| Tile::new(zxy).into()),
                (1, 0) => parent.child10.get_or_insert_with(|| Tile::new(zxy).into()),
                (1, 1) => parent.child11.get_or_insert_with(|| Tile::new(zxy).into()),
                _ => unreachable!(),
            };
            node
        }
    }
}
