use flatgeom::{MultiLineString, MultiPoint, MultiPolygon};
use nusamai_citygml::{
    geometry::{GeometryRef, GeometryStore},
    GeometryType,
};
use nusamai_gpkg::{
    geometry::{
        write_indexed_multilinestring, write_indexed_multipoint, write_indexed_multipolygon,
    },
    table::GpkgGeometryType,
};

use super::bbox::{
    get_indexed_multilinestring_bbox, get_indexed_multipoint_bbox, get_indexed_multipolygon_bbox,
    Bbox,
};
use crate::pipeline::{PipelineError, Result};

#[derive(Debug)]
pub(super) struct EncodedGeometry {
    pub(super) geometry_type: GpkgGeometryType,
    pub(super) bytes: Vec<u8>,
    pub(super) bbox: Bbox,
}

fn geometry_range(
    entry: &GeometryRef,
    available: usize,
    table_name: &str,
    obj_id: &str,
) -> Result<std::ops::Range<usize>> {
    let end = entry.pos.checked_add(entry.len).ok_or_else(|| {
        PipelineError::Other(format!(
            "geometry range overflow in table {table_name}, feature {obj_id}"
        ))
    })?;
    let range = entry.pos as usize..end as usize;
    if range.end > available {
        return Err(PipelineError::Other(format!(
            "geometry range {:?} exceeds {available} geometries in table {table_name}, feature {obj_id}",
            range
        )));
    }
    Ok(range)
}

pub(super) fn encode_feature_geometry(
    geom_store: &GeometryStore,
    geometries: &[GeometryRef],
    srs_id: u16,
    table_name: &str,
    obj_id: &str,
) -> Result<Option<EncodedGeometry>> {
    if geometries.is_empty() {
        return Ok(None);
    }

    let mut multipoint = MultiPoint::<u32>::new();
    let mut multiline = MultiLineString::<u32>::new();
    let mut multipolygon = MultiPolygon::<u32>::new();

    for entry in geometries {
        match entry.ty {
            GeometryType::Point => {
                let range = geometry_range(entry, geom_store.multipoint.len(), table_name, obj_id)?;
                for point in geom_store.multipoint.iter_range(range) {
                    multipoint.push(point);
                }
            }
            GeometryType::Curve => {
                let range =
                    geometry_range(entry, geom_store.multilinestring.len(), table_name, obj_id)?;
                for linestring in geom_store.multilinestring.iter_range(range) {
                    multiline.add_linestring(linestring.iter());
                }
            }
            GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
                let range =
                    geometry_range(entry, geom_store.multipolygon.len(), table_name, obj_id)?;
                for polygon in geom_store.multipolygon.iter_range(range) {
                    multipolygon.push(&polygon);
                }
            }
        }
    }

    let geometry_type_count = [
        !multipoint.is_empty(),
        !multiline.is_empty(),
        !multipolygon.is_empty(),
    ]
    .into_iter()
    .filter(|populated| *populated)
    .count();
    if geometry_type_count == 0 {
        return Ok(None);
    }
    if geometry_type_count > 1 {
        return Err(PipelineError::Other(format!(
            "mixed geometry types are not supported in table {table_name}, feature {obj_id}"
        )));
    }

    if geom_store.epsg != srs_id {
        return Err(PipelineError::Other(format!(
            "geometry SRS mismatch in table {table_name}, feature {obj_id}: expected EPSG:{srs_id}, actual EPSG:{}",
            geom_store.epsg
        )));
    }

    let mut bytes = Vec::new();
    let encoded = if !multipoint.is_empty() {
        write_indexed_multipoint(
            &mut bytes,
            &geom_store.vertices,
            &multipoint,
            i32::from(srs_id),
        )?;
        EncodedGeometry {
            geometry_type: GpkgGeometryType::MultiPoint,
            bbox: get_indexed_multipoint_bbox(&geom_store.vertices, &multipoint),
            bytes,
        }
    } else if !multiline.is_empty() {
        write_indexed_multilinestring(
            &mut bytes,
            &geom_store.vertices,
            &multiline,
            i32::from(srs_id),
        )?;
        EncodedGeometry {
            geometry_type: GpkgGeometryType::MultiLineString,
            bbox: get_indexed_multilinestring_bbox(&geom_store.vertices, &multiline),
            bytes,
        }
    } else {
        write_indexed_multipolygon(
            &mut bytes,
            &geom_store.vertices,
            &multipolygon,
            i32::from(srs_id),
        )?;
        EncodedGeometry {
            geometry_type: GpkgGeometryType::MultiPolygon,
            bbox: get_indexed_multipolygon_bbox(&geom_store.vertices, &multipolygon),
            bytes,
        }
    };

    Ok(Some(encoded))
}

