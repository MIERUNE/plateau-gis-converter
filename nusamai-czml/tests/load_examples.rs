use nusamai_czml::*;

#[test]
fn load_examples() {
    for path in glob::glob("./tests/samples/*.json").unwrap() {
        let path = path.unwrap();
        println!("loading {path:?}");
        let src = std::fs::read_to_string(path).unwrap();
        let a: Packet = serde_json::from_str(&src).unwrap();

        // 'null' should not appear in output
        let ser = serde_json::to_string(&a).unwrap();
        assert!(!ser.contains("null"));
    }
}
