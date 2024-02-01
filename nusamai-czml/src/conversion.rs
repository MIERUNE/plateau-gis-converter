use nusamai_geometry::Polygon;

use crate::models::CzmlPolygon;

pub fn multi_polygon_to_czml_polygon(
    vertices: &[[f64; 3]],
    poly_idx: &Polygon<1, u32>,
) -> CzmlPolygon {
    let mut czml_polygon = CzmlPolygon::default();

    czml_polygon
}
