use tempfile;
use assert_cmd::prelude::*;
use std::process::{Command, Stdio};
use std::time::Duration;

#[test]
fn runonce() -> Result<(), Box<dyn std::error::Error>> {
    // create a temp directory
    let tempdir = tempfile::tempdir()?;

    // create a subdirectory
    std::fs::create_dir(tempdir.path().join("subdir"))?;

    let mut cmd = Command::cargo_bin("antr")?;
    cmd.current_dir(tempdir.path());
    cmd.arg("--runonce");
    cmd.arg("ls");
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.env("RUST_LOG", "info");

    let child = cmd.spawn().expect("failed to execute process");

    // Wait for the child process to start
    std::thread::sleep(Duration::from_secs(1));

    // create a file
    std::fs::File::create(tempdir.path().join("subdir/file"))?;

    let output = child.wait_with_output().expect("child process wasn't running");

    output.assert().success();

    Ok(())
}
