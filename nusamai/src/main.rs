use std::io::BufRead;

use clap::Parser;
use rayon::{prelude::*, ThreadPoolBuilder};

use citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use nusamai::configuration::Config;
use nusamai::pipeline::{self, TransformError};
use nusamai::pipeline::{
    feedback, Feedback, Percel, Sender, Sink, SinkInfo, SinkProvider, Source, SourceInfo,
    SourceProvider, Transformer,
};
use nusamai_plateau::TopLevelCityObject;

pub struct CityGMLSourceProvider {
    // FIXME: Use the configuration mechanism
    filenames: Vec<String>,
}

impl SourceProvider for CityGMLSourceProvider {
    fn create(&self, _config: &Config) -> Box<dyn Source> {
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

impl Source for CityGMLSource {
    fn run(&mut self, sink: Sender, feedback: &Feedback) {
        let pool = ThreadPoolBuilder::new().build().unwrap();
        pool.install(|| {
            self.filenames.par_iter().for_each(|filename| {
                println!("loading city objects from: {} ...", filename);

                let Ok(file) = std::fs::File::open(filename) else {
                    panic!("failed to open file {}", filename);
                };
                let reader = std::io::BufReader::new(file);
                let mut xml_reader = quick_xml::NsReader::from_reader(reader);
                match CityGMLReader::new().start_root(&mut xml_reader) {
                    Ok(mut st) => match toplevel_dispatcher(&mut st, &sink, feedback) {
                        Ok(size) => size,
                        Err(e) => panic!("Err: {:?}", e),
                    },
                    Err(e) => panic!("Err: {:?}", e),
                };
            });
        })
    }
}

fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
    sink: &Sender,
    feedback: &Feedback,
) -> Result<(), ParseError> {
    match st.parse_children(|st| {
        if feedback.is_cancelled() {
            return Ok(());
        }

        match st.current_path() {
            b"core:cityObjectMember" => {
                let mut cityobj: nusamai_plateau::models::CityObject = Default::default();
                cityobj.parse(st)?;
                let geometries = st.collect_geometries();

                if let Some(root) = cityobj.into_object() {
                    let cityobj = TopLevelCityObject { root, geometries };
                    if sink.send(Percel { cityobj }).is_err() {
                        return Ok(());
                    }
                }

                Ok(())
            }
            b"gml:boundedBy" | b"app:appearanceMember" => {
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

struct NoopTransformer {}

impl Transformer for NoopTransformer {
    fn transform(
        &self,
        percel: Percel,
        sender: &Sender,
        _feedback: &feedback::Feedback,
    ) -> Result<(), TransformError> {
        // no-op
        sender.send(percel)?;
        Ok(())
    }
}

struct DummySinkProvider {}

impl SinkProvider for DummySinkProvider {
    fn create(&self, _config: &Config) -> Box<dyn Sink> {
        Box::new(DummySink {
            num_features: 0,
            num_vertices: 0,
        })
    }

    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "Noop Sink".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

struct DummySink {
    num_features: usize,
    num_vertices: usize,
}

impl Sink for DummySink {
    fn receive(&mut self, percel: Percel, _feedback: &mut Feedback) {
        self.num_features += 1;
        self.num_vertices += percel.cityobj.geometries.vertices.len();
    }

    fn finalize(&mut self, _feedback: &mut Feedback) {
        println!("total number of features: {:#?}", self.num_features);
        println!("total vertices: {}", self.num_vertices);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    filenames: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let input_driver_factory: Box<dyn SourceProvider> = Box::new(CityGMLSourceProvider {
        filenames: args.filenames,
    });
    let output_driver_factory: Box<dyn SinkProvider> = Box::new(DummySinkProvider {});

    let input_driver = input_driver_factory.create(&input_driver_factory.config());
    let transformer = Box::new(NoopTransformer {});
    let output_driver = output_driver_factory.create(&input_driver_factory.config());

    // start the pipeline
    let (handle, watcher, _canceller) = pipeline::run(input_driver, transformer, output_driver);

    std::thread::scope(|scope| {
        // log watcher
        scope.spawn(move || {
            for msg in watcher {
                println!("Feedback message from the pipeline {:?}", msg);
            }
        });
    });

    // wait for the pipeline to finish
    handle.join();
}
