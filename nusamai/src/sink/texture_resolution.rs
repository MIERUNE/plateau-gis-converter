/// Limits the texture resolution based on the distance (meters) between the vertices of a polygon.
const MAX_TEXTURE_PIXELS_PER_METER: f64 = 30.0;

// WARN: This function has an equivalent in `atlas-packer/src/texture.rs`.
fn uv_to_pixel_coords(uv_coords: &[(f64, f64)], width: u32, height: u32) -> Vec<(u32, u32)> {
    uv_coords
        .iter()
        .map(|(u, v)| {
            (
                (u.clamp(0.0, 1.0) * width as f64).min(width as f64 - 1.0) as u32,
                ((1.0 - v.clamp(0.0, 1.0)) * height as f64).min(height as f64 - 1.0) as u32,
            )
        })
        .collect()
}

pub fn get_texture_downsample_scale_of_polygon(
    vertices: &[(f64, f64, f64, f64, f64)], // (x, y, z, u, v)
    texture_size: (u32, u32),
    limit_texture_resolution: Option<bool>,
) -> f64 {
    let uv_coords = vertices.iter().map(|v| (v.3, v.4)).collect::<Vec<_>>();

    let pixel_coords = uv_to_pixel_coords(&uv_coords, texture_size.0, texture_size.1);

    let pixel_per_distance = (0..vertices.len())
        .map(|i| {
            let j = (i + 1) % vertices.len();
            let (euc0, txl0) = (
                (vertices[i].0, vertices[i].1, vertices[i].2),
                pixel_coords[i],
            );
            let (euc1, txl1) = (
                (vertices[j].0, vertices[j].1, vertices[j].2),
                pixel_coords[j],
            );
            let euc_dist =
                ((euc0.0 - euc1.0).powi(2) + (euc0.1 - euc1.1).powi(2) + (euc0.2 - euc1.2).powi(2))
                    .sqrt();
            let txl_dist = ((txl0.0 as f64 - txl1.0 as f64).powi(2)
                + (txl0.1 as f64 - txl1.1 as f64).powi(2))
            .sqrt();
            txl_dist / euc_dist
        })
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or(1.0);

    if limit_texture_resolution.unwrap_or(false) {
        1.0_f64.min(MAX_TEXTURE_PIXELS_PER_METER / pixel_per_distance)
    } else {
        1.0
    }
}
