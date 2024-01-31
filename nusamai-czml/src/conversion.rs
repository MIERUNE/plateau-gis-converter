use nusamai_geometry::{CoordNum, MultiPolygon, Polygon};

use crate::{models::CzmlPolygon, PositionListOfListsProperties, PositionListProperties};

pub fn polygon_to_czml_polygon<const D: usize, T: CoordNum>(
    polygon: &Polygon<D, T>,
) -> CzmlPolygon {
    let mut czml_polygon = CzmlPolygon::default();
    let mut positions = PositionListProperties {
        ..Default::default()
    };
    let mut holes = PositionListOfListsProperties {
        ..Default::default()
    };

    czml_polygon
}

pub fn multi_polygon_to_czml_polygon<const D: usize, T: CoordNum>(
    multi_polygon: &MultiPolygon<D, T>,
) -> CzmlPolygon {
    let mut czml_polygon = CzmlPolygon::default();
    let mut positions = PositionListProperties {
        ..Default::default()
    };
    let mut holes = PositionListOfListsProperties {
        ..Default::default()
    };

    czml_polygon
}
