#[inline]
fn cross((ax, ay, az): (f64, f64, f64), (bx, by, bz): (f64, f64, f64)) -> (f64, f64, f64) {
    (ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
}

pub fn calculate_normal(
    vertex_iter: impl IntoIterator<Item = [f64; 3]>,
) -> Option<(f64, f64, f64)> {
    let mut iter = vertex_iter.into_iter();
    let first = iter.next()?;
    let mut prev = first;

    let mut sum = (0., 0., 0.);

    for data in iter {
        // ..
        let (x, y, z) = (data[0], data[1], data[2]);
        let c = cross(
            (prev[0] - x, prev[1] - y, prev[2] - z),
            (prev[0] + x, prev[1] + y, prev[2] + z),
        );
        sum.0 += c.0;
        sum.1 += c.1;
        sum.2 += c.2;
        prev = [x, y, z];
    }

    {
        let (x, y, z) = (first[0], first[1], first[2]);
        let c = cross(
            (prev[0] - x, prev[1] - y, prev[2] - z),
            (prev[0] + x, prev[1] + y, prev[2] + z),
        );
        sum.0 += c.0;
        sum.1 += c.1;
        sum.2 += c.2;
    }

    match (sum.0 * sum.0 + sum.1 * sum.1 + sum.2 * sum.2).sqrt() {
        d if d < 1e-30 => None,
        d => Some((sum.0 / d, sum.1 / d, sum.2 / d)),
    }
}

pub fn add_padding(buf: &mut Vec<u8>, align: usize) {
    let len = buf.len();
    let pad = (align - (len % align)) % align;
    buf.resize(len + pad, 0);
}
