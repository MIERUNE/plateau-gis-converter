use std::io::BufReader;
use std::io::Read;

use nusamai_3dtiles_json::subtree;
use nusamai_3dtiles_json::tileset;

#[test]
fn load_tileset_json() {
    for path in glob::glob("./tests/samples/**/tileset.json").unwrap() {
        let path = path.unwrap();
        println!("loading {:?}", path);
        let src = std::fs::read_to_string(path).unwrap();
        let a: tileset::Tileset = serde_json::from_str(&src).unwrap();
        let _ = format!("{:?}", a);

        // 'null' should not appear in output
        let a = serde_json::to_string(&a).unwrap();
        assert!(!a.contains("null"));
    }
}

#[test]
fn load_subtree_json() {
    for path in glob::glob("./tests/samples/**/*.subtree").unwrap() {
        let path = path.unwrap();
        println!("loading {:?}", path);
        let file = std::fs::File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        // Read header
        let mut buf4 = [0; 4];
        let mut buf8 = [0; 8];
        reader.read_exact(&mut buf4).unwrap();
        reader.read_exact(&mut buf4).unwrap();
        reader.read_exact(&mut buf8).unwrap();
        let json_length = u64::from_le_bytes(buf8) as usize;
        reader.read_exact(&mut buf8).unwrap();
        let bin_length = u64::from_le_bytes(buf8) as usize;

        // Read JSON part
        let mut json_part = vec![0; json_length];
        reader.read_exact(&mut json_part).unwrap();
        let a: subtree::Subtree = serde_json::from_slice(&json_part).unwrap();
        let _ = format!("{:?}", a);

        // 'null' should not appear when re-serialized
        let a = serde_json::to_string(&a).unwrap();
        assert!(!a.contains("null"));

        // Skip padding
        if json_length % 8 != 0 {
            let mut skip = vec![0; json_length % 8];
            reader.read_exact(&mut skip).unwrap();
        }

        // Read Bin part
        let mut bin_part = vec![0; bin_length];
        reader.read_exact(&mut bin_part).unwrap();
    }
}
