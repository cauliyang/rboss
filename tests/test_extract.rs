use assert_cmd::cmd::Command;
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn test_rboss() {
    let mut cmd = Command::cargo_bin("rboss").unwrap();
    cmd.assert().success();
}

#[test]
fn test_extract() {
    // read to vec<u8>
    let ground_truth = fs::read("tests/data/extract_1.sam").unwrap();
    let ground_truth_sha256 = sha256::digest(ground_truth.as_slice());

    let mut cmd = Command::cargo_bin("rboss").unwrap();
    cmd.args([
        "extract",
        "tests/data/extract_1.txt",
        "tests/data/reads.bam",
    ]);
    cmd.assert().success();

    let output = cmd.output().expect("failed to execute process");
    let output_sha256 = sha256::digest(output.stdout.as_slice());

    assert_eq!(ground_truth_sha256, output_sha256);
}

#[test]
fn test_extract_binary() {
    // read to vec<u8>
    let ground_truth = fs::read("tests/data/extract_1.bam").unwrap();
    let ground_truth_sha256 = sha256::digest(ground_truth.as_slice());

    let mut cmd = Command::cargo_bin("rboss").unwrap();
    cmd.args([
        "extract",
        "tests/data/extract_1.txt",
        "tests/data/reads.bam",
        "-b",
    ]);
    cmd.assert().success();

    let output = cmd.output().expect("failed to execute process");
    let output_sha256 = sha256::digest(output.stdout.as_slice());

    assert_eq!(ground_truth_sha256, output_sha256);
}
