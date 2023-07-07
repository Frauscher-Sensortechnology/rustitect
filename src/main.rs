//! # Rustitect - a rust-to-arc42 Documentation Generator
//!
//! This application generates arc42 class specifications in Asciidoc format
//! from Rust code, utilizing PlantUML for diagram generation.
//!
//! The application consists of several modules:
//!
//! - The `cli` module defines a struct `Cli` for parsing command-line arguments.
//! - The `plantuml_parser` module provides a function `parse_to_string` that takes a path as input and returns a PlantUML string representation of the Rust code.
//! - The `asciidoc_generator` module contains a struct `AsciiDocGenerator` that generates arc42 class specifications in Asciidoc format based on the PlantUML string and diagram image.
//!
//! The `main` function is the entry point of the application. It initializes the logger, parses command-line arguments using `Cli::parse()`, and determines the input source (either from a file or standard input).
//!
//! The input is then passed to the `plantuml_parser::parse_to_string` function, which generates a PlantUML string representation of the Rust code.
//!
//! The PlantUML string is further processed by the `PlantUMLGenerator` from the `plantuml_generator` crate, which generates a diagram image.
//!
//! Finally, the `AsciiDocGenerator` from the `asciidoc_generator` crate uses the PlantUML string and diagram image to generate the arc42 class specifications in Asciidoc format, which are printed to the standard output.
//!
//! Note: This documentation assumes that the `plantuml_generator` and `asciidoc_generator` crates provide the required functionality for diagram generation and Asciidoc generation, respectively. Please refer to their documentation for more accurate information.

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{PathBuf};
use clap::Parser;
use log::{debug};
use crate::cli::Cli;
use crate::plantuml_parser::PlantumlParser;

mod cli;
mod plantuml_parser;

fn main() {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();

    let args = Cli::parse();
    let run_all_tasks = match is_no_only_flag_set(&args) {
        true => {
            debug!("run all tasks");
            true
        },
        false => false
    };

    // Create an input reader based on the availability of the input file argument
    let mut input_reader: Box<dyn Read> = match args.input_file {
        Some(input_file) => {
            let input_path = PathBuf::from(input_file);
            debug!("Input file is: {}", input_path.display());
            Box::new(File::open(input_path).expect("Failed to open input file"))
        },
        None => Box::new(io::stdin()),
    };

    // Read the input into a string buffer
    let mut buffer = String::new();
    input_reader.read_to_string(&mut buffer).expect("Failed to read input");

    //Create an output file path when the argument is set else use stdout
    let output_file = get_path_of_string(args.output_file.as_deref());

    if run_all_tasks || args.plantuml_only {
        let plantuml_string =
            PlantumlParser::parse_code_to_string(&buffer);
        debug!("PlantUML string: {}", plantuml_string);
    }
}

/// Converts an optional string argument to a `PathBuf` or returns an error if
/// no input file is specified.
///
/// This function takes an optional string argument `file_string` and attempts
/// to convert it to a `PathBuf` representing a file path.
/// If `file_string` is `Some`, it creates a `PathBuf` using the provided
/// string and returns it as a successful `Result`.
/// If `file_string` is `None`, it returns an error of type `io::Error` with
/// an `InvalidInput` error kind, indicating that no input file was specified.
///
/// # Arguments
///
/// - `file_string`: An optional string slice (`&str`) representing the file path.
///
/// # Returns
///
/// A `Result` that contains either a `PathBuf` representing the file path or an error of type `io::Error`.
///
/// # Examples
///
/// ```rust
/// use std::path::PathBuf;
///
/// let file_path = get_path_of_string(Some("/path/to/file.txt"));
/// assert_eq!(file_path, Ok(PathBuf::from("/path/to/file.txt")));
///
/// let no_file_path = get_path_of_string(None);
/// assert!(no_file_path.is_err());
/// ```
///
/// In the above example, the `get_path_of_string` function is used to convert an optional string argument to a `PathBuf`.
/// The first example demonstrates a successful conversion with a provided file path, while the second example shows an error returned when no input file is specified.
///
/// # Error Handling
///
/// If no input file is specified (i.e., `file_string` is `None`), the function returns an `io::Error` with an `InvalidInput` error kind.
/// This error can be handled using standard Rust error handling techniques, such as pattern matching or the `?` operator.
///
/// # Panics
///
/// This function does not panic under normal circumstances.
/// However, if an error occurs during the creation of the `PathBuf`, such as if the provided file path is invalid, a panic may occur.
/// It is recommended to handle errors appropriately to avoid panics.
fn get_path_of_string(file_string: Option<&str>) -> Result<PathBuf, io::Error> {
    if let Some(path) = file_string {
        return Ok(PathBuf::from(path));
    }

    Err(io::Error::new(io::ErrorKind::InvalidInput, "No input file specified"))
}

/// Returns true if no `only` flag is set.
/// Checks all only flags. If any of them is set, returns false.
fn is_no_only_flag_set(args: &Cli) -> bool {
    if args.plantuml_only {
        return false;
    }
    true
}
