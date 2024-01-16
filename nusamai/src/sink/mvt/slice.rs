//! Polygon slicing algorithm based on [geojson-vt](https://github.com/mapbox/geojson-vt).

use hashbrown::HashMap;

use nusamai_citygml::object::CityObject;
use nusamai_geometry::{LineString2, MultiPolygon2, Polygon2};
use nusamai_mvt::{webmercator::lnglat_to_web_mercator, TileZXY};

pub fn slice_cityobj_geoms(
    obj: &CityObject,
    min_z: u8,
    max_z: u8,
    max_detail: u32,
    buffer_pixels: u32,
    f: impl Fn(TileZXY, MultiPolygon2<i16>) -> Result<(), ()>,
) -> Result<(), ()> {
    assert!(
        max_z >= min_z,
        "max_z must be greater than or equal to min_z"
    );

    if obj.geometry_store.multipolygon.is_empty() {
        return Ok(());
    }

    let idx_mpoly = &obj.geometry_store.multipolygon;
    let mut tiled_mpolys = HashMap::new();

    let extent = 2u32.pow(max_detail);
    let buffer = extent * buffer_pixels / 256;

    idx_mpoly.iter().for_each(|idx_poly| {
        let poly = idx_poly.transform(|c| {
            let [lng, lat, _height] = obj.geometry_store.vertices[c[0] as usize];
            let (mx, my) = lnglat_to_web_mercator(lng, lat);
            [mx, my]
        });

        // Early rejection of polygons that are not front-facing.
        if !poly.exterior().is_cw() {
            return;
        }
        debug_assert!(poly.exterior().ring_area() > 0.0);

        let area = poly.area();

        // Slice for each zoom level
        for zoom in min_z..=max_z {
            // Skip if the polygon is smaller than 4 square subpixels
            //
            // TODO: emulate the 'tiny-polygon-reduction' of tippecanoe
            if area * (4u64.pow(zoom as u32 + max_detail) as f64) < 4.0 {
                continue;
            }

            let z_scale = 2u32.pow(zoom as u32) as f64;
            let scaled_poly = poly.transform(|c| [(c[0] * z_scale), (c[1] * z_scale)]);
            slice_polygon(zoom, extent, buffer, &scaled_poly, &mut tiled_mpolys);
        }
    });

    for ((z, x, y), mpoly) in tiled_mpolys {
        if mpoly.is_empty() {
            continue;
        }
        f((z, x, y), mpoly)?;
    }

    Ok(())
    // TODO: linestring, point
}

