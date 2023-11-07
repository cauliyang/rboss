use assert_cmd::cmd::Command;

#[test]
fn test_rboss() {
    let mut cmd = Command::cargo_bin("rboss").unwrap();
    cmd.assert().success();
}

#[test]
fn test_extract() {
    // read to vec<u8>
    let mut cmd = Command::cargo_bin("rboss").unwrap();
    cmd.args([
        "extract",
        "tests/data/extract_1.txt",
        "tests/data/reads.bam",
    ]);
    cmd.assert().success();
}

#[test]
fn test_extract_binary() {
    // read to vec<u8>

    let mut cmd = Command::cargo_bin("rboss").unwrap();
    cmd.args([
        "extract",
        "tests/data/extract_1.txt",
        "tests/data/reads.bam",
        "-b",
    ]);
    cmd.assert().success();
}
