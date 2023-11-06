use assert_cmd::cmd::Command;
use pretty_assertions::{assert_eq, assert_ne};
use sha256;
use std::fs;

#[test]
fn test_fa2fq() {
    let ground_truth = fs::read("tests/data/test_case1.fq").unwrap();
    let ground_truth_sha256 = sha256::digest(ground_truth.as_slice());

    let mut cmd = Command::cargo_bin("rboss").unwrap();
    cmd.args(&["fa2fq", "tests/data/test_case1.fa"]);
    cmd.assert().success();

    let output = cmd.output().expect("failed to execute process");
    let output_sha256 = sha256::digest(output.stdout.as_slice());

    assert_eq!(ground_truth_sha256, output_sha256);
}
