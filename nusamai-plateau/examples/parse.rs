use clap::Parser;

use std::io::BufRead;

use citygml::CityGMLElement;
use citygml::{CityGMLReader, ParseError, SubTreeReader};
use nusamai_plateau::models::Building;

fn example_toplevel_dispatcher<R: BufRead>(st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
    st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember/bldg:Building" => {
            let mut building: Building = Default::default();
            building.parse(st)?;
            println!("Building: {:#?}", building);
            Ok(())
        }
        _ => Ok(()),
    })
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let Ok(file) = std::fs::File::open(&args.filename) else {
        panic!("failed to open file {}", &args.filename);
    };
    let mut xml_reader = quick_xml::NsReader::from_reader(std::io::BufReader::new(file));

    match CityGMLReader::new().start_root(&mut xml_reader) {
        Ok(Some(mut st)) => match example_toplevel_dispatcher(&mut st) {
            Ok(_) => (),
            Err(e) => panic!("Err: {:?}", e),
        },
        Ok(None) => (),
        Err(e) => panic!("Err: {:?}", e),
    }
}
