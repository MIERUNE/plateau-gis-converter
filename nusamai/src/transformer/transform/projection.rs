use std::sync::Arc;

use nusamai_citygml::schema::Schema;
use nusamai_plateau::Entity;
use nusamai_projection::{
    crs::*, etmerc::ExtendedTransverseMercatorProjection, jprect::JPRZone, vshift::Jgd2011ToWgs84,
};

use crate::{pipeline::Feedback, transformer::Transform};

/// Coordinate transformation
#[derive(Clone)]
pub struct ProjectionTransform {
    jgd2wgs: Arc<Jgd2011ToWgs84>,
    output_epsg: EpsgCode,
    jpr_zone_proj: Option<ExtendedTransverseMercatorProjection>,
}

impl Transform for ProjectionTransform {
    fn transform(&mut self, _feedback: &Feedback, entity: Entity, out: &mut Vec<Entity>) {
        let input_epsg = {
            let geom_store = entity.geometry_store.read().unwrap();
            geom_store.epsg
        };

        match input_epsg {
            EPSG_JGD2011_GEOGRAPHIC_3D => self.transform_from_jgd2011(&entity, None),
            EPSG_WGS84_GEOGRAPHIC_3D => self.transform_from_wgs84(&entity, None),
            EPSG_JGD2011_JPRECT_I_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_II_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_III_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_IV_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_V_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VI_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VIII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_IX_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_X_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XI_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XIII_JGD2011_HEIGHT => {
                self.transform_from_jgd2011(&entity, Some(input_epsg));
            }
            _ => {
                panic!("Unsupported input CRS: {}", input_epsg);
            }
        }

        out.push(entity);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        schema.epsg = Some(self.output_epsg);
    }
}

impl ProjectionTransform {
    pub fn new(jgd2wgs: Arc<Jgd2011ToWgs84>, output_epsg: EpsgCode) -> Self {
        // For Japan Plane Rectangular CS
        let jpr_zone_proj = JPRZone::from_epsg(output_epsg).map(|zone| zone.projection());

        Self {
            jgd2wgs,
            output_epsg,
            jpr_zone_proj,
        }
    }

