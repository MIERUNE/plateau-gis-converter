// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod example;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert_and_save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn convert_and_save(input_path: String, output_path: String, filetype: String) {
    // let mut fin = BufReader::new(File::open(input_path).unwrap());

    match &*filetype {
        "GeoJSON" => {
            example::citygml_to_geojson(&input_path, &output_path);
        }
        _ => {
            println!("Unknown filetype: {}", filetype);
        }
    }
}
