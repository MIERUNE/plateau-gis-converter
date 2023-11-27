use clap::Parser;

use serde::{Deserialize, Serialize};
use std::io::BufRead;

use citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use nusamai_plateau::models::CityObject;

#[derive(Debug, Deserialize, Serialize)]
struct TopLevelCityObject {
    cityobj: CityObject,
    geometries: citygml::Geometries,
}

fn example_toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<(usize, usize), ParseError> {
    let bincode_config = bincode::config::standard();
    let mut uncompressed_size = 0;
    let mut compressed_size = 0;

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let toplevel_cityobj = {
                let mut cityobj: CityObject = Default::default();
                cityobj.parse(st)?;
                let geometries = st.collect_geometries();

                TopLevelCityObject {
                    cityobj,
                    geometries,
                }
            };

            println!(
                "vertices={} polygons={}",
                toplevel_cityobj.geometries.vertices.len(),
                toplevel_cityobj.geometries.polygons.len()
            );

            // print top-level city object
            // println!("TLCO: {:#?}", toplevel_cityobj);

            println!("{}", serde_json::to_string(&toplevel_cityobj).unwrap());

            // serialize to bincode
            let serialized =
                bincode::serde::encode_to_vec(&toplevel_cityobj, bincode_config).unwrap();

            let compressed =
                zstd::stream::encode_all(std::io::Cursor::new(&serialized), 3).unwrap();

            println!(
                "bin_size={} compressed={}",
                serialized.len(),
                compressed.len()
            );

            uncompressed_size += serialized.len();
            compressed_size += compressed.len();
            Ok(())
        }
        other => {
            println!("skipping {}", std::str::from_utf8(other).unwrap_or("???"));
            st.skip_current_element()?;
            Ok(())
        }
    }) {
        Ok(_) => Ok((uncompressed_size, compressed_size)),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
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

    for filename in &args.filenames {
        let (original_xml_size, compressed_xml_size, reader) = {
            let Ok(xml_data) = std::fs::read(filename) else {
                panic!("failed to open file {}", filename);
            };
            let compressed = zstd::stream::encode_all(std::io::Cursor::new(&xml_data), 3).unwrap();
            (
                xml_data.len(),
                compressed.len(),
                std::io::Cursor::new(xml_data),
            )
        };

        let mut xml_reader = quick_xml::NsReader::from_reader(reader);
        let (uncompressed_size, compressed_size) =
            match CityGMLReader::new().start_root(&mut xml_reader) {
                Ok(mut st) => match example_toplevel_dispatcher(&mut st) {
                    Ok(size) => size,
                    Err(e) => panic!("Err: {:?}", e),
                },
                Err(e) => panic!("Err: {:?}", e),
            };

        println!(
            "uncompressed_binary={:.2} (MB)",
            original_xml_size as f32 / 1024. / 1024.
        );
        println!(
            "compressed_xml={:.2} (MB)",
            compressed_xml_size as f32 / 1024. / 1024.
        );
        println!(
            "uncompressed_binary={:.2} (MB)",
            uncompressed_size as f32 / 1024. / 1024.
        );
        println!(
            "compressed_binary={:.2} (MB)",
            compressed_size as f32 / 1024. / 1024.
        );
    }
}
