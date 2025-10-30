// tests/cli.rs
use assert_cmd::cargo::cargo_bin_cmd;

#[test]

fn test_run_cmd() {
    use assert_cmd::cargo::cargo_bin_cmd;

    let mut cmd = cargo_bin_cmd!("nusamai");
    let assert = cmd
        .arg("../nusamai-plateau/tests/data/sendai-shi/udx/urf/574026_urf_6668_huchi_op.gml")
        .arg("--sink")
        .arg("noop")
        .arg("--output")
        .arg("dummy")
        .arg("--rules")
        .arg("./tests/rules.json")
        .arg("--schema")
        .arg("schema.json")
        .assert();
    assert.success();
}
