fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let filename = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("Expected a path to a file as first argument.");
            std::process::exit(-1);
        }
    };

    // Unlike `shapefile::Reader::from_path`, this falls back to Shift_JIS for
    // .dbf files that carry no encoding marker at all.
    let mut reader =
        nusamai_shapefile::encoding::reader_from_path(std::path::Path::new(filename)).unwrap();

    for result in reader.iter_shapes_and_records() {
        let (shape, record) = result.unwrap();
        println!("Shape: {shape}, records: ");
        for (name, value) in record {
            println!("\t{name}: {value:?} ");
        }
    }
}
