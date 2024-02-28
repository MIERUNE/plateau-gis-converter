use std::sync::Arc;

use crate::transformer::Transform;

use nusamai_citygml::schema::Schema;
use nusamai_plateau::Entity;
use nusamai_projection::crs::*;
use nusamai_projection::vshift::Jgd2011ToWgs84;

/// Coordinate transformation. Currently supports only JGD2011 to WGS 84.
#[derive(Clone)]
pub struct ProjectionTransform {
    output_epsg: EPSGCode,
    jgd2wgs: Arc<Jgd2011ToWgs84>,
}

impl Transform for ProjectionTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        let input_epsg = {
            let geom_store = entity.geometry_store.read().unwrap();
            geom_store.epsg
        };

        // TODO: From: JGD2011 To: WGS 84 のみを想定している。他のCRSにも今後対応する。

        match input_epsg {
            EPSG_JGD2011_GEOGRAPHIC_3D => {
                match self.output_epsg {
                    EPSG_WGS84_GEOGRAPHIC_3D => {
                        let mut geom_store = entity.geometry_store.write().unwrap();
                        geom_store.vertices.iter_mut().for_each(|v| {
                            // Swap x and y (lat, lng -> lng, lat)
                            let (lng, lat, height) = (v[1], v[0], v[2]);
                            // JGD2011 to WGS 84 (elevation to ellipsoidal height)
                            (v[0], v[1], v[2]) = self.jgd2wgs.convert(lng, lat, height);
                        });
                        geom_store.epsg = EPSG_WGS84_GEOGRAPHIC_3D;
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
                        unimplemented!("JPR conversion is not implemented yet.");
                    }
                    _ => {
                        panic!("Unsupported output CRS: {}", self.output_epsg);
                    }
                }
            }
            EPSG_WGS84_GEOGRAPHIC_3D => match self.output_epsg {
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
                    unimplemented!("JPR conversion is not implemented yet.");
                }
                _ => {
                    panic!("Unsupported output CRS: {}", self.output_epsg);
                }
            },
            _ => {
                panic!("Unsupported input CRS: {}", input_epsg);
            }
        }

        out.push(entity);
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // TODO: Change the CRS information in the schema (?)
    }
}

impl ProjectionTransform {
    pub fn new(output_epsg: EPSGCode) -> Self {
        Self {
            output_epsg: EPSG_WGS84_GEOGRAPHIC_3D,
            jgd2wgs: Jgd2011ToWgs84::default().into(),
        }
    }
}
