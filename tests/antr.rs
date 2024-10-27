use tempfile;
use assert_cmd::prelude::*;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::Write;

#[test]
fn file_trigger_in_current_directory() -> Result<(), Box<dyn std::error::Error>> {
    // create a temp directory
    let tempdir = tempfile::tempdir()?;

    let mut cmd = Command::cargo_bin("antr")?;
    cmd.current_dir(tempdir.path());
    cmd.arg("--runonce");
    cmd.arg("ls");
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.env("RUST_LOG", "info");

    let child = cmd.spawn().expect("failed to execute process");

    // Create a channel to capture the output of the child process
    let (tx, rx) = mpsc::channel();

    // Run the command on a separate thread
    thread::spawn(move || {
        let output = child.wait_with_output().expect("child process wasn't running");
        let _ = tx.send(output); // Send the output to the main thread
    });

    // Wait for the child process to start
    std::thread::sleep(Duration::from_secs(1));

    // create a file
    std::fs::File::create(tempdir.path().join("trigger.txt"))?;

    // Wait for the process to finish with a timeout
    match rx.recv_timeout(Duration::from_secs(2)) {
        Ok(output) => {
            output.assert().success();
        }
        Err(e) => {
            return Err(format!("Failed to receive output: {:?}", e).into());
        }
    }

    Ok(())
}

#[test]
fn file_trigger_in_subdirectory() -> Result<(), Box<dyn std::error::Error>> {
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

#[test]
fn manual_trigger() -> Result<(), Box<dyn std::error::Error>> {
    // create a temp directory
    let tempdir = tempfile::tempdir()?;

    // create a subdirectory
    std::fs::create_dir(tempdir.path().join("subdir"))?;

    let mut cmd = Command::cargo_bin("antr")?;
    cmd.current_dir(tempdir.path());
    cmd.arg("--runonce");
    cmd.arg("ls");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.env("RUST_LOG", "info");

    let mut child = cmd.spawn().expect("failed to execute process");

    // Wait for the child process to start
    std::thread::sleep(Duration::from_secs(1));

    // send a newline to the child process
    let mut child_stdin = child.stdin.take().expect("child process stdin not available");
    writeln!(child_stdin, "")?;

    let output = child.wait_with_output().expect("child process wasn't running");

    output.assert().success();

    Ok(())
}
