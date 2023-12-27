use std::fs;
use std::io::BufRead;
use std::path::Path;
use url::Url;

use rayon::prelude::*;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Parcel, Sender};
use crate::source::{DataSource, DataSourceProvider, SourceInfo};
use nusamai_citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use nusamai_plateau::TopLevelCityObject;

pub struct CityGMLSourceProvider {
    // FIXME: Use the configuration mechanism
    pub filenames: Vec<String>,
}

impl DataSourceProvider for CityGMLSourceProvider {
    fn create(&self, _config: &Config) -> Box<dyn DataSource> {
        Box::new(CityGMLSource {
            filenames: self.filenames.clone(),
        })
    }

    fn info(&self) -> SourceInfo {
        SourceInfo {
            name: "Dummy Source".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

pub struct CityGMLSource {
    filenames: Vec<String>,
}

impl DataSource for CityGMLSource {
    fn run(&mut self, downstream: Sender, feedback: &Feedback) {
        let code_resolver = nusamai_plateau::codelist::Resolver::new();

        let _ = self.filenames.par_iter().try_for_each(|filename| {
            println!("loading city objects from: {} ...", filename);
            let Ok(file) = std::fs::File::open(filename) else {
                panic!("failed to open file {}", filename);
            };
            let reader = std::io::BufReader::new(file);
            let mut xml_reader = quick_xml::NsReader::from_reader(reader);
            let source_url =
                Url::from_file_path(fs::canonicalize(Path::new(filename)).unwrap()).unwrap();

            let context = nusamai_citygml::ParseContext::new(source_url, &code_resolver);
            let mut citygml_reader = CityGMLReader::new(context);

            match citygml_reader.start_root(&mut xml_reader) {
                Ok(mut st) => match toplevel_dispatcher(&mut st, &downstream, feedback) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        });
    }
}

fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
    sink: &Sender,
    feedback: &Feedback,
) -> Result<(), ParseError> {
    match st.parse_children(|st| {
        if feedback.is_cancelled() {
            return Err(ParseError::Cancelled);
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
                    let cityobj = TopLevelCityObject { root, geometries };
                    if sink.send(Parcel { cityobj }).is_err() {
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
    }) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}
