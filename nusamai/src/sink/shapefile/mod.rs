//! Shapefile sink

mod attributes;
mod crs;
mod null_shape;

use std::{
    fs::{remove_file, File},
    io::BufWriter,
    path::PathBuf,
};

use attributes::{attributes_to_record, make_table_builder};
use indexmap::IndexMap;
use nusamai_citygml::{
    object::{Map, ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};
use nusamai_plateau::Entity;
use nusamai_shapefile::conversion::indexed_multipolygon_to_shape;
use rayon::iter::{ParallelBridge, ParallelIterator};

use self::crs::ProjectionRepository;
use crate::{
    get_parameter_value,
    option::use_lod_config,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
    transformer::TransformerRegistry,
};

use super::option::output_parameter;

pub struct ShapefileSinkProvider {}

impl DataSinkProvider for ShapefileSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "shapefile".to_string(),
            name: "Shapefile".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());
        params
    }

    fn transformer_options(&self) -> TransformerRegistry {
        let mut settings: TransformerRegistry = TransformerRegistry::new();
        settings.insert(use_lod_config("max_lod"));

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_settings = self.transformer_options();

        Box::<ShapefileSink>::new(ShapefileSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings,
        })
    }
}

pub struct ShapefileSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
}

impl DataSink for ShapefileSink {
    fn make_requirements(&mut self, properties: TransformerRegistry) -> DataRequirements {
        let default_requirements = DataRequirements {
            shorten_names_for_shapefile: true,
            tree_flattening: transformer::TreeFlatteningSpec::Flatten {
                feature: transformer::FeatureFlatteningOption::AllExceptThematicSurfaces,
                data: transformer::DataFlatteningOption::TopLevelOnly,
                object: transformer::ObjectFlatteningOption::None,
            },

            ..Default::default()
        };

        for config in properties.configs.iter() {
            let _ = &self.transform_settings.update_transformer(config.clone());
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        let (ra, rb) = rayon::join(
            || {
                // Convert CityObjects to Shapefile objects

                upstream
                    .into_iter()
                    .par_bridge()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let Value::Object(object) = &parcel.entity.root else {
                            return Ok(());
                        };
                        let typename = object.typename.clone();

                        let (shape, attributes) = entity_to_shape(parcel.entity);

                        if sender.send((typename, shape, attributes)).is_err() {
                            return Err(PipelineError::Canceled);
                        };

                        Ok(())
                    })
            },
            || {
                // Write Shapefile to a file

                // Attribute fields for the features
                // FieldName byte representation cannot exceed 11 bytes
                let mut grouped_features = IndexMap::<String, Vec<(shapefile::Shape, Map)>>::new();

                receiver
                    .into_iter()
                    .for_each(|(typename, shape, attributes)| {
                        grouped_features
                            .entry(typename.to_string())
                            .or_default()
                            .push((shape, attributes));
                    });

                // Write a Shapefile file set for each typename
                grouped_features
                    .into_iter()
                    .try_for_each(|(typename, features)| {
                        feedback.ensure_not_canceled()?;

                        let typedef = schema.types.get(&typename).ok_or_else(|| {
                            PipelineError::Other(format!(
                                "Type {} not found in the schema",
                                typename
                            ))
                        })?;

                        let (table_builder, fields_default) = make_table_builder(typedef);

                        // Create all the files needed for the shapefile to be complete (.shp, .shx, .dbf)
                        std::fs::create_dir_all(&self.output_path)?;
                        let shp_path = self
                            .output_path
                            .join(format!("{}.shp", typename.replace(':', "_")));

                        let feature_count = features.len();
                        let has_no_geometry = features
                            .iter()
                            .all(|(shape, _)| matches!(shape, shapefile::Shape::NullShape));

                        // NOTE: Need to be scoped to drop the writer before removing .shp/.shx
                        {
                            let mut writer = shapefile::Writer::from_path(&shp_path, table_builder)
                                .map_err(|err| match err {
                                    shapefile::Error::IoError(io_err) => {
                                        PipelineError::IoError(io_err)
                                    }
                                    _ => PipelineError::Other(err.to_string()),
                                })?;

                            // Write each feature
                            for (shape, attributes) in features {
                                let record = attributes_to_record(attributes, &fields_default);

                                match shape {
                                    shapefile::Shape::PolygonZ(polygon) => {
                                        writer.write_shape_and_record(&polygon, &record).map_err(
                                            |err| match err {
                                                shapefile::Error::IoError(io_err) => {
                                                    PipelineError::IoError(io_err)
                                                }
                                                _ => PipelineError::Other(err.to_string()),
                                            },
                                        )?;
                                    }
                                    shapefile::Shape::NullShape if !has_no_geometry => {
                                        // FIXME: feature may have no geometry. e.g.
                                        // - Building (no geometry)
                                        //     - BuildingPart (has geometry)
                                        //     - BuildingPart (has geometry)
                                        feedback.warn(
                                            "Feature without geometry is not supported yet."
                                                .to_string(),
                                        );
                                    }
                                    shapefile::Shape::NullShape if has_no_geometry => {
                                        // Write dummy data once because shapefile-rs cannot write NullShape file
                                        let point = shapefile::Point::default();
                                        writer.write_shape_and_record(&point, &record).map_err(
                                            |err| match err {
                                                shapefile::Error::IoError(io_err) => {
                                                    PipelineError::IoError(io_err)
                                                }
                                                _ => PipelineError::Other(err.to_string()),
                                            },
                                        )?;
                                    }
                                    _ => {
                                        log::warn!("Unsupported shape type");
                                    }
                                }
                            }
                        }

                        // If this type has no geometry (i.e. Data or Object stereotype)
                        if has_no_geometry {
                            // Remove dummy .shp and .shx and write a NullShape file.
                            remove_file(&shp_path)?;
                            let shx_path = shp_path.with_extension("shx");
                            remove_file(&shx_path)?;
                            null_shape::write_shp(
                                BufWriter::new(File::create(shp_path)?),
                                feature_count,
                            )?;
                            null_shape::write_shx(
                                BufWriter::new(File::create(shx_path)?),
                                feature_count,
                            )?;
                        } else {
                            // write .prj file if this type has geometry

                            let repo = ProjectionRepository::new();
                            let prj_path = &shp_path.with_extension("prj");
                            crs::write_prj(
                                BufWriter::new(File::create(prj_path)?),
                                &repo,
                                &schema.epsg.unwrap(),
                            )?;
                        }

                        Ok::<(), PipelineError>(())
                    })
            },
        );

