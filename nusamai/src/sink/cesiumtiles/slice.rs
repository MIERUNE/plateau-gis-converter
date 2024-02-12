//! Polygon slicing algorithm based on [geojson-vt](https://github.com/mapbox/geojson-vt).

use hashbrown::HashMap;

use super::tiling;
use nusamai_citygml::{
    geometry::GeometryType,
    object::{ObjectStereotype, Value},
};
use nusamai_geometry::{LineString3, MultiPolygon3, Polygon3};
use nusamai_mvt::TileZXY;
use nusamai_plateau::Entity;

pub fn slice_cityobj_geoms<E>(
    obj: &Entity,
    min_z: u8,
    max_z: u8,
    f: impl Fn(TileZXY, MultiPolygon3) -> Result<(), E>,
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
                let poly = idx_poly.transform(|c| geom_store.vertices[c[0] as usize]);

                // Slice for each zoom level
                for zoom in min_z..=max_z {
                    slice_polygon(zoom, &poly, &mut tiled_mpolys);
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

fn slice_polygon(zoom: u8, poly: &Polygon3, out: &mut HashMap<(u8, u32, u32), MultiPolygon3>) {
    if poly.exterior().is_empty() {
        return;
    }

    let mut new_ring_buffer: Vec<[f64; 3]> = Vec::with_capacity(poly.exterior().len() + 1);

    // Slice along Y-axis
    let y_range = {
        let (min_y, max_y) = poly
            .exterior()
            .iter()
            .fold((f64::MAX, f64::MIN), |(min_y, max_y), c| {
                (min_y.min(c[1]), max_y.max(c[1]))
            });
        tiling::iter_y_slice(zoom, min_y, max_y)
    };

    let mut y_sliced_polys = Vec::with_capacity(y_range.len());

    for yi in y_range.clone() {
        let (k1, k2) = tiling::y_slice_range(zoom, yi);
        let mut y_sliced_poly = Polygon3::new();

        // todo?: check interior bbox to optimize

        for ring in poly.rings() {
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
                            let z = (b[2] - a[2]) * (k1 - a[1]) / (b[1] - a[1]) + a[2];
                            new_ring_buffer.push([x, k1, z])
                        }
                    } else if a[1] > k2 {
                        if b[1] < k2 {
                            let x = (b[0] - a[0]) * (k2 - a[1]) / (b[1] - a[1]) + a[0];
                            let z = (b[2] - a[2]) * (k2 - a[1]) / (b[1] - a[1]) + a[2];
                            new_ring_buffer.push([x, k2, z])
                        }
                    } else {
                        new_ring_buffer.push(a)
                    }

                    if b[1] < k1 && a[1] > k1 {
                        let x = (b[0] - a[0]) * (k1 - a[1]) / (b[1] - a[1]) + a[0];
                        let z = (b[2] - a[2]) * (k1 - a[1]) / (b[1] - a[1]) + a[2];
                        new_ring_buffer.push([x, k1, z])
                    } else if b[1] > k2 && a[1] < k2 {
                        let x = (b[0] - a[0]) * (k2 - a[1]) / (b[1] - a[1]) + a[0];
                        let z = (b[2] - a[2]) * (k2 - a[1]) / (b[1] - a[1]) + a[2];
                        new_ring_buffer.push([x, k2, z])
                    }

                    Some(b)
                })
                .unwrap();

            y_sliced_poly.add_ring(new_ring_buffer.iter().copied());
        }

        y_sliced_polys.push(y_sliced_poly);
    }

    // Slice along X-axis
    for (yi, y_sliced_poly) in y_range.zip(y_sliced_polys.iter()) {
        let x_iter = {
            let (min_x, max_x) = y_sliced_poly
                .exterior()
                .iter()
                .fold((f64::MAX, f64::MIN), |(min_x, max_x), c| {
                    (min_x.min(c[0]), max_x.max(c[0]))
                });

            tiling::iter_x_slice(zoom, yi, min_x, max_x)
        };

        for (xi, xs) in x_iter {
            let (k1, k2) = tiling::x_slice_range(zoom, xi, xs);

            // todo?: check interior bbox to optimize ...

            let key = (
                zoom,
                xi.rem_euclid(1 << zoom) as u32, // handling geometry crossing the antimeridian
                yi,
            );
            let tile_mpoly = out.entry(key).or_default();

            for (ri, ring) in y_sliced_poly.rings().enumerate() {
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
                                let z = (b[2] - a[2]) * (k1 - a[0]) / (b[0] - a[0]) + a[2];
                                new_ring_buffer.push([k1, y, z])
                            }
                        } else if a[0] > k2 {
                            if b[0] < k2 {
                                let y = (b[1] - a[1]) * (k2 - a[0]) / (b[0] - a[0]) + a[1];
                                let z = (b[2] - a[2]) * (k2 - a[0]) / (b[0] - a[0]) + a[2];
                                new_ring_buffer.push([k2, y, z])
                            }
                        } else {
                            new_ring_buffer.push(a)
                        }

                        if b[0] < k1 && a[0] > k1 {
                            let y = (b[1] - a[1]) * (k1 - a[0]) / (b[0] - a[0]) + a[1];
                            let z = (b[2] - a[2]) * (k1 - a[0]) / (b[0] - a[0]) + a[2];
                            new_ring_buffer.push([k1, y, z])
                        } else if b[0] > k2 && a[0] < k2 {
                            let y = (b[1] - a[1]) * (k2 - a[0]) / (b[0] - a[0]) + a[1];
                            let z = (b[2] - a[2]) * (k2 - a[0]) / (b[0] - a[0]) + a[2];
                            new_ring_buffer.push([k2, y, z])
                        }

                        Some(b)
                    })
                    .unwrap();

                let ring =
                    LineString3::from_raw(new_ring_buffer.iter().flatten().copied().collect());

                match ri {
                    0 => tile_mpoly.add_exterior(ring.iter()),
                    _ => tile_mpoly.add_interior(ring.iter()),
                };
            }
        }
    }
}