    fn transform_from_jgd2011(&mut self, entity: &Entity, rectangular: Option<EpsgCode>) {
        let pre_transform = |v: &mut [f64; 3]| {
            if let Some(rectangular_input_epsg) = rectangular {
                let zone = JPRZone::from_epsg(rectangular_input_epsg).unwrap();
                let proj = zone.projection();
                let (lng, lat, z) = proj.project_inverse(v[0], v[1], v[2]).unwrap();
                (v[0], v[1], v[2]) = (lng, lat, z);
            }
        };
        match self.output_epsg {
            EPSG_JGD2011_GEOGRAPHIC_3D => {
                let mut geom_store = entity.geometry_store.write().unwrap();
                geom_store.vertices.iter_mut().for_each(|v| {
                    pre_transform(v);
                    // Swap x and y (lat, lng -> lng, lat)
                    (v[0], v[1], v[2]) = (v[1], v[0], v[2]);
                });
                geom_store.epsg = self.output_epsg;
            }
            EPSG_WGS84_GEOGRAPHIC_3D => {
                let mut geom_store = entity.geometry_store.write().unwrap();
                geom_store.vertices.iter_mut().for_each(|v| {
                    pre_transform(v);
                    // Swap x and y (lat, lng -> lng, lat)
                    let (lng, lat, height) = (v[1], v[0], v[2]);
                    // JGD2011 to WGS 84 (elevation to ellipsoidal height)
                    (v[0], v[1], v[2]) = self.jgd2wgs.convert(lng, lat, height);
                });
                geom_store.epsg = self.output_epsg;
            }
            EPSG_WEB_MERCATOR => {
                let mut geom_store = entity.geometry_store.write().unwrap();
                geom_store.vertices.iter_mut().for_each(|v| {
                    pre_transform(v);
                    // Swap x and y (lat, lng -> lng, lat)
                    let (lng, lat) = (v[1], v[0]);
                    // LngLat to Web Mercator
                    (v[0], v[1]) = nusamai_mvt::webmercator::lnglat_to_web_mercator_meters(lng, lat)
                });
                geom_store.epsg = self.output_epsg;
            }
            EPSG_JGD2011_JPRECT_I_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_II_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_III_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_IV_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_V_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VI_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VIII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_IX_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_X_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XI_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XIII_JGD2011_HEIGHT => {
                // To Japan Plane Rectangular CS + JGD2011 (vertical) height
                let proj = self.jpr_zone_proj.as_ref().unwrap();
                let mut geom_store = entity.geometry_store.write().unwrap();
                geom_store.vertices.iter_mut().for_each(|v| {
                    pre_transform(v);
                    let (lng, lat) = (v[1], v[0]);
                    // Change x and y; keep the height
                    // TODO: error handling
                    (v[0], v[1], _) = proj.project_forward(lng, lat, 0.).unwrap();
                });
                geom_store.epsg = self.output_epsg;
            }
            EPSG_JGD2011_JPRECT_I
            | EPSG_JGD2011_JPRECT_II
            | EPSG_JGD2011_JPRECT_III
            | EPSG_JGD2011_JPRECT_IV
            | EPSG_JGD2011_JPRECT_V
            | EPSG_JGD2011_JPRECT_VI
            | EPSG_JGD2011_JPRECT_VII
            | EPSG_JGD2011_JPRECT_VIII
            | EPSG_JGD2011_JPRECT_IX
            | EPSG_JGD2011_JPRECT_X
            | EPSG_JGD2011_JPRECT_XI
            | EPSG_JGD2011_JPRECT_XII
            | EPSG_JGD2011_JPRECT_XIII
            | EPSG_JGD2011_JPRECT_XIV
            | EPSG_JGD2011_JPRECT_XV
            | EPSG_JGD2011_JPRECT_XVI
            | EPSG_JGD2011_JPRECT_XVII
            | EPSG_JGD2011_JPRECT_XVIII
            | EPSG_JGD2011_JPRECT_XIX => {
                // To Japan Plane Rectangular CS
                let proj = self.jpr_zone_proj.as_ref().unwrap();
                let mut geom_store = entity.geometry_store.write().unwrap();
                geom_store.vertices.iter_mut().for_each(|v| {
                    pre_transform(v);
                    let (lng, lat) = (v[1], v[0]);
                    // Change x and y; keep the height
                    // TODO: error handling
                    (v[0], v[1], _) = proj.project_forward(lng, lat, 0.).unwrap();
                });
                geom_store.epsg = self.output_epsg;
            }
            _ => {
                panic!("Unsupported output CRS: {}", self.output_epsg);
            }
        };
    }

    fn transform_from_wgs84(&mut self, _entity: &Entity, _rectangular: Option<EpsgCode>) {
        match self.output_epsg {
            EPSG_WGS84_GEOGRAPHIC_3D => {
                // Do nothing
            }
            EPSG_JGD2011_JPRECT_I_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_II_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_III_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_IV_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_V_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VI_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_VIII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_IX_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_X_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XI_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_XIII_JGD2011_HEIGHT => {
                // TODO: implement
                unimplemented!("WGS84 to EPSG:{} not supported yet", self.output_epsg);
            }
            EPSG_JGD2011_JPRECT_I
            | EPSG_JGD2011_JPRECT_II
            | EPSG_JGD2011_JPRECT_III
            | EPSG_JGD2011_JPRECT_IV
            | EPSG_JGD2011_JPRECT_V
            | EPSG_JGD2011_JPRECT_VI
            | EPSG_JGD2011_JPRECT_VII
            | EPSG_JGD2011_JPRECT_VIII
            | EPSG_JGD2011_JPRECT_IX
            | EPSG_JGD2011_JPRECT_X
            | EPSG_JGD2011_JPRECT_XI
            | EPSG_JGD2011_JPRECT_XII
            | EPSG_JGD2011_JPRECT_XIII
            | EPSG_JGD2011_JPRECT_XIV
            | EPSG_JGD2011_JPRECT_XV
            | EPSG_JGD2011_JPRECT_XVI
            | EPSG_JGD2011_JPRECT_XVII
            | EPSG_JGD2011_JPRECT_XVIII
            | EPSG_JGD2011_JPRECT_XIX => {
                // TODO: implement
                unimplemented!("WGS84 to EPSG:{} not supported yet", self.output_epsg);
            }
            _ => {
                panic!("Unsupported output CRS: {}", self.output_epsg);
            }
        }
    }
}
