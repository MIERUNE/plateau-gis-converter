//! Polygon slicing algorithm based on [geojson-vt](https://github.com/mapbox/geojson-vt).

use flatgeom::{LineString2, MultiPolygon2, Polygon2};
use hashbrown::HashMap;
use nusamai_citygml::{
    geometry::GeometryType,
    object::{ObjectStereotype, Value},
};
use nusamai_plateau::Entity;
use tinymvt::{webmercator::lnglat_to_web_mercator, TileZXY};

pub fn slice_cityobj_geoms<E>(
    obj: &Entity,
    min_z: u8,
    max_z: u8,
    max_detail: u32,
    buffer_pixels: u32,
    f: impl Fn(TileZXY, MultiPolygon2) -> Result<(), E>,
) -> Result<(), E> {
    assert!(
        max_z >= min_z,
        "max_z must be greater than or equal to min_z"
    );

    let geom_store = obj.geometry_store.read().unwrap();
    if geom_store.multipolygon.is_empty() {
        return Ok(());
    }

    let mut tiled_mpolys = HashMap::new();

    let extent = 1 << max_detail;
    let buffer = extent * buffer_pixels / 256;

    let Value::Object(obj) = &obj.root else {
        return Ok(());
    };
    let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype else {
        return Ok(());
    };

    geometries.iter().for_each(|entry| match entry.ty {
        GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
            for idx_poly in geom_store
                .multipolygon
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                let poly = idx_poly.transform(|c| {
                    let [lng, lat, _height] = geom_store.vertices[*c as usize];
                    let (mx, my) = lnglat_to_web_mercator(lng, lat);
                    [mx, my]
                });

                // Early rejection of polygons that are not front-facing.
                if !poly.exterior().is_cw() {
                    continue;
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

                    slice_polygon(zoom, extent, buffer, &poly, &mut tiled_mpolys);
                }
            }
        }
        GeometryType::Curve => {
            // TODO: implement
        }
        GeometryType::Point => {
            // TODO: implement
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
    out: &mut HashMap<(u8, u32, u32), MultiPolygon2>,
) {
    let z_scale = (1 << zoom) as f64;
    let buf_width = buffer as f64 / extent as f64;
    let mut new_ring_buffer: Vec<[f64; 2]> = Vec::with_capacity(poly.exterior().len() + 1);

    // Slice along Y-axis
    let y_range = {
        let (min_y, max_y) = poly
            .exterior()
            .iter()
            .fold((f64::MAX, f64::MIN), |(min_y, max_y), c| {
                (min_y.min(c[1]), max_y.max(c[1]))
            });
        (min_y * z_scale).floor() as u32..(max_y * z_scale).ceil() as u32
    };

    let mut y_sliced_polys = Vec::with_capacity(y_range.len());

    for yi in y_range.clone() {
        let k1 = (yi as f64 - buf_width) / z_scale;
        let k2 = ((yi + 1) as f64 + buf_width) / z_scale;
        let mut y_sliced_poly = Polygon2::new();

        // todo?: check interior bbox to optimize

        for ring in poly.rings() {
            if ring.raw_coords().is_empty() {
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

            y_sliced_poly.add_ring(new_ring_buffer.iter().copied());
        }

        y_sliced_polys.push(y_sliced_poly);
    }

    let mut norm_coords_buf = Vec::new();

    // Slice along X-axis
    for (yi, y_sliced_poly) in y_range.zip(y_sliced_polys.iter()) {
        let x_range = {
            let (min_x, max_x) = y_sliced_poly
                .exterior()
                .iter()
                .fold((f64::MAX, f64::MIN), |(min_x, max_x), c| {
                    (min_x.min(c[0]), max_x.max(c[0]))
                });
            (min_x * z_scale).floor() as i32..(max_x * z_scale).ceil() as i32
        };

        for xi in x_range {
            let k1 = (xi as f64 - buf_width) / z_scale;
            let k2 = ((xi + 1) as f64 + buf_width) / z_scale;

            // todo?: check interior bbox to optimize ...

            let key = (
                zoom,
                xi.rem_euclid(1 << zoom) as u32, // handling geometry crossing the antimeridian
                yi,
            );
            let tile_mpoly = out.entry(key).or_default();

            for (ri, ring) in y_sliced_poly.rings().enumerate() {
                if ring.raw_coords().is_empty() {
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

                // get integer coordinates and simplify the ring
                {
                    norm_coords_buf.clear();
                    norm_coords_buf.extend(new_ring_buffer.iter().map(|&[x, y]| {
                        let tx = x * z_scale - xi as f64;
                        let ty = y * z_scale - yi as f64;
                        [tx, ty]
                    }));

                    // remove closing point if exists
                    if norm_coords_buf.len() >= 2
                        && norm_coords_buf[0] == *norm_coords_buf.last().unwrap()
                    {
                        norm_coords_buf.pop();
                    }

                    if norm_coords_buf.len() < 3 {
                        continue;
                    }
                }

                let mut ring = LineString2::from_raw(norm_coords_buf.clone().into());
                ring.reverse_inplace();

                match ri {
                    0 => tile_mpoly.add_exterior(ring.iter()),
                    _ => tile_mpoly.add_interior(ring.iter()),
                };
            }
        }
    }
}
