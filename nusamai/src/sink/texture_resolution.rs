/// Limits the texture resolution based on the distance (in meters) between the vertices of the polygon.
/// The resolution of aerial photographs is usually between 10cm and 20cm.
/// The pixel resolution should be limited to around 10cm (0.1m),
/// but this means that signs and other objects will not be visible, so adjustments are necessary.
const MIN_METER_PER_PIXEL: f64 = 0.025;

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

fn get_distance_par_pixel(vertices: &[(f64, f64, f64)], pixel_coords: &[(u32, u32)]) -> f64 {
    let mut valid_scales = Vec::new();
    let epsilon = 1e-6;

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        let (euc0, txl0) = (vertices[i], pixel_coords[i]);
        let (euc1, txl1) = (vertices[j], pixel_coords[j]);

        // 3D Euclidean distance
        let euc_dist =
            ((euc0.0 - euc1.0).powi(2) + (euc0.1 - euc1.1).powi(2) + (euc0.2 - euc1.2).powi(2))
                .sqrt();

        // 2D pixel distance
        let txl_dist = ((txl0.0 as f64 - txl1.0 as f64).powi(2)
            + (txl0.1 as f64 - txl1.1 as f64).powi(2))
        .sqrt();

        if txl_dist > epsilon && euc_dist.is_finite() {
            let scale = euc_dist / txl_dist;
            if scale.is_finite() && scale > 0.0 {
                valid_scales.push(scale);
            }
        }
    }

    let avg_scale = valid_scales.iter().sum::<f64>() / valid_scales.len() as f64;
    avg_scale
}

/// Obtain the downsample scale to limit the distance per pixel to a specific value or less.
pub fn get_texture_downsample_scale_of_polygon(
    vertices: &[(f64, f64, f64, f64, f64)], // (x, y, z, u, v)
    texture_size: (u32, u32),
) -> f64 {
    let uv_coords = vertices.iter().map(|v| (v.3, v.4)).collect::<Vec<_>>();
    let pixel_coords = uv_to_pixel_coords(&uv_coords, texture_size.0, texture_size.1);
    let vertices = vertices.iter().map(|v| (v.0, v.1, v.2)).collect::<Vec<_>>();
    let pixel_per_distance = get_distance_par_pixel(&vertices, &pixel_coords);

    if pixel_per_distance < MIN_METER_PER_PIXEL {
        1.0 / (MIN_METER_PER_PIXEL / pixel_per_distance)
    } else {
        1.0
    }
}

/// A downsample scale is applied, taking into account geometric error and distance per pixel.
/// The downsample scale is a value between 0.0 and 1.0.
pub fn apply_downsample_factor(geometric_error: f64, downsample_scale: f32) -> f32 {
    let error_factor = if geometric_error == 0.0 {
        1.0
    } else {
        // Applying a scale factor increases the distance per pixel and decreases the texture resolution.
        // The resolution of textures generated from aerial photographs is typically 10~20 cm.
        // When the geometric error is 0, the scale approaches 1.
        let error_factor = 1.0 / (1.0 + (geometric_error / 15.0).powf(0.85));
        if error_factor.is_nan() {
            1.0
        } else {
            error_factor
        }
    };

    (error_factor * downsample_scale as f64).clamp(0.0, 1.0) as f32
}
