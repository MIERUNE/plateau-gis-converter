//! CityGML (.gml) Source Provider

use std::{
    fs,
    io::BufRead,
    path::{Path, PathBuf},
    sync::RwLock,
};

use nusamai_citygml::{CityGmlElement, CityGmlReader, Envelope, ParseError, SubTreeReader};
use nusamai_plateau::{appearance::AppearanceStore, models, Entity};
use rayon::prelude::*;
use url::Url;

use crate::{
    parameters::Parameters,
    pipeline::{self, Feedback, Parcel, PipelineError, Sender},
    source::{DataSource, DataSourceProvider, SourceInfo},
};

pub struct CityGmlSourceProvider {
    // FIXME: Use the configuration mechanism
    pub filenames: Vec<PathBuf>,
}

impl DataSourceProvider for CityGmlSourceProvider {
    fn create(&self, _params: &Parameters) -> Box<dyn DataSource> {
        Box::new(CityGmlSource {
            filenames: self.filenames.clone(),
            appearance_parsing: false,
        })
    }

    fn info(&self) -> SourceInfo {
        SourceInfo {
            name: "CityGML".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        Parameters::default()
    }
}

pub struct CityGmlSource {
    filenames: Vec<PathBuf>,
    appearance_parsing: bool,
}

impl DataSource for CityGmlSource {
    fn set_appearance_parsing(&mut self, value: bool) {
        self.appearance_parsing = value;
    }

    fn run(&mut self, downstream: Sender, feedback: &Feedback) -> pipeline::Result<()> {
        let code_resolver = nusamai_plateau::codelist::Resolver::new();

        self.filenames.par_iter().try_for_each(|filename| {
            feedback.ensure_not_canceled()?;

            feedback.info(format!("Parsing CityGML file: {:?} ...", filename));
            let file = std::fs::File::open(filename)?;
            let reader = std::io::BufReader::with_capacity(1024 * 1024, file);
            let mut xml_reader = quick_xml::NsReader::from_reader(reader);
            let source_url = Url::from_file_path(fs::canonicalize(Path::new(filename))?).unwrap();

            let context = nusamai_citygml::ParseContext::new(source_url.clone(), &code_resolver);
            let mut citygml_reader = CityGmlReader::new(context);

            let mut st = citygml_reader.start_root(&mut xml_reader)?;
            match toplevel_dispatcher(&mut st, &downstream, feedback, self.appearance_parsing) {
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
    parse_appearances: bool,
) -> Result<(), ParseError> {
    let mut entities = Vec::new();
    let mut global_appearances = AppearanceStore::default();
    let mut envelope = Envelope::default();

    st.parse_children(|st| {
        if feedback.is_canceled() {
            return Err(ParseError::Canceled);
        }

        match st.current_path() {
            b"gml:boundedBy" => {
                // skip
                Ok(())
            }
            b"gml:boundedBy/gml:Envelope" => {
                envelope.parse(st)?;
                Ok(())
            }
            b"core:cityObjectMember" => {
                let mut cityobj: models::TopLevelCityObject = Default::default();
                cityobj.parse(st)?;
                let geometry_store = st.collect_geometries(envelope.crs_uri.clone());

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

    if parse_appearances {
        for entity in entities {
            if feedback.is_canceled() {
                return Err(ParseError::Canceled);
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
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::sync_channel;

    use self::pipeline::feedback;
    use super::*;

    #[test]
    fn with_and_without_appearance() {
        for use_appearance in [false, true] {
            let (sender, receiver) = sync_channel(100);
            let source_provider = CityGmlSourceProvider {
                filenames: vec![PathBuf::from(
                    "../nusamai-plateau/tests/data/yokosuka-shi/udx/bldg/52397519_bldg_6697_op.gml",
                )],
            };
            let mut source = source_provider.create(&Parameters::default());
            source.set_appearance_parsing(use_appearance);
            let (_, feedback, _) = feedback::watcher();

            // Start the CityGML source
            std::thread::scope(|scope| {
                scope.spawn(move || {
                    source.run(sender, &feedback).unwrap();
                });

                let mut found_mat_or_tex = false;
                for parcel in receiver.iter() {
                    let appearance = parcel.entity.appearance_store.read().unwrap();
                    assert!(use_appearance || appearance.themes.is_empty());
                    assert!(use_appearance || appearance.textures.is_empty());
                    assert!(use_appearance || appearance.materials.is_empty());
                    found_mat_or_tex |= !appearance.materials.is_empty();
                    found_mat_or_tex |= !appearance.textures.is_empty();
                }
                assert_eq!(found_mat_or_tex, use_appearance);
            });
        }
    }
}
