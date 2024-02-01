use nusamai_geometry::Polygon;

use crate::models::CzmlPolygon;

//
pub fn indexed_multipolygon_to_czml_polygon(
    vertices: &[[f64; 3]],
    poly_idx: &Polygon<1, u32>,
) -> CzmlPolygon {
    let mut czml_polygon = CzmlPolygon::default();

    let coords = poly_idx
        .exterior()
        .iter()
        .map(|&idx| vertices[idx as usize])
        .collect::<Vec<_>>();

    czml_polygon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut poly = Polygon::new();
    }
}
