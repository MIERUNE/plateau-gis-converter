//! CityGML (.gml) Source Provider

use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::sync::RwLock;
use url::Url;

use rayon::prelude::*;

use crate::parameters::Parameters;
use crate::pipeline;
use crate::pipeline::{Feedback, Parcel, Sender};
use crate::source::{DataSource, DataSourceProvider, SourceInfo};
use nusamai_citygml::object::Entity;
use nusamai_citygml::{CityGmlElement, CityGmlReader, ParseError, SubTreeReader};

pub struct CityGmlSourceProvider {
    // FIXME: Use the configuration mechanism
    pub filenames: Vec<String>,
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
    filenames: Vec<String>,
}

impl DataSource for CityGmlSource {
    fn run(&mut self, downstream: Sender, feedback: &Feedback) -> pipeline::Result<()> {
        let code_resolver = nusamai_plateau::codelist::Resolver::new();

        self.filenames.par_iter().try_for_each(|filename| {
            log::info!("loading city objects from: {} ...", filename);
            let file = std::fs::File::open(filename).unwrap();
            let reader = std::io::BufReader::with_capacity(1024 * 1024, file);
            let mut xml_reader = quick_xml::NsReader::from_reader(reader);
            let source_url =
                Url::from_file_path(fs::canonicalize(Path::new(filename)).unwrap()).unwrap();

            let context = nusamai_citygml::ParseContext::new(source_url, &code_resolver);
            let mut citygml_reader = CityGmlReader::new(context);

            match citygml_reader.start_root(&mut xml_reader) {
                Ok(mut st) => match toplevel_dispatcher(&mut st, &downstream, feedback) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        })?;

        Ok(())
    }
}

fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
    downstream: &Sender,
    feedback: &Feedback,
) -> Result<(), ParseError> {
    let result = st.parse_children(|st| {
        if feedback.is_canceled() {
            return Ok(());
        }

        match st.current_path() {
            b"gml:boundedBy" => {
                st.skip_current_element()?;
                Ok(())
            }
            b"core:cityObjectMember" => {
                let mut cityobj: nusamai_plateau::models::TopLevelCityObject = Default::default();
                cityobj.parse(st)?;
                let geometries = st.collect_geometries();

                if let Some(root) = cityobj.into_object() {
                    let entity = Entity {
                        root,
                        geometry_store: RwLock::new(geometries).into(),
                    };
                    if downstream.send(Parcel { entity }).is_err() {
                        feedback.cancel();
                        return Ok(());
                    }
                }
                Ok(())
            }
            b"app:appearanceMember" => {
                st.skip_current_element()?;
                Ok(())
            }
            other => Err(ParseError::SchemaViolation(format!(
                "Unrecognized element {}",
                String::from_utf8_lossy(other)
            ))),
        }
    });
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
