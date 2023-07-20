use std::process::Command;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};

#[test]
fn test_main_without_arguments() {
    let path = path_of_project_exe();

    let output = Command::new(path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_main_help_is_printed() {
    let path = path_of_project_exe();
    let project_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let expected_output = format!("Usage: {project_name}");

    let output = Command::new(path)
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    let output_as_string = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(output_as_string.contains(&expected_output));
}

#[test]
fn test_main_asciidoc_output_is_correct() {
    let path = path_of_project_exe();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let file_path = std::path::Path::new(&manifest_dir).join("resources/simple_struct.adoc");
    let input_file_path = std::path::Path::new(&manifest_dir).join("resources/simple_struct.rs");
    let mut expected_output = String::new();
    let mut file = File::open(file_path.clone()).expect("Failed to open input file");
    file.read_to_string(&mut expected_output).expect("Failed to read input file");

    println!("file_path: {}", file_path.to_str().unwrap());

    let output = Command::new(path)
        .args(input_file_path.to_str())
        .output()
        .expect("Failed to execute command");

    let output_as_string = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert_eq!(output_as_string, expected_output);
}

#[test]
fn test_main_asciidoc_preserve_name() {
    let path = path_of_project_exe();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input_file_path = std::path::Path::new(&manifest_dir)
        .join("resources/simple_struct.rs");
    let expected_output_file = std::path::Path::new(&manifest_dir)
        .join("simple_struct.adoc");

    let output = Command::new(path)
        .args(input_file_path.to_str())
        .args(["--preserve-names"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(expected_output_file.exists());

    //CLEAN UP
    std::fs::remove_file(expected_output_file).unwrap();
}

#[test]
fn test_main_preserve_name_only_with_input_file() {
    let path = path_of_project_exe();

    let mut command = Command::new(path)
        .stdin(std::process::Stdio::piped())
        .args(["--preserve-names"])
        .spawn()
        .expect("Failed to spawn command");

    // Write data to `stdin` of the command process and wait to complete
    let stdin = command.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"class-input-data.").expect("Failed to write to stdin");
    let output = command.wait_with_output().expect("Failed to wait for command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(101));
}

fn path_of_project_exe() -> PathBuf {
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let mut path = PathBuf::from(cargo_dir);
    path.push("target");
    path.push("debug");
    path.push(project_name.clone());
    path
}