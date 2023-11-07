use assert_cmd::cmd::Command;
use tempfile::tempdir;

#[test]
fn test_rsoft() {
    let mut cmd = Command::cargo_bin("rboss").unwrap();

    let temp_dir = tempdir().unwrap();

    cmd.args([
        "rsoft",
        "tests",
        "-t",
        temp_dir.path().to_str().unwrap(),
        "-s",
        "bam",
    ]);

    cmd.assert().success();

    assert!(temp_dir.path().join("extract_1.bam").is_symlink());
    assert!(temp_dir.path().join("reads.bam").is_symlink());
}

#[test]
fn test_rsoft_all() {
    let mut cmd = Command::cargo_bin("rboss").unwrap();

    let temp_dir = tempdir().unwrap();

    cmd.args([
        "rsoft",
        "tests/data",
        "-t",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(temp_dir.path().join("extract_1.bam").is_symlink());
    assert!(temp_dir.path().join("reads.bam").is_symlink());
}
