use nusamai_gltf_json::*;

#[test]
fn load_examples() {
    for path in glob::glob("./tests/samples/2.0/*.gltf").unwrap() {
        // deserialize
        let path = path.unwrap();
        println!("loading {path:?}");
        let src = std::fs::read_to_string(path).unwrap();
        let gltf: Gltf = serde_json::from_str(&src).unwrap();

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
fn load_3dtiles_examples() {
    for path in glob::glob("./tests/samples/3d-tiles/**/*.gltf").unwrap() {
        // deserialize
        let path = path.unwrap();
        println!("loading {path:?}");
        let src = std::fs::read_to_string(path).unwrap();
        let gltf: Gltf = serde_json::from_str(&src).unwrap();

        // serialize
        let ser = serde_json::to_string(&gltf).unwrap();
        // 'null' should not appear in output
        assert!(!ser.contains("null"));

        // deserialize again (expect the same deserialization result)
        let gltf2 = serde_json::from_str(&ser).unwrap();
        assert_eq!(gltf, gltf2);
    }
}
