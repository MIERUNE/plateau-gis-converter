use std::sync::Arc;

use crate::transformer::Transform;

use nusamai_citygml::object::Entity;
use nusamai_projection::crs::*;
use nusamai_projection::vshift::JGD2011ToWGS84;

pub struct ProjectionTransform {
    jgd2wgs: Arc<JGD2011ToWGS84>,
}

impl Transform for ProjectionTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        {
            let mut geom_store = entity.geometry_store.write().unwrap();

            // FIXME: 仮実装。From: JGD2011 To: WGS 84 を仮定している。
            geom_store.vertices.iter_mut().for_each(|v| {
                // Swap x and y (lat, lng -> lng, lat)
                let (lng, lat, height) = (v[1], v[0], v[2]);
                // JGD2011 to WGS 84 (elevation to ellipsoidal height)
                (v[0], v[1], v[2]) = self.jgd2wgs.convert(lng, lat, height);
            });

            // Ensure that the source is JGD2011 and the destination is WGS 84
            assert_eq!(geom_store.epsg, EPSG_JGD2011_GEOGRAPHIC_3D);
            geom_store.epsg = EPSG_WGS84_GEOGRAPHIC_3D;
        }

        out.push(entity);
    }
}

impl ProjectionTransform {
    pub fn new(jgd2wgs: Arc<JGD2011ToWGS84>) -> Self {
        Self { jgd2wgs }
    }
}