        match ra {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(err) => feedback.fatal_error(err),
        }
        match rb {
            Ok(_) => {}
            Err(err) => feedback.fatal_error(err),
        }

        Ok(())
    }
}

/// Create Shapefile features from a Entity
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
/// TODO: Implement MultiLineString and MultiPoint handling
pub fn entity_to_shape(entity: Entity) -> (shapefile::Shape, Map) {
    let Value::Object(mut obj) = entity.root else {
        return (shapefile::Shape::NullShape, Map::default());
    };

    let ObjectStereotype::Feature { id, geometries } = obj.stereotype else {
        return (shapefile::Shape::NullShape, obj.attributes);
    };

    // Insert Feature id as a attribute
    obj.attributes.insert("id".to_string(), Value::String(id));

    let geom_store = entity.geometry_store.read().unwrap();

    let mut mpoly = flatgeom::MultiPolygon::<u32>::new();

    geometries.iter().for_each(|entry| match entry.ty {
        GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
            for idx_poly in geom_store
                .multipolygon
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                mpoly.push(&idx_poly);
            }
        }
        GeometryType::Curve => unimplemented!(),
        GeometryType::Point => unimplemented!(),
    });

    if !mpoly.is_empty() {
        let shape =
            shapefile::Shape::PolygonZ(indexed_multipolygon_to_shape(&geom_store.vertices, &mpoly));

        return (shape, obj.attributes);
    }

    (shapefile::Shape::NullShape, obj.attributes)
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use flatgeom::MultiPolygon;
    use nusamai_citygml::{object::Object, GeometryRef};
    use nusamai_projection::crs::EPSG_JGD2011_GEOGRAPHIC_3D;
    use shapefile::NO_DATA;

    use super::*;

    #[test]
    fn test_toplevel_cityobj_multipolygon() {
        let vertices: Vec<[f64; 3]> = vec![
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
        ];
        let mut mpoly = MultiPolygon::<u32>::new();
        mpoly.add_exterior([0, 1, 2, 3, 0]);
        let geometries = nusamai_citygml::GeometryStore {
            epsg: EPSG_JGD2011_GEOGRAPHIC_3D,
            vertices,
            multipolygon: mpoly,
            ..Default::default()
        };

        let obj = Entity {
            root: Value::Object(Object {
                typename: "dummy".into(),
                attributes: Default::default(),
                stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                    id: "dummy".into(),
                    geometries: vec![GeometryRef {
                        ty: GeometryType::Solid,
                        pos: 0,
                        len: 1,
                        lod: 1,
                    }],
                },
            }),
            base_url: url::Url::parse("file:///dummy").unwrap(),
            geometry_store: RwLock::new(geometries).into(),
            appearance_store: Default::default(),
        };

        let (shapes, attributes) = entity_to_shape(obj);
        assert_eq!(
            attributes.get("id").unwrap(),
            &Value::String("dummy".into())
        );

        if let shapefile::Shape::PolygonZ(polygon) = &shapes {
            assert_eq!(polygon.rings().len(), 1);
            assert_eq!(
                polygon.ring(0).unwrap(),
                &shapefile::PolygonRing::Outer(vec![
                    // Outer ring: re-ordered to clockwise
                    shapefile::PointZ::new(0., 0., 111., NO_DATA),
                    shapefile::PointZ::new(0., 5., 111., NO_DATA),
                    shapefile::PointZ::new(5., 5., 111., NO_DATA),
                    shapefile::PointZ::new(5., 0., 111., NO_DATA),
                    shapefile::PointZ::new(0., 0., 111., NO_DATA), // automatically closed
                ])
            )
        } else {
            panic!("Expected PolygonZ.");
        }
    }
}
