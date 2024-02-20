//! CityGML (.gml) Source Provider

use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use rayon::prelude::*;
use url::Url;

use crate::parameters::Parameters;
use crate::pipeline::{self, PipelineError};
use crate::pipeline::{Feedback, Parcel, Sender};
use crate::source::{DataSource, DataSourceProvider, SourceInfo};
use nusamai_citygml::{CityGmlElement, CityGmlReader, ParseError, SubTreeReader};
use nusamai_plateau::appearance::AppearanceStore;
use nusamai_plateau::models;
use nusamai_plateau::Entity;

pub struct CityGmlSourceProvider {
    // FIXME: Use the configuration mechanism
    pub filenames: Vec<PathBuf>,
}

impl DataSourceProvider for CityGmlSourceProvider {
    fn create(&self, _params: &Parameters) -> Box<dyn DataSource> {
        Box::new(CityGmlSource {
            filenames: self.filenames.clone(),
        })
    }

    fn info(&self) -> SourceInfo {
        SourceInfo {
            name: "CityGML".to_string(),
        }
    }

    fn parameters(&self) -> Parameters {
        Parameters::default()
    }
}

pub struct CityGmlSource {
    filenames: Vec<PathBuf>,
}

impl DataSource for CityGmlSource {
    fn run(&mut self, downstream: Sender, feedback: &Feedback) -> pipeline::Result<()> {
        let code_resolver = nusamai_plateau::codelist::Resolver::new();

        self.filenames.par_iter().try_for_each(|filename| {
            feedback.ensure_not_canceled()?;

            log::info!("Parsing CityGML file: {:?} ...", filename);
            let file = std::fs::File::open(filename)?;
            let reader = std::io::BufReader::with_capacity(1024 * 1024, file);
            let mut xml_reader = quick_xml::NsReader::from_reader(reader);
            let source_url = Url::from_file_path(fs::canonicalize(Path::new(filename))?).unwrap();

            let context = nusamai_citygml::ParseContext::new(source_url.clone(), &code_resolver);
            let mut citygml_reader = CityGmlReader::new(context);

            let mut st = citygml_reader.start_root(&mut xml_reader)?;
            match toplevel_dispatcher(&mut st, &downstream, feedback) {
                Ok(_) => Ok::<(), PipelineError>(()),
                Err(ParseError::Canceled) => Err(PipelineError::Canceled),
                Err(e) => Err(e.into()),
            }
        })?;

        Ok(())
    }
}

// TODO: Move this to nusamai-plateau ?
fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
    downstream: &Sender,
    feedback: &Feedback,
) -> Result<(), ParseError> {
    let parse_appearances = true;
    let mut entities = Vec::new();
    let mut global_appearances = AppearanceStore::default();

    st.parse_children(|st| {
        if feedback.is_canceled() {
            return Err(ParseError::Canceled);
        }

        match st.current_path() {
            b"gml:boundedBy" => {
                st.skip_current_element()?;
                Ok(())
            }
            b"core:cityObjectMember" => {
                let mut cityobj: models::TopLevelCityObject = Default::default();
                cityobj.parse(st)?;
                let geometry_store = st.collect_geometries();

                if let Some(root) = cityobj.into_object() {
                    let entity = Entity {
                        root,
                        base_url: url::Url::parse("file:///dummy").unwrap(),
                        geometry_store: RwLock::new(geometry_store).into(),
                        appearance_store: Default::default(), // TODO: from local appearances
                    };

                    if parse_appearances {
                        // store the entity to bind the appearance later
                        entities.push(entity);
                    } else {
                        // send the entity immediately
                        if downstream.send(Parcel { entity }).is_err() {
                            feedback.cancel();
                            return Ok(());
                        }
                    }
                }
                Ok(())
            }
            b"app:appearanceMember" => {
                if parse_appearances {
                    let mut app: models::appearance::AppearanceProperty = Default::default();
                    app.parse(st)?;
                    let models::appearance::AppearanceProperty::Appearance(app) = app else {
                        unreachable!();
                    };
                    global_appearances.update(app);
                } else {
                    st.skip_current_element()?;
                }
                Ok(())
            }
            other => Err(ParseError::SchemaViolation(format!(
                "Unrecognized element {}",
                String::from_utf8_lossy(other)
            ))),
        }
    })?;

    for entity in entities {
        if feedback.is_canceled() {
            break;
        }

        // merge global appearances into the entity's local appearance store
        {
            let geom_store = entity.geometry_store.read().unwrap();
            entity.appearance_store.write().unwrap().merge_global(
                &mut global_appearances,
                &geom_store.ring_ids,
                &geom_store.surface_spans,
            );
        }

        if downstream.send(Parcel { entity }).is_err() {
            feedback.cancel();
            break;
        }
    }

    Ok(())
}
