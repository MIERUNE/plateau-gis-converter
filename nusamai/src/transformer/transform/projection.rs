use std::sync::Arc;

use crate::transformer::Transform;

use nusamai_citygml::object::Entity;
use nusamai_citygml::schema::Schema;
use nusamai_projection::crs::*;
use nusamai_projection::vshift::Jgd2011ToWgs84;

/// Coordinate transformation. Currently supports only JGD2011 to WGS 84.
#[derive(Clone)]
pub struct ProjectionTransform {
    jgd2wgs: Arc<Jgd2011ToWgs84>,
}

impl Transform for ProjectionTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        let epsg = {
            let geom_store = entity.geometry_store.read().unwrap();
            geom_store.epsg
        };

        // TODO: From: JGD2011 To: WGS 84 のみを想定している。他のCRSにも今後対応する。

        match epsg {
            EPSG_JGD2011_GEOGRAPHIC_3D => {
                let mut geom_store = entity.geometry_store.write().unwrap();
                geom_store.vertices.iter_mut().for_each(|v| {
                    // Swap x and y (lat, lng -> lng, lat)
                    let (lng, lat, height) = (v[1], v[0], v[2]);
                    // JGD2011 to WGS 84 (elevation to ellipsoidal height)
                    (v[0], v[1], v[2]) = self.jgd2wgs.convert(lng, lat, height);
                });

                // Ensure that the source CRS is JGD2011 and the destination is WGS 84
                assert_eq!(geom_store.epsg, EPSG_JGD2011_GEOGRAPHIC_3D);
                geom_store.epsg = EPSG_WGS84_GEOGRAPHIC_3D;
            }
            EPSG_WGS84_GEOGRAPHIC_3D => {
                // Do nothing
            }
            _ => {
                panic!("Unsupported CRS: {}", epsg);
            }
        }

        out.push(entity);
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // TODO: Change the CRS information in the schema (?)
    }
}

impl ProjectionTransform {
    pub fn new(jgd2wgs: Arc<Jgd2011ToWgs84>) -> Self {
        Self { jgd2wgs }
    }
}
