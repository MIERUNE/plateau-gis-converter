//! Direct conversion from projected Web Mercator metres to XYZ tile coordinates.

const WEB_MERCATOR_WORLD_CIRCUMFERENCE_METERS: f64 = 2.0 * std::f64::consts::PI * 6_378_137.0;

/// Returns zoom-level global tile coordinates. The integer part identifies the
/// XYZ tile and the fractional part identifies the position within that tile.
pub(super) fn web_mercator_to_tile_coordinates(easting: f64, northing: f64, zoom: u8) -> [f64; 2] {
    let tile_count = 2_f64.powi(i32::from(zoom));
    let tile_width = WEB_MERCATOR_WORLD_CIRCUMFERENCE_METERS / tile_count;
    let half_world = WEB_MERCATOR_WORLD_CIRCUMFERENCE_METERS / 2.0;

    [
        (easting + half_world) / tile_width,
        (half_world - northing) / tile_width,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_zoom_level_tile_coordinates_directly_from_web_mercator_meters() {
        let half_world = WEB_MERCATOR_WORLD_CIRCUMFERENCE_METERS / 2.0;

        assert_eq!(
            web_mercator_to_tile_coordinates(-half_world, half_world, 0),
            [0.0, 0.0]
        );
        assert_eq!(web_mercator_to_tile_coordinates(0.0, 0.0, 2), [2.0, 2.0]);
        assert_eq!(
            web_mercator_to_tile_coordinates(
                WEB_MERCATOR_WORLD_CIRCUMFERENCE_METERS / 16.0,
                WEB_MERCATOR_WORLD_CIRCUMFERENCE_METERS / 32.0,
                2,
            ),
            [2.25, 1.875]
        );
        assert_eq!(
            web_mercator_to_tile_coordinates(
                WEB_MERCATOR_WORLD_CIRCUMFERENCE_METERS / 4.0,
                WEB_MERCATOR_WORLD_CIRCUMFERENCE_METERS / 4.0,
                2,
            ),
            [3.0, 1.0]
        );
    }
}
