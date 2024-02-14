/// Bounding box
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

    /// Update the bounding box with a new point and return the updated bounding box
    #[must_use]
    pub fn updated_with(&mut self, x: f64, y: f64) -> Self {
        Bbox {
            min_x: self.min_x.min(x),
            min_y: self.min_y.min(y),
            max_x: self.max_x.max(x),
            max_y: self.max_y.max(y),
        }
    }

    /// Merge with another bounding box and return the merged bounding box
    #[must_use]
    pub fn merged_with(&self, other: &Bbox) -> Bbox {
        Bbox {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updated_with() {
        let mut bbox = Bbox::default();
        bbox = bbox.updated_with(0.5, 0.5);
        assert_eq!(bbox.to_tuple(), (0.5, 0.5, 0.5, 0.5));

        bbox = bbox.updated_with(1.0, 1.0);
        assert_eq!(bbox.to_tuple(), (0.5, 0.5, 1.0, 1.0));

        bbox = bbox.updated_with(-1.0, -1.0);
        assert_eq!(bbox.to_tuple(), (-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn test_merged_with() {
        let bbox1 = Bbox {
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

        let merged = bbox1.merged_with(&bbox2);
        assert_eq!(merged.min_x, 0.0);
        assert_eq!(merged.min_y, 0.0);
        assert_eq!(merged.max_x, 2.0);
        assert_eq!(merged.max_y, 2.0);
    }
}
