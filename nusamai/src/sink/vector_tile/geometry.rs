//! Geometry conversion shared by vector tile encoders.

use flatgeom::MultiPolygon2;

#[derive(Debug, PartialEq)]
pub(crate) struct QuantizedPolygon {
    pub(crate) exterior: Vec<[i32; 2]>,
    pub(crate) interiors: Vec<Vec<[i32; 2]>>,
}

pub(crate) fn quantize_polygons(
    multipolygon: &MultiPolygon2,
    extent: i32,
) -> Vec<QuantizedPolygon> {
    multipolygon
        .into_iter()
        .filter_map(|polygon| {
            let mut exterior = None;
            let mut interiors = Vec::new();

            for (ring_index, ring) in polygon.rings().enumerate() {
                let quantized = ring.into_iter().map(|[x, y]| {
                    [
                        (x * f64::from(extent) + 0.5) as i32,
                        (y * f64::from(extent) + 0.5) as i32,
                    ]
                });
                let Some(ring) = simplify_ring(quantized) else {
                    continue;
                };

                let area = signed_ring_area2(&ring);
                if ring_index == 0 {
                    if area > 0 {
                        exterior = Some(ring);
                    }
                } else if area < 0 {
                    interiors.push(ring);
                }
            }

            exterior.map(|exterior| QuantizedPolygon {
                exterior,
                interiors,
            })
        })
        .collect()
}

fn simplify_ring(points: impl IntoIterator<Item = [i32; 2]>) -> Option<Vec<[i32; 2]>> {
    let points = points.into_iter().collect::<Vec<_>>();
    if points.len() < 3 {
        return None;
    }

    let mut simplified = Vec::with_capacity(points.len());
    simplified.push(points[0]);
    for window in points.windows(3) {
        let &[previous, current, next] = window.try_into().unwrap();
        if previous == current || is_collinear(previous, current, next) {
            continue;
        }
        simplified.push(current);
    }
    simplified.push(*points.last().unwrap());

    (simplified.len() >= 3).then_some(simplified)
}

fn is_collinear(previous: [i32; 2], current: [i32; 2], next: [i32; 2]) -> bool {
    if current == next {
        return false;
    }

    let [current_x, current_y] = current.map(i64::from);
    let [previous_x, previous_y] = previous.map(i64::from);
    let [next_x, next_y] = next.map(i64::from);
    ((next_y - previous_y) * (current_x - previous_x)).abs()
        == ((current_y - previous_y) * (next_x - previous_x)).abs()
}

pub(crate) fn signed_ring_area2(ring: &[[i32; 2]]) -> i64 {
    ring.iter()
        .zip(ring.iter().cycle().skip(1))
        .take(ring.len())
        .map(|([x1, y1], [x2, y2])| {
            (i64::from(*x1) * i64::from(*y2)) - (i64::from(*x2) * i64::from(*y1))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{signed_ring_area2, simplify_ring};

    #[test]
    fn ring_simplification_removes_duplicates_and_collinear_points() {
        let ring = simplify_ring([[0, 0], [0, 0], [1, 0], [2, 0], [2, 2], [0, 2]])
            .expect("ring should remain valid");

        assert_eq!(ring, vec![[0, 0], [2, 0], [2, 2], [0, 2]]);
        assert!(signed_ring_area2(&ring) > 0);
    }

    #[test]
    fn duplicate_next_point_is_not_treated_as_collinear() {
        let ring = simplify_ring([[0, 0], [2, 0], [2, 0], [2, 2], [0, 2]])
            .expect("ring should remain valid");

        assert_eq!(ring, vec![[0, 0], [2, 0], [2, 2], [0, 2]]);
    }
}
