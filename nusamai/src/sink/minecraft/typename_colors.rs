use std::collections::HashMap;

pub fn get_typename_colors() -> HashMap<&'static str, [u8; 3]> {
    let mut typename_colors: HashMap<&str, [u8; 3]> = HashMap::new();

    typename_colors.insert("bldg", [182, 82, 54]);
    typename_colors.insert("road", [109, 68, 50]);
    typename_colors.insert("railway", [207, 74, 55]);
    typename_colors.insert("track", [132, 104, 112]);
    typename_colors.insert("square", [234, 236, 237]);
    typename_colors.insert("waterway", [212, 241, 249]);
    typename_colors.insert("luse", [98, 189, 105]);
    typename_colors.insert("frn", [132, 104, 112]);
    typename_colors.insert("veg", [37, 82, 59]);
    typename_colors.insert("wtr", [0, 255, 240]);
    typename_colors.insert("dem", [132, 103, 113]);
    typename_colors.insert("brid", [207, 74, 55]);
    typename_colors.insert("tun", [105, 105, 105]);
    typename_colors.insert("gen", [234, 236, 237]);
    typename_colors.insert("urf", [84, 64, 205]);
    typename_colors.insert("uro", [84, 64, 205]);

    typename_colors
}
