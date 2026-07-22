//! Vector tile geometry slicing based on [geojson-vt](https://github.com/mapbox/geojson-vt).

use flatgeom::{LineString2, MultiLineString2, MultiPoint2, MultiPolygon2, Polygon2};
use hashbrown::HashMap;
use nusamai_citygml::{
    geometry::GeometryType,
    object::{ObjectStereotype, Value},
    GeometryRef,
};
use nusamai_plateau::Entity;
use tinymvt::{webmercator::lnglat_to_web_mercator, TileZXY};

use crate::pipeline::PipelineError;

use super::feature::SlicedGeometry;

type TileKey = (u8, u32, u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VectorTileGeometryType {
    Point,
    LineString,
    Polygon,
}

impl From<GeometryType> for VectorTileGeometryType {
    fn from(geometry_type: GeometryType) -> Self {
        match geometry_type {
            GeometryType::Point => Self::Point,
            GeometryType::Curve => Self::LineString,
            GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => Self::Polygon,
        }
    }
}

fn resolve_vector_tile_geometry_type(
    geometries: &[GeometryRef],
) -> Result<Option<VectorTileGeometryType>, PipelineError> {
    let mut resolved = None;

    for geometry in geometries {
        let current = VectorTileGeometryType::from(geometry.ty);
        match resolved {
            Some(previous) if previous != current => {
                return Err(PipelineError::Other(format!(
                    "A vector tile feature must contain one geometry type, but found {previous:?} and {current:?}"
                )));
            }
            Some(_) => {}
            None => resolved = Some(current),
        }
    }

    Ok(resolved)
}

pub(crate) fn validate_zoom_range(min_z: u8, max_z: u8) {
    assert!(
        min_z <= max_z,
        "Invalid zoom range: min_z ({min_z}) must be less than or equal to max_z ({max_z})"
    );
}

pub(crate) fn slice_cityobj_geoms(
    obj: &Entity,
    min_z: u8,
    max_z: u8,
    max_detail: u32,
    buffer_pixels: u32,
    f: impl Fn(TileZXY, SlicedGeometry) -> Result<(), PipelineError>,
) -> Result<(), PipelineError> {
    validate_zoom_range(min_z, max_z);

    let extent = 1_u32 << max_detail;
    let buffer = extent * buffer_pixels / 256;

    let Value::Object(root_object) = &obj.root else {
        return Ok(());
    };
    let ObjectStereotype::Feature { geometries, .. } = &root_object.stereotype else {
        return Ok(());
    };

    let Some(geometry_type) = resolve_vector_tile_geometry_type(geometries).map_err(|error| {
        let feature_id = root_object.stereotype.id().unwrap_or("<unknown>");
        PipelineError::Other(format!(
            "Failed to slice vector tile feature '{feature_id}': {error}"
        ))
    })?
    else {
        return Ok(());
    };

    let geom_store = obj.geometry_store.read().unwrap();

    match geometry_type {
        VectorTileGeometryType::Point => {
            let mut tiled_points: HashMap<TileKey, MultiPoint2> = HashMap::new();
            for entry in geometries {
                for index in geom_store
                    .multipoint
                    .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
                {
                    let [longitude, latitude, _height] = geom_store.vertices[index as usize];
                    let (x, y) = lnglat_to_web_mercator(longitude, latitude);

                    for zoom in min_z..=max_z {
                        slice_point(zoom, extent, buffer, [x, y], &mut tiled_points);
                    }
                }
            }

            for ((z, x, y), points) in tiled_points {
                if !points.is_empty() {
                    f((z, x, y), SlicedGeometry::Point(points))?;
                }
            }
        }
        VectorTileGeometryType::LineString => {
            let mut tiled_lines: HashMap<TileKey, MultiLineString2> = HashMap::new();
            for entry in geometries {
                for indexed_line in geom_store
                    .multilinestring
                    .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
                {
                    let line = indexed_line.transform(|index| {
                        let [longitude, latitude, _height] = geom_store.vertices[*index as usize];
                        let (x, y) = lnglat_to_web_mercator(longitude, latitude);
                        [x, y]
                    });

                    for zoom in min_z..=max_z {
                        slice_linestring(zoom, extent, buffer, line.raw_coords(), &mut tiled_lines);
                    }
                }
            }

            for ((z, x, y), lines) in tiled_lines {
                if !lines.is_empty() {
                    f((z, x, y), SlicedGeometry::LineString(lines))?;
                }
            }
        }
        VectorTileGeometryType::Polygon => {
            let mut tiled_polygons: HashMap<TileKey, MultiPolygon2> = HashMap::new();
            for entry in geometries {
                for indexed_polygon in geom_store
                    .multipolygon
                    .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
                {
                    let polygon = indexed_polygon.transform(|coordinate| {
                        let [longitude, latitude, _height] =
                            geom_store.vertices[*coordinate as usize];
                        let (x, y) = lnglat_to_web_mercator(longitude, latitude);
                        [x, y]
                    });

                    // Early rejection of polygons that are not front-facing.
                    if !polygon.exterior().is_cw() {
                        continue;
                    }
                    debug_assert!(polygon.exterior().ring_area() > 0.0);

                    let area = polygon.area();
                    for zoom in min_z..=max_z {
                        // Skip if the polygon is smaller than 4 square subpixels.
                        // TODO: emulate the 'tiny-polygon-reduction' of tippecanoe.
                        if area * (4u64.pow(zoom as u32 + max_detail) as f64) < 4.0 {
                            continue;
                        }

                        slice_polygon(zoom, extent, buffer, &polygon, &mut tiled_polygons);
                    }
                }
            }

            for ((z, x, y), polygons) in tiled_polygons {
                if !polygons.is_empty() {
                    f((z, x, y), SlicedGeometry::Polygon(polygons))?;
                }
            }
        }
    }

    Ok(())
}

fn slice_point(
    zoom: u8,
    extent: u32,
    buffer: u32,
    [x, y]: [f64; 2],
    out: &mut HashMap<TileKey, MultiPoint2>,
) {
    if !x.is_finite() || !y.is_finite() {
        return;
    }

    let tile_count = 1_i64 << zoom;
    let scale = tile_count as f64;
    let buffer_width = buffer as f64 / extent as f64;
    let tile_x = x * scale;
    let tile_y = y * scale;
    let min_x = (tile_x - buffer_width).floor() as i64;
    let max_x = (tile_x + buffer_width).floor() as i64;
    let min_y = ((tile_y - buffer_width).floor() as i64).max(0);
    let max_y = ((tile_y + buffer_width).floor() as i64).min(tile_count - 1);

    if min_y > max_y {
        return;
    }

    for yi in min_y..=max_y {
        let local_y = tile_y - yi as f64;
        if local_y < -buffer_width || local_y >= 1.0 + buffer_width {
            continue;
        }

        for xi in min_x..=max_x {
            let local_x = tile_x - xi as f64;
            if local_x < -buffer_width || local_x >= 1.0 + buffer_width {
                continue;
            }

            let key = (zoom, xi.rem_euclid(tile_count) as u32, yi as u32);
            out.entry(key).or_default().push([local_x, local_y]);
        }
    }
}

fn slice_linestring(
    zoom: u8,
    extent: u32,
    buffer: u32,
    line: &[[f64; 2]],
    out: &mut HashMap<TileKey, MultiLineString2>,
) {
    if line.len() < 2
        || line
            .iter()
            .any(|point| !point[0].is_finite() || !point[1].is_finite())
    {
        return;
    }

    let tile_count = 1_i64 << zoom;
    let scale = tile_count as f64;
    let buffer_width = buffer as f64 / extent as f64;
    let (min_x, max_x, min_y, max_y) = line.iter().fold(
        (f64::MAX, f64::MIN, f64::MAX, f64::MIN),
        |(min_x, max_x, min_y, max_y), [x, y]| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );
    let min_tile_x = (min_x * scale - buffer_width).floor() as i64;
    let max_tile_x = (max_x * scale + buffer_width).floor() as i64;
    let min_tile_y = ((min_y * scale - buffer_width).floor() as i64).max(0);
    let max_tile_y = ((max_y * scale + buffer_width).floor() as i64).min(tile_count - 1);

    if min_tile_y > max_tile_y {
        return;
    }

    let scaled_line = line
        .iter()
        .map(|[x, y]| [x * scale, y * scale])
        .collect::<Vec<_>>();

    for yi in min_tile_y..=max_tile_y {
        for xi in min_tile_x..=max_tile_x {
            let bounds = [
                xi as f64 - buffer_width,
                yi as f64 - buffer_width,
                xi as f64 + 1.0 + buffer_width,
                yi as f64 + 1.0 + buffer_width,
            ];
            let fragments = clip_linestring(&scaled_line, bounds);
            if fragments.is_empty() {
                continue;
            }

            let key = (zoom, xi.rem_euclid(tile_count) as u32, yi as u32);
            let tile_lines = out.entry(key).or_default();
            for fragment in fragments {
                let local = fragment
                    .into_iter()
                    .map(|[x, y]| [x - xi as f64, y - yi as f64])
                    .collect::<Vec<_>>();
                if local.len() >= 2 {
                    tile_lines.add_linestring(local);
                }
            }
        }
    }
}

fn clip_linestring(line: &[[f64; 2]], bounds: [f64; 4]) -> Vec<Vec<[f64; 2]>> {
    let mut fragments = Vec::new();
    let mut current = Vec::new();

    for segment in line.windows(2) {
        let Some((start, end)) = clip_segment(segment[0], segment[1], bounds) else {
            push_fragment(&mut fragments, &mut current);
            continue;
        };

        if current.last() == Some(&start) {
            if current.last() != Some(&end) {
                current.push(end);
            }
        } else {
            push_fragment(&mut fragments, &mut current);
            current.push(start);
            if start != end {
                current.push(end);
            }
        }
    }
    push_fragment(&mut fragments, &mut current);

    fragments
}

fn push_fragment(fragments: &mut Vec<Vec<[f64; 2]>>, current: &mut Vec<[f64; 2]>) {
    if current.len() >= 2 {
        fragments.push(std::mem::take(current));
    } else {
        current.clear();
    }
}

fn clip_segment(
    start: [f64; 2],
    end: [f64; 2],
    [min_x, min_y, max_x, max_y]: [f64; 4],
) -> Option<([f64; 2], [f64; 2])> {
    let dx = end[0] - start[0];
    let dy = end[1] - start[1];
    let mut enter = 0.0_f64;
    let mut exit = 1.0_f64;

    for (p, q) in [
        (-dx, start[0] - min_x),
        (dx, max_x - start[0]),
        (-dy, start[1] - min_y),
        (dy, max_y - start[1]),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
            continue;
        }

        let ratio = q / p;
        if p < 0.0 {
            if ratio > exit {
                return None;
            }
            enter = enter.max(ratio);
        } else {
            if ratio < enter {
                return None;
            }
            exit = exit.min(ratio);
        }
    }

    Some((
        [start[0] + enter * dx, start[1] + enter * dy],
        [start[0] + exit * dx, start[1] + exit * dy],
    ))
}

fn slice_polygon(
    zoom: u8,
    extent: u32,
    buffer: u32,
    poly: &Polygon2,
    out: &mut HashMap<TileKey, MultiPolygon2>,
) {
    let tile_count = 1_u32 << zoom;
    let z_scale = f64::from(tile_count);
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
        let start = ((min_y * z_scale).floor() as i64).clamp(0, i64::from(tile_count));
        let end = ((max_y * z_scale).ceil() as i64).clamp(0, i64::from(tile_count));
        start as u32..end as u32
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
                xi.rem_euclid(tile_count as i32) as u32, // handling geometry crossing the antimeridian
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