fn slice_polygon(
    zoom: u8,
    extent: u32,
    buffer: u32,
    poly: &Polygon2,
    out: &mut HashMap<(u8, u32, u32), MultiPolygon2<i16>>,
) {
    if poly.exterior().is_empty() {
        return;
    }

    let buf_width = buffer as f64 / extent as f64;
    let mut new_ring_buffer: Vec<[f64; 2]> = Vec::with_capacity(poly.exterior().len() + 1);

    // Slice along X-axis
    let x_range = {
        let (min_x, max_x) = poly
            .exterior()
            .iter()
            .fold((f64::MAX, f64::MIN), |(min_x, max_x), c| {
                (min_x.min(c[0]), max_x.max(c[0]))
            });
        min_x.floor() as u32..max_x.ceil() as u32
    };

    let mut x_sliced_polys = Vec::with_capacity(x_range.len());

    for xi in x_range.clone() {
        let k1 = xi as f64 - buf_width;
        let k2 = (xi + 1) as f64 + buf_width;
        let mut x_sliced_poly = Polygon2::new();

        // todo?: check interior bbox to optimize

        for ring in poly.rings() {
            if ring.coords().is_empty() {
                continue;
            }

            new_ring_buffer.clear();
            ring.iter_closed()
                .fold(None, |a, b| {
                    let Some(a) = a else { return Some(b) };

                    if a[0] < k1 {
                        if b[0] > k1 {
                            let y = (b[1] - a[1]) * (k1 - a[0]) / (b[0] - a[0]) + a[1];
                            // let z = (b[2] - a[2]) * (k1 - a[0]) / (b[0] - a[0]) + a[2];
                            new_ring_buffer.push([k1, y])
                        }
                    } else if a[0] > k2 {
                        if b[0] < k2 {
                            let y = (b[1] - a[1]) * (k2 - a[0]) / (b[0] - a[0]) + a[1];
                            // let z = (b[2] - a[2]) * (k2 - a[0]) / (b[0] - a[0]) + a[2];
                            new_ring_buffer.push([k2, y])
                        }
                    } else {
                        new_ring_buffer.push(a)
                    }

                    if b[0] < k1 && a[0] > k1 {
                        let y = (b[1] - a[1]) * (k1 - a[0]) / (b[0] - a[0]) + a[1];
                        // let z = (b[2] - a[2]) * (k1 - a[0]) / (b[0] - a[0]) + a[2];
                        new_ring_buffer.push([k1, y])
                    } else if b[0] > k2 && a[0] < k2 {
                        let y = (b[1] - a[1]) * (k2 - a[0]) / (b[0] - a[0]) + a[1];
                        // let z = (b[2] - a[2]) * (k2 - a[0]) / (b[0] - a[0]) + a[2];
                        new_ring_buffer.push([k2, y])
                    }

                    Some(b)
                })
                .unwrap();

            x_sliced_poly.add_ring(new_ring_buffer.iter().copied());
        }

        x_sliced_polys.push(x_sliced_poly);
    }

    // Slice along Y-axis
    for (xi, x_sliced_poly) in x_range.zip(x_sliced_polys.iter()) {
        let y_range = {
            let (min_y, max_y) = x_sliced_poly
                .exterior()
                .iter()
                .fold((f64::MAX, f64::MIN), |(min_y, max_y), c| {
                    (min_y.min(c[1]), max_y.max(c[1]))
                });
            min_y.floor() as u32..max_y.ceil() as u32
        };

        for yi in y_range {
            let k1 = yi as f64 - buf_width;
            let k2 = (yi + 1) as f64 + buf_width;

            // todo?: check interior bbox to optimize

            let tile_mpoly = out.entry((zoom, xi, yi)).or_default();

            for (ri, ring) in x_sliced_poly.rings().enumerate() {
                if ring.coords().is_empty() {
                    continue;
                }

                new_ring_buffer.clear();
                ring.iter_closed()
                    .fold(None, |a, b| {
                        let Some(a) = a else { return Some(b) };

                        if a[1] < k1 {
                            if b[1] > k1 {
                                let x = (b[0] - a[0]) * (k1 - a[1]) / (b[1] - a[1]) + a[0];
                                // let z = (b[2] - a[2]) * (k1 - a[1]) / (b[1] - a[1]) + a[2];
                                new_ring_buffer.push([x, k1])
                            }
                        } else if a[1] > k2 {
                            if b[1] < k2 {
                                let x = (b[0] - a[0]) * (k2 - a[1]) / (b[1] - a[1]) + a[0];
                                // let z = (b[2] - a[2]) * (k2 - a[1]) / (b[1] - a[1]) + a[2];
                                new_ring_buffer.push([x, k2])
                            }
                        } else {
                            new_ring_buffer.push(a)
                        }

                        if b[1] < k1 && a[1] > k1 {
                            let x = (b[0] - a[0]) * (k1 - a[1]) / (b[1] - a[1]) + a[0];
                            // let z = (b[2] - a[2]) * (k1 - a[1]) / (b[1] - a[1]) + a[2];
                            new_ring_buffer.push([x, k1])
                        } else if b[1] > k2 && a[1] < k2 {
                            let x = (b[0] - a[0]) * (k2 - a[1]) / (b[1] - a[1]) + a[0];
                            // let z = (b[2] - a[2]) * (k2 - a[1]) / (b[1] - a[1]) + a[2];
                            new_ring_buffer.push([x, k2])
                        }

                        Some(b)
                    })
                    .unwrap();

                let simplified = {
                    let mut coords: Vec<[i16; 2]> = new_ring_buffer
                        .iter()
                        .map(|&[x, y]| {
                            let tx = (((x - xi as f64) * (extent as f64)) + 0.5) as i16;
                            let ty = (((y - yi as f64) * (extent as f64)) + 0.5) as i16;
                            [tx, ty]
                        })
                        .collect();

                    // remove closing point if exists
                    if coords.len() >= 2 && coords[0] == *coords.last().unwrap() {
                        coords.pop();
                    }

                    if coords.len() < 3 {
                        continue;
                    }

                    let mut simplified = Vec::new();
                    simplified.push(coords[0]);
                    for c in coords.windows(3) {
                        let &[prev, curr, next] = c else {
                            unreachable!()
                        };
                        // Remove duplicate points
                        if prev == curr || curr == next {
                            continue;
                        }
                        // Reject collinear points
                        let [curr_x, curr_y] = curr;
                        let [prev_x, prev_y] = prev;
                        let [next_x, next_y] = next;
                        if ((next_y - prev_y) as i32 * (curr_x - prev_x) as i32).abs()
                            == ((curr_y - prev_y) as i32 * (next_x - prev_x) as i32).abs()
                        {
                            continue;
                        }
                        simplified.push(curr);
                    }
                    simplified.push(*coords.last().unwrap());

                    simplified
                };

                let flat_coords: Vec<i16> = simplified.iter().flatten().copied().collect();
                let mut ring = LineString2::from_raw(flat_coords.into());
                ring.reverse_inplace();

                // Skip the polygon if:
                // - The exterior ring is not front-facing
                // - Smaller than 4 square subpixels
                //
                // TODO: emulate the 'tiny-polygon-reduction' of tippecanoe
                if ri == 0 && ring.signed_ring_area() < 4.0 {
                    break;
                }

                match ri {
                    0 => tile_mpoly.add_exterior(ring.iter()),
                    _ => tile_mpoly.add_interior(ring.iter()),
                };
            }
        }
    }
}
