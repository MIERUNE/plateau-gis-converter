use nusamai_gltf::*;

#[test]
fn load_examples() {
    for path in glob::glob("./tests/examples/*.gltf").unwrap() {
        let path = path.unwrap();
        println!("loading {:?}", path);
        let src = std::fs::read_to_string(path).unwrap();
        let a: GLTF = serde_json::from_str(&src).unwrap();
        println!("{:?}", a);

        // 'null' should not appear in output
        let a = serde_json::to_string(&a).unwrap();
        assert!(!a.contains("null"));
        println!("{:?}", a);
    }
}
