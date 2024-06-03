use flatgeom::MultiPolygon;

pub struct Bbox {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Default for Bbox {
    fn default() -> Self {
        Bbox {
            min_x: f64::MAX,
            min_y: f64::MAX,
            max_x: f64::MIN,
            max_y: f64::MIN,
        }
    }
}

impl Bbox {
    /// To a tuple (min_x, min_y, max_x, max_y)
    pub fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.min_x, self.min_y, self.max_x, self.max_y)
    }

    /// Update the bounding box with a new point
    pub fn update(&mut self, x: f64, y: f64) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    /// Merge with another bounding box
    pub fn merge(&mut self, other: &Bbox) {
        self.min_x = self.min_x.min(other.min_x);
        self.min_y = self.min_y.min(other.min_y);
        self.max_x = self.max_x.max(other.max_x);
        self.max_y = self.max_y.max(other.max_y);
    }
}

// Get Bounding box of a MultiPolygon
pub fn get_indexed_multipolygon_bbox(vertices: &[[f64; 3]], mpoly: &MultiPolygon<u32>) -> Bbox {
    let mut bbox: Bbox = Default::default();

    for poly in mpoly {
        for point_idx in &poly.exterior() {
            let [x, y, _z] = vertices[point_idx as usize];
            bbox.update(x, y);
        }
    }
    bbox
}

#[cfg(test)]
mod tests {
    use nusamai_projection::crs::EPSG_JGD2011_GEOGRAPHIC_3D;

    use super::*;

    #[test]
    fn test_update() {
        let mut bbox = Bbox::default();
        bbox.update(0.5, 0.5);
        assert_eq!(bbox.to_tuple(), (0.5, 0.5, 0.5, 0.5));

        bbox.update(1.0, 1.0);
        assert_eq!(bbox.to_tuple(), (0.5, 0.5, 1.0, 1.0));

        bbox.update(-1.0, -1.0);
        assert_eq!(bbox.to_tuple(), (-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn test_merge() {
        let mut bbox1 = Bbox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 1.0,
            max_y: 1.0,
        };
        let bbox2 = Bbox {
            min_x: 1.0,
            min_y: 1.0,
            max_x: 2.0,
            max_y: 2.0,
        };

        bbox1.merge(&bbox2);

        assert_eq!(bbox1.min_x, 0.0);
        assert_eq!(bbox1.min_y, 0.0);
        assert_eq!(bbox1.max_x, 2.0);
        assert_eq!(bbox1.max_y, 2.0);
    }

    #[test]
    fn test_get_indexed_multipolygon_bbox() {
        let vertices: Vec<[f64; 3]> = vec![
            [10., 100., 111.],
            [10., 200., 111.],
            [20., 200., 111.],
            [20., 100., 111.],
        ];
        let mut mpoly = MultiPolygon::<u32>::new();
        mpoly.add_exterior([0, 1, 2, 3, 0]);
        let geometries = nusamai_citygml::GeometryStore {
            epsg: EPSG_JGD2011_GEOGRAPHIC_3D,
            vertices,
            multipolygon: mpoly,
            ..Default::default()
        };

        let bbox = get_indexed_multipolygon_bbox(&geometries.vertices, &geometries.multipolygon);

        assert_eq!(bbox.to_tuple(), (10., 100., 20., 200.));
    }
}
