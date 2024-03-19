#[inline]
fn cross((ax, ay, az): (f64, f64, f64), (bx, by, bz): (f64, f64, f64)) -> (f64, f64, f64) {
    (ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
}

pub fn calculate_normal(vertices: &[f64], dim: usize) -> Option<(f64, f64, f64)> {
    let len = vertices.len();
    if len < dim * 3 {
        // At least 3 vertices required
        return None;
    }
    let last_point = (
        vertices[len - dim],
        vertices[len - dim + 1],
        vertices[len - dim + 2],
    );

    let (sum, _) =
        vertices
            .chunks_exact(dim)
            .fold(((0., 0., 0.), last_point), |(acc, prev), data| {
                let (x, y, z) = (data[0], data[1], data[2]);
                let c = cross(
                    (prev.0 - x, prev.1 - y, prev.2 - z),
                    (prev.0 + x, prev.1 + y, prev.2 + z),
                );
                ((acc.0 + c.0, acc.1 + c.1, acc.2 + c.2), (x, y, z))
            });

    match (sum.0 * sum.0 + sum.1 * sum.1 + sum.2 * sum.2).sqrt() {
        d if d < 1e-30 => None,
        d => Some((sum.0 / d, sum.1 / d, sum.2 / d)),
    }
}
