fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let filename = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("Expected a path to a file as first argument.");
            std::process::exit(-1);
        }
    };

    let mut reader = shapefile::Reader::from_path(filename).unwrap();

    for result in reader.iter_shapes_and_records() {
        let (shape, record) = result.unwrap();
        println!("Shape: {shape}, records: ");
        for (name, value) in record {
            println!("\t{name}: {value:?} ");
        }
    }
}