#[cfg(test)]
mod tests {
    use nusamai_projection::crs::{EPSG_WGS84_GEOGRAPHIC_2D, EPSG_WGS84_GEOGRAPHIC_3D};

    use super::*;

    #[test]
    fn feature_geometry_point_uses_schema_srid() {
        let mut points = MultiPoint::<u32>::new();
        points.push(0);
        let store = GeometryStore {
            epsg: EPSG_WGS84_GEOGRAPHIC_3D,
            vertices: vec![[139.0, 35.0, 12.0]],
            multipoint: points,
            ..Default::default()
        };
        let refs = vec![GeometryRef {
            ty: GeometryType::Point,
            lod: 0,
            pos: 0,
            len: 1,
        }];

        let geometry = encode_feature_geometry(&store, &refs, 4979, "geojson:Feature", "point-1")
            .unwrap()
            .unwrap();

        assert_eq!(geometry.geometry_type, GpkgGeometryType::MultiPoint);
        assert_eq!(&geometry.bytes[4..8], &4979_i32.to_le_bytes());
        assert_eq!(geometry.bbox.to_tuple(), (139.0, 35.0, 139.0, 35.0));
    }

    #[test]
    fn feature_geometry_line_uses_schema_srid() {
        let mut lines = MultiLineString::<u32>::new();
        lines.add_linestring([0, 1]);
        let store = GeometryStore {
            epsg: EPSG_WGS84_GEOGRAPHIC_2D,
            vertices: vec![[139.0, 35.0, 0.0], [140.0, 36.0, 0.0]],
            multilinestring: lines,
            ..Default::default()
        };
        let refs = vec![GeometryRef {
            ty: GeometryType::Curve,
            lod: 0,
            pos: 0,
            len: 1,
        }];

        let geometry = encode_feature_geometry(&store, &refs, 4326, "geojson:Feature", "line-1")
            .unwrap()
            .unwrap();

        assert_eq!(geometry.geometry_type, GpkgGeometryType::MultiLineString);
        assert_eq!(&geometry.bytes[4..8], &4326_i32.to_le_bytes());
        assert_eq!(geometry.bbox.to_tuple(), (139.0, 35.0, 140.0, 36.0));
    }

    #[test]
    fn feature_geometry_rejects_schema_and_store_srid_mismatch() {
        let mut points = MultiPoint::<u32>::new();
        points.push(0);
        let store = GeometryStore {
            epsg: EPSG_WGS84_GEOGRAPHIC_2D,
            vertices: vec![[139.0, 35.0, 0.0]],
            multipoint: points,
            ..Default::default()
        };
        let refs = vec![GeometryRef {
            ty: GeometryType::Point,
            lod: 0,
            pos: 0,
            len: 1,
        }];

        let error =
            encode_feature_geometry(&store, &refs, 3857, "geojson:Feature", "point-1").unwrap_err();

        assert!(error.to_string().contains("point-1"));
        assert!(error.to_string().contains("expected EPSG:3857"));
        assert!(error.to_string().contains("actual EPSG:4326"));
    }

    #[test]
    fn feature_geometry_rejects_mixed_types() {
        let mut points = MultiPoint::<u32>::new();
        points.push(0);
        let mut lines = MultiLineString::<u32>::new();
        lines.add_linestring([0, 1]);
        let store = GeometryStore {
            epsg: EPSG_WGS84_GEOGRAPHIC_2D,
            vertices: vec![[139.0, 35.0, 0.0], [140.0, 36.0, 0.0]],
            multipoint: points,
            multilinestring: lines,
            ..Default::default()
        };
        let refs = vec![
            GeometryRef {
                ty: GeometryType::Point,
                lod: 0,
                pos: 0,
                len: 1,
            },
            GeometryRef {
                ty: GeometryType::Curve,
                lod: 0,
                pos: 0,
                len: 1,
            },
        ];

        let error =
            encode_feature_geometry(&store, &refs, 4326, "geojson:Feature", "mixed-1").unwrap_err();

        assert!(error.to_string().contains("mixed geometry types"));
        assert!(error.to_string().contains("mixed-1"));
    }
}
