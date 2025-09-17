use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// Helper function to create a temporary test directory
fn create_test_dir(name: &str) -> io::Result<PathBuf> {
    let temp_dir = env::temp_dir().join(format!("fromdir_test_{}", name));

    // Remove if exists
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }

    // Create fresh directory
    fs::create_dir_all(&temp_dir)?;

    // Create a test file inside
    let test_file = temp_dir.join("test_file.txt");
    let mut file = File::create(&test_file)?;
    writeln!(file, "This is a test file")?;

    Ok(temp_dir)
}

// Helper to get the path to the fromdir binary
fn fromdir_bin() -> PathBuf {
    // For integration tests, Cargo automatically sets the right environment variables
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    Path::new(&manifest_dir)
        .join("target")
        .join(profile)
        .join("fromdir")
}

#[test]
fn test_command_execution() -> io::Result<()> {
    let test_dir = create_test_dir("command_execution")?;

    // Run fromdir to execute ls in the test directory
    let output = Command::new(fromdir_bin())
        .arg(test_dir.to_string_lossy().to_string())
        .arg("ls")
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test_file.txt"));

    Ok(())
}

#[test]
fn test_command_with_args() -> io::Result<()> {
    let test_dir = create_test_dir("command_with_args")?;

    // Run fromdir to execute echo with arguments
    let output = Command::new(fromdir_bin())
        .arg(test_dir.to_string_lossy().to_string())
        .arg("echo")
        .arg("hello")
        .arg("world")
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello world");

    Ok(())
}

#[test]
fn test_invalid_directory() -> io::Result<()> {
    let invalid_dir = "/directory/does/not/exist";

    // Run fromdir with an invalid directory
    let output = Command::new(fromdir_bin())
        .arg(invalid_dir)
        .arg("ls")
        .output()?;

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to change directory"));

    Ok(())
}

#[test]
fn test_invalid_command() -> io::Result<()> {
    let test_dir = create_test_dir("invalid_command")?;

    // Run fromdir with a command that doesn't exist
    let output = Command::new(fromdir_bin())
        .arg(test_dir.to_string_lossy().to_string())
        .arg("command_that_does_not_exist")
        .output()?;

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to execute"));

    Ok(())
}

#[test]
fn test_no_args() -> io::Result<()> {
    // Run fromdir with no arguments
    let output = Command::new(fromdir_bin()).output()?;

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));

    Ok(())
}

#[test]
fn test_directory_only() -> io::Result<()> {
    let test_dir = create_test_dir("directory_only")?;

    // Run fromdir with directory but no command
    let output = Command::new(fromdir_bin())
        .arg(test_dir.to_string_lossy().to_string())
        .output()?;

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No command specified"));

    Ok(())
}
