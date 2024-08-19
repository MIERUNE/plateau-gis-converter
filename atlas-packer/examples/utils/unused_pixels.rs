use clap::{Arg, ArgAction, Command};
use image::{open, GenericImageView, Rgba};

pub fn unused_pixels() -> (usize, usize) {
    let matches = Command::new("Image Processor")
        .about("Processes images to find unused pixels")
        .arg(
            Arg::new("INPUT")
                .help("Path to the image file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("unused_pixels")
                .help("Outputs unused pixels")
                .short('u')
                .long("unused")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    let input = matches
        .get_one::<String>("INPUT")
        .expect("INPUT is required");
    let img = open(input).unwrap_or_else(|e| {
        eprintln!("Failed to open image: {}", e);
        std::process::exit(1);
    });
    let all_pixels = img.width() as usize * img.height() as usize;

    if matches.get_flag("unused_pixels") {
        let unused_pixels = img
            .pixels()
            .filter(|&(_, _, pixel)| matches!(pixel, Rgba([0, 0, 0, 0])))
            .count();
        (all_pixels, unused_pixels)
    } else {
        (all_pixels, 0)
    }
}
