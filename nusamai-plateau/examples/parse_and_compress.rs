use std::{io::BufRead, time::Instant};

use clap::Parser;
use nusamai_citygml::{object::Value, CityGmlElement, CityGmlReader, ParseError, SubTreeReader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TopLevelCityObject {
    root: Value,
    geometries: nusamai_citygml::GeometryStore,
}

fn example_toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
    encoded_data: &mut Vec<u8>,
    encoded_sizes: &mut Vec<usize>,
) -> Result<(), ParseError> {
    let bincode_params = bincode::config::standard();

    let parse_result = {
        let path: &[u8] = &st.current_path();
        match path {
            b"core:cityObjectMember" => {
                let mut cityobj: nusamai_plateau::models::TopLevelCityObject = Default::default();
                cityobj.parse(st)?;
                let geometries = st.collect_geometries(None);

                if let Some(root) = cityobj.into_object() {
                    let obj = self::TopLevelCityObject { root, geometries };

                    // print top-level city object
                    // println!(
                    //     "vertices={} polygons={}",
                    //     toplevel_cityobj.geometries.vertices.len(),
                    //     toplevel_cityobj.geometries.polygons.len()
                    // );
                    // println!("TLCO: {:#?}", toplevel_cityobj);
                    // println!("{}", serde_json::to_string(&toplevel_cityobj).unwrap());

                    // serialize with bincode
                    let start = encoded_data.len();
                    bincode::serde::encode_into_std_write(obj, encoded_data, bincode_params)
                        .unwrap();
                    encoded_sizes.push(encoded_data.len() - start);
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
    };

    match parse_result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Err: {e:?}");
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
    const ZSTD_LEVEL: i32 = 2;

    let args = Args::parse();

    let mut encoded_data = Vec::new();
    let mut encoded_sizes = Vec::new();
    let mut total_xml_uncompressed = 0;
    let mut total_xml_compressed = 0;

    // Parse XML files
    {
        let mut total_parsing_time = std::time::Duration::new(0, 0);

        for filename in &args.filenames {
            println!("loading city objects from: {filename} ...");

            let reader = {
                let Ok(xml_data) = std::fs::read(filename) else {
                    panic!("failed to open file {filename}");
                };

                // Just for reference, get the compressed size of the XML file
                let compressed =
                    zstd::stream::encode_all(std::io::Cursor::new(&xml_data), ZSTD_LEVEL).unwrap();
                total_xml_uncompressed += xml_data.len();
                total_xml_compressed += compressed.len();
                std::io::Cursor::new(xml_data)
            };

            let inst = Instant::now();
            let mut xml_reader = quick_xml::NsReader::from_reader(reader);
            let context = nusamai_citygml::ParseContext::default();
            match CityGmlReader::new(context).start_root(&mut xml_reader) {
                Ok(mut st) => {
                    match example_toplevel_dispatcher(
                        &mut st,
                        &mut encoded_data,
                        &mut encoded_sizes,
                    ) {
                        Ok(size) => size,
                        Err(e) => panic!("Err: {e:?}"),
                    }
                }
                Err(e) => panic!("Err: {e:?}"),
            };
            let parsing_time = inst.elapsed();
            println!("elapsed time parsing: {parsing_time:?}");
            total_parsing_time += parsing_time;
        }

        println!("total parsing time: {total_parsing_time:?}");
    }

    println!("total number of features: {}", encoded_sizes.len());

    let (enc_dict, dec_dict) = {
        println!("building Zstd dictionary...");
        let Ok(dict) = zstd::dict::from_continuous(&encoded_data, &encoded_sizes, 2048 * 1024)
        else {
            panic!("failed to create dictionary");
        };
        println!("done.");
        (
            zstd::dict::EncoderDictionary::copy(&dict, ZSTD_LEVEL),
            zstd::dict::DecoderDictionary::copy(&dict),
        )
    };

    let total_bin_uncompressed = encoded_sizes.iter().sum::<usize>();
    let mut total_bin_lz4_compressed = 0;
    let mut total_bin_zstd_without_dict = 0;
    let mut total_bin_zstd_with_dict = 0;

    // LZ4 compression
    {
        let inst = Instant::now();
        encoded_sizes
            .iter()
            .scan(0, |state, &x| {
                let start = *state;
                *state += x;
                Some((start, *state))
            })
            .for_each(|(begin, end)| {
                let data = &encoded_data[begin..end];
                let compressed = lz4_flex::compress_prepend_size(data);
                total_bin_lz4_compressed += compressed.len();
                let decompressed = lz4_flex::decompress_size_prepended(&compressed).unwrap();
                assert_eq!(data, &decompressed[..]);
            });
        println!("compress and decompress with LZ4: {:?}", inst.elapsed());
    }

    // Zstd compression without dictionary
    {
        let mut comp = zstd::bulk::Compressor::new(ZSTD_LEVEL).unwrap();
        let mut decomp = zstd::bulk::Decompressor::new().unwrap();
        let inst = Instant::now();
        encoded_sizes
            .iter()
            .scan(0, |state, &x| {
                let start = *state;
                *state += x;
                Some((start, *state))
            })
            .for_each(|(begin, end)| {
                let data = &encoded_data[begin..end];
                let compressed = comp.compress(data).unwrap();
                total_bin_zstd_without_dict += compressed.len();
                let decompressed = decomp.decompress(&compressed, data.len()).unwrap();
                assert_eq!(data, &decompressed[..]);
            });
        println!(
            "compress and decompress with Zstd (w/o dict): {:?}",
            inst.elapsed()
        );
    }

    // Zstd compression with dictionary
    {
        let mut comp = zstd::bulk::Compressor::with_prepared_dictionary(&enc_dict).unwrap();
        let mut decomp = zstd::bulk::Decompressor::with_prepared_dictionary(&dec_dict).unwrap();
        let inst = Instant::now();
        encoded_sizes
            .iter()
            .scan(0, |state, &x| {
                let start = *state;
                *state += x;
                Some((start, *state))
            })
            .for_each(|(begin, end)| {
                let data = &encoded_data[begin..end];
                let compressed = comp.compress(data).unwrap();
                total_bin_zstd_with_dict += compressed.len();
                let decompressed = decomp.decompress(&compressed, data.len()).unwrap();
                assert_eq!(data, &decompressed[..]);
            });
        println!(
            "compress and decompress with Zstd (w/ dict): {:?}",
            inst.elapsed()
        );
    }

    println!(
        "uncompressed_xml: {:.2} (MiB)",
        total_xml_uncompressed as f32 / 1024. / 1024.
    );
    println!(
        "compressed_xml: {:.2} (MiB)",
        total_xml_compressed as f32 / 1024. / 1024.
    );
    println!(
        "bin_uncompressed: {:.2} (MiB)",
        total_bin_uncompressed as f32 / 1024. / 1024.
    );
    println!(
        "bin_lz4: {:.2} (MiB)",
        total_bin_lz4_compressed as f32 / 1024. / 1024.
    );
    println!(
        "bin_zstd w/o dict: {:.2} (MiB)",
        total_bin_zstd_without_dict as f32 / 1024. / 1024.
    );
    println!(
        "bin_zstd w/ dict: {:.2} (MiB)",
        total_bin_zstd_with_dict as f32 / 1024. / 1024.
    );
}
