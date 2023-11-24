use clap::Parser;

use std::io::{BufRead, BufReader, Write};

use citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use nusamai_plateau::models::CityObject;

fn example_toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
    let bincode_config = bincode::config::standard();

    st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: CityObject = Default::default();
            cityobj.parse(st)?;

            // print top-level city object
            println!("TLCO: {:#?}", cityobj);

            // serialize to bincode
            let serialized = bincode::serde::encode_to_vec(cityobj, bincode_config).unwrap();

            println!("bin_size={}", serialized.len());

            let mut compressed = Vec::new();
            flate2::write::GzEncoder::new(&mut compressed, flate2::Compression::fast())
                .write_all(&serialized)
                .unwrap();

            println!(
                "bin_size={} compressed={}",
                serialized.len(),
                compressed.len()
            );

            Ok(())
        }
        _ => Ok(()),
    })
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    filenames: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for filename in &args.filenames {
        let Ok(file) = std::fs::File::open(filename) else {
            panic!("failed to open file {}", filename);
        };
        let mut xml_reader = quick_xml::NsReader::from_reader(BufReader::new(file));

        match CityGMLReader::new().start_root(&mut xml_reader) {
            Ok(mut st) => match example_toplevel_dispatcher(&mut st) {
                Ok(_) => (),
                Err(e) => panic!("Err: {:?}", e),
            },
            Err(e) => panic!("Err: {:?}", e),
        }
    }
}
