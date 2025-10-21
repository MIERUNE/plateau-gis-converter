use std::{fs::File, io::BufReader};

use nusamai_gltf::glb::Glb;
use nusamai_gltf_json::*;

#[test]
fn load_glb_examples() {
    for path in glob::glob("./nusamai-gltf-json/tests/samples/2.0/*.glb").unwrap() {
        let path = path.unwrap();
        println!("loading {path:?}");

        let reader = BufReader::new(File::open(&path).unwrap());
        let glb = Glb::from_reader(reader).unwrap();

        let gltf: Gltf = serde_json::from_slice(&glb.json).unwrap();

        if let Some(bin) = glb.bin {
            let buffer = gltf.buffers.first().unwrap();
            assert!(bin.len() <= buffer.byte_length as usize + 3);
        }

        // serialize
        let ser = serde_json::to_string(&gltf).unwrap();
        // 'null' should not appear in output
        assert!(!ser.contains("null"));

        // deserialize again (expect the same deserialization result)
        let gltf2 = serde_json::from_str(&ser).unwrap();
        assert_eq!(gltf, gltf2);
    }
}

#[test]
fn load_3dtiles_glb_examples() {
    for path in glob::glob("./nusamai-gltf-json/tests/samples/3d-tiles/**/*.glb").unwrap() {
        let path = path.unwrap();
        println!("loading {path:?}");

        let reader = BufReader::new(File::open(&path).unwrap());
        let glb = Glb::from_reader(reader).unwrap();

        let gltf: Gltf = serde_json::from_slice(&glb.json).unwrap();

        if let Some(bin) = glb.bin {
            let buffer = gltf.buffers.first().unwrap();
            assert!(bin.len() <= buffer.byte_length as usize + 3);
        }

        // serialize
        let ser = serde_json::to_string(&gltf).unwrap();
        // 'null' should not appear in output
        assert!(!ser.contains("null"));

        // deserialize again (expect the same deserialization result)
        let gltf2 = serde_json::from_str(&ser).unwrap();
        assert_eq!(gltf, gltf2);
    }
}
