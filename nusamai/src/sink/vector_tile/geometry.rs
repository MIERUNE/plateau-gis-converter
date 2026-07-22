//! Geometry conversion shared by vector tile encoders.

use flatgeom::{MultiLineString2, MultiPoint2, MultiPolygon2};

#[derive(Debug, PartialEq)]
pub(crate) struct QuantizedPolygon {
    pub(crate) exterior: Vec<[i32; 2]>,
    pub(crate) interiors: Vec<Vec<[i32; 2]>>,
}

pub(crate) fn quantize_points(multipoint: &MultiPoint2, extent: i32) -> Vec<[i32; 2]> {
    let mut points = multipoint
        .iter()
        .map(|point| quantize_coordinate(point, extent))
        .collect::<Vec<_>>();
    points.dedup();
    points
}

pub(crate) fn quantize_linestrings(
    multilinestring: &MultiLineString2,
    extent: i32,
) -> Vec<Vec<[i32; 2]>> {
    multilinestring
        .iter()
        .filter_map(|line| {
            simplify_linestring(line.iter().map(|point| quantize_coordinate(point, extent)))
        })
        .collect()
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

fn quantize_coordinate([x, y]: [f64; 2], extent: i32) -> [i32; 2] {
    [
        (x * f64::from(extent)).round() as i32,
        (y * f64::from(extent)).round() as i32,
    ]
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

fn simplify_linestring(points: impl IntoIterator<Item = [i32; 2]>) -> Option<Vec<[i32; 2]>> {
    let mut simplified = Vec::new();

    for point in points {
        if simplified.last() == Some(&point) {
            continue;
        }

        while simplified.len() >= 2 {
            let previous = simplified[simplified.len() - 2];
            let current = simplified[simplified.len() - 1];
            if !is_forward_collinear(previous, current, point) {
                break;
            }
            simplified.pop();
        }

        simplified.push(point);
    }

    (simplified.len() >= 2).then_some(simplified)
}

fn is_forward_collinear(previous: [i32; 2], current: [i32; 2], next: [i32; 2]) -> bool {
    let [ax, ay] = previous.map(i128::from);
    let [bx, by] = current.map(i128::from);
    let [cx, cy] = next.map(i128::from);
    let ab = [bx - ax, by - ay];
    let bc = [cx - bx, cy - by];
    let cross = ab[0] * bc[1] - ab[1] * bc[0];
    let dot = ab[0] * bc[0] + ab[1] * bc[1];
    cross == 0 && dot >= 0
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
