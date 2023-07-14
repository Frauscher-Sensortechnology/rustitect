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
use std::io::{Read, Write};
use std::path::{PathBuf};
use clap::Parser;
use log::{debug};
use processing::Processing;
use crate::cli::Cli;

mod cli;
mod processing;
mod parser;
mod model;

/// The main entry point of the program.
fn main() {
    env_logger::init();

    let args = Cli::parse();

    let input = read_input(&args.input_file);
    let processing = Processing { args: args.clone() };
    let output = processing.start(&input);

    write_output(&output, &args.output_file);
}

/// Reads the input from the specified input file or from stdin.
/// Returns the input content as a string.
fn read_input(input_file: &Option<String>) -> String {
    let mut input_buffer = String::new();

    match input_file {
        Some(input_file) => {
            let input_path = PathBuf::from(input_file);
            debug!("Input file is: {}", input_path.display());
            let mut file = File::open(input_path).expect("Failed to open input file");
            file.read_to_string(&mut input_buffer).expect("Failed to read input file");
        },
        None => {
            io::stdin().read_to_string(&mut input_buffer).expect("Failed to read from stdin");
        }
    };

    input_buffer
}

/// Writes the output content to the specified output file or to stdout.
fn write_output(output: &str, output_file: &Option<String>) {
    match output_file {
        Some(output_file) => {
            let output_path = PathBuf::from(output_file);
            debug!("Output file is: {}", output_path.display());
            let mut file = File::create(output_path).expect("Failed to create output file");
            file.write_all(output.as_bytes()).expect("Failed to write output file");
        },
        None => {
            io::stdout().write_all(output.as_bytes()).expect("Failed to write to stdout");
        }
    };
}