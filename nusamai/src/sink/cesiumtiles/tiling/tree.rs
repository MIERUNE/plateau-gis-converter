use super::scheme::{calc_parent_zxy, geometric_error};
use nusamai_3dtiles_json::tileset;
use nusamai_mvt::TileZXY;

#[derive(Debug)]
pub struct TileSpec {
    pub zxy: TileZXY,
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Default for TileSpec {
    fn default() -> Self {
        TileSpec {
            zxy: (0, u32::MAX, u32::MAX),
            min_lng: f64::MAX,
            max_lng: f64::MIN,
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_height: f64::MAX,
            max_height: f64::MIN,
        }
    }
}

#[derive(Default, Debug)]
pub struct Tile {
    spec: TileSpec,
    has_content: bool,
    child00: Option<Box<Tile>>,
    child01: Option<Box<Tile>>,
    child10: Option<Box<Tile>>,
    child11: Option<Box<Tile>>,
}

impl Tile {
    fn update_boundary(&mut self) {
        for c in [
            &mut self.child00,
            &mut self.child01,
            &mut self.child10,
            &mut self.child11,
        ]
        .into_iter()
        .flatten()
        {
            c.update_boundary();
            self.spec.min_lng = self.spec.min_lng.min(c.spec.min_lng);
            self.spec.max_lng = self.spec.max_lng.max(c.spec.max_lng);
            self.spec.min_lat = self.spec.min_lat.min(c.spec.min_lat);
            self.spec.max_lat = self.spec.max_lat.max(c.spec.max_lat);
            self.spec.min_height = self.spec.min_height.min(c.spec.min_height);
            self.spec.max_height = self.spec.max_height.max(c.spec.max_height);
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

        let (z, x, y) = self.spec.zxy;
        let content = if self.has_content {
            Some(tileset::Content {
                uri: format!("{}/{}/{}.glb", z, x, y),
                ..Default::default()
            })
        } else {
            None
        };

        tileset::Tile {
            geometric_error: geometric_error(z, y),
            refine: Some(tileset::Refine::Replace),
            bounding_volume: tileset::BoundingVolume::new_region([
                self.spec.min_lng.to_radians(),
                self.spec.min_lat.to_radians(),
                self.spec.max_lng.to_radians(),
                self.spec.max_lat.to_radians(),
                self.spec.min_height,
                self.spec.max_height,
            ]),
            content,
            children,
            ..Default::default()
        }
    }

    fn new(zxy: TileZXY) -> Self {
        Tile {
            spec: TileSpec {
                zxy,
                ..Default::default()
            },
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
                spec: TileSpec {
                    zxy: (0, 0, 0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

impl TileTree {
    pub fn into_tileset_root(self) -> tileset::Tile {
        self.root.into_tileset_tile()
    }

    pub fn add_node(&mut self, tilespec: TileSpec) {
        let node = self.get_node(tilespec.zxy);
        node.has_content = true;
        node.spec = tilespec;
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
