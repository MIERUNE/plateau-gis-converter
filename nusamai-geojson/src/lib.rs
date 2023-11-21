use nusamai_geometry::{CoordNum, Geometry, MultiPolygon, Polygon};

pub fn geometries_to_geojson<const D: usize, T: CoordNum>(
    geometries: Vec<Geometry<D, T>>,
) -> geojson::GeoJson {
    let geojson_features = geometries
        .iter()
        .map(|geom| nusamai_to_geojson_geometry(geom))
        .map(geojson_geometry_to_feature);

    let geojson_feature_collection = geojson::FeatureCollection {
        bbox: None,
        features: geojson_features.collect(),
        foreign_members: None,
    };

    geojson::GeoJson::from(geojson_feature_collection)
}

pub fn geojson_geometry_to_feature(geojson_geom: geojson::Geometry) -> geojson::Feature {
    geojson::Feature {
        bbox: None,
        geometry: Some(geojson_geom),
        id: None,
        properties: None,
        foreign_members: None,
    }
}

pub fn nusamai_to_geojson_geometry<const D: usize, T: CoordNum>(
    geometry: &Geometry<D, T>,
) -> geojson::Geometry {
    match geometry {
        Geometry::MultiPoint(_) => unimplemented!(),
        Geometry::LineString(_) => unimplemented!(),
        Geometry::MultiLineString(_) => unimplemented!(),
        Geometry::Polygon(poly) => polygon_to_geojson_geometry(poly),
        Geometry::MultiPolygon(mpoly) => multi_polygon_to_geojson_geometry(mpoly),
    }
}

fn polygon_to_geojson_geometry<const D: usize, T: CoordNum>(
    poly: &Polygon<D, T>,
) -> geojson::Geometry {
    let rings = polygon_to_rings(poly);
    geojson::Geometry::new(geojson::Value::Polygon(rings))
}

fn multi_polygon_to_geojson_geometry<const D: usize, T: CoordNum>(
    mpoly: &MultiPolygon<D, T>,
) -> geojson::Geometry {
    let ring_list: Vec<geojson::PolygonType> =
        mpoly.iter().map(|poly| polygon_to_rings(&poly)).collect();
    geojson::Geometry::new(geojson::Value::MultiPolygon(ring_list))
}

fn polygon_to_rings<const D: usize, T: CoordNum>(poly: &Polygon<D, T>) -> geojson::PolygonType {
    let mut rings = Vec::new();

    let exterior = poly.exterior();
    let exterior_positions = exterior
        .iter_closed()
        .map(|slice| slice.iter().map(|&t| t.to_f64().unwrap()).collect())
        .collect::<Vec<Vec<f64>>>();

    rings.push(exterior_positions);

    for interior in poly.interiors() {
        let interior_positions = interior
            .iter_closed()
            .map(|slice| slice.iter().map(|&t| t.to_f64().unwrap()).collect())
            .collect::<Vec<Vec<f64>>>();
        rings.push(interior_positions);
    }

    rings
}

#[cfg(test)]
mod tests {
    use super::*;
    use geojson::GeoJson;
    use nusamai_geometry::{MultiPolygon2, Polygon2};

    #[test]
    fn test_polygon_basic() {
        let rings: Vec<Vec<f64>> = vec![
            vec![100., 200., 100., 100., 200., 100., 200., 200.],
            vec![110., 190., 130., 190., 130., 110., 110., 110.],
            vec![160., 190., 170., 110., 150., 110.],
        ];
        let coords: Vec<f64> = rings.into_iter().flatten().collect();

        let hole_indices: Vec<u32> = vec![4];
        let poly = Polygon2::from_raw(coords.into(), hole_indices.into());

        let geojson_geometry = polygon_to_geojson_geometry(&poly);
        let geojson = GeoJson::from(geojson_geometry);

        println!("{}", geojson);
    }

    #[test]
    fn test_multi_polygon_basic() {
        let mut mpoly = MultiPolygon2::new();

        // 1st polygon
        mpoly.add_exterior([[0., 0.], [5., 0.], [5., 5.], [0., 5.], [0., 0.]]);
        mpoly.add_interior([[1., 1.], [2., 1.], [2., 2.], [1., 2.], [1., 1.]]);
        mpoly.add_interior([[3., 3.], [4., 3.], [4., 4.], [3., 4.], [3., 3.]]);

        // 2nd polygon
        mpoly.add_exterior([[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]);
        mpoly.add_interior([[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]);

        // 3rd polygon
        mpoly.add_exterior([[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]);
        mpoly.add_interior([[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]);

        let geojson_geometry = multi_polygon_to_geojson_geometry(&mpoly);
        println!("{}", GeoJson::from(geojson_geometry));
        // TODO: check the result
    }
}
