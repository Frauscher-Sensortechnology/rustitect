//! # Rustitect - a rust-to-arc42 Documentation Generator
//!
//! This application generates arc42 class specifications in Asciidoc format
//! from Rust code, utilizing PlantUML for diagram generation.
//!
//! The application consists of several modules:
//!
//! - The `cli` module defines a struct `Cli` for parsing command-line arguments.
//! - The `plantuml_parser` module provides a function `parse_to_string` that takes a path as
//! input and returns a PlantUML string representation of the Rust code.
//! - The `rust_doc_parser` module provides a function `parse_code_doc_to_markdown_string` that
//! takes a path as input and returns a Markdown string representation of the Rust code.
//! - The `asciidoc_parser` will use the extracted markdown of the `rust_doc_parser` to generate
//! the representative asciidoc. For this pandoc is used and needs to be installed on the system.
//!
//! The `main` function is the entry point of the application. It initializes the logger, parses
//! command-line arguments using `Cli::parse()`, and determines the input source (either from a
//! file or standard input).
//!
//! The input is then passed to the `plantuml_parser::parse_to_string` function, which generates a
//! PlantUML string representation of the Rust code.
//!
//! # Dependencies
//! This module relies on various external crates such as `clap`, `regex`, `syn` and an own
//! version of `ruml` to function correctly.
//!
//! Additionally, the module also utilizes internal modules: `cli`, `model`,
//! `parser`, and `processing` to carry out its functionalities.

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

use processing::Processing;

use crate::cli::OutputFormat::AsciidocPlantuml;
use crate::cli::{Cli, OutputFormat};

mod cli;
mod model;
mod parser;
mod processing;

/// The main entry point of the Rustitect application.
///
/// Processes the command-line arguments, and orchestrates
/// the reading, processing, and writing of data.
fn main() {
    let mut args = Cli::parse();

    handle_preserve_names_and_set_output_file(&mut args);

    let input = read_input(&args.input_file);
    let processing = Processing { args: args.clone() };
    let output = processing.start(&input);

    let prefix = args.file_name_prefix.expect("File name prefix not set");
    write_output(output, &args.output_file, prefix);
}

/// Checks if the 'preserve_names' argument is provided.
///
/// If so, ensures that the input isn't coming from stdin, as name preservation
/// from stdin isn't supported. It also constructs the output file name based on
/// the input file name and the desired output format.
fn handle_preserve_names_and_set_output_file(args: &mut Cli) {
    let stdin = PathBuf::from("-");
    if args.preserve_names {
        let input_path = PathBuf::from(args.input_file.as_ref().unwrap());

        if input_path == stdin {
            let mut cmd = Cli::command();
            cmd.error(
                ErrorKind::ArgumentConflict,
                "Can't preserve names, when input is stdin",
            )
            .exit();
        } else {
            let name = input_path.file_stem().unwrap().to_str().unwrap();
            let extension = get_output_format_extension(&args.format);
            args.output_file = Some(format!("{name}{extension}"));
        }
    }
}

/// Determines the appropriate file extension based on the specified output format.
fn get_output_format_extension(format: &OutputFormat) -> &str {
    match format {
        OutputFormat::Asciidoc => ".adoc",
        OutputFormat::AsciidocPlantuml => ".puml",
        OutputFormat::Markdown => ".md",
        OutputFormat::Plantuml => ".puml",
    }
}

/// Reads the content of the specified file or from stdin if no file is provided.
fn read_input(input_file: &Option<String>) -> String {
    let mut input_buffer = String::new();

    match input_file {
        Some(input_file) => {
            let input_path = PathBuf::from(input_file);
            let mut file = File::open(input_path).expect("Failed to open input file");
            file.read_to_string(&mut input_buffer)
                .expect("Failed to read input file");
        }
        None => {
            io::stdin()
                .read_to_string(&mut input_buffer)
                .expect("Failed to read from stdin");
        }
    };

    input_buffer
}

/// Writes the processed output either to the specified file or to stdout.
fn write_output(
    output: HashMap<OutputFormat, String>,
    output_file: &Option<String>,
    file_name_prefix: String,
) {
    match output_file {
        Some(output_file) => {
            let file_name = Path::new(output_file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let output_is_combined = output.contains_key(&AsciidocPlantuml);
            for (format, mut content) in output {
                if output_is_combined && format == OutputFormat::Asciidoc {
                    content = content.replace("FILENAME", file_name);
                }
                let extension = get_output_format_extension(&format);
                let output_file_name = format!("{}{}{}", file_name_prefix, file_name, extension);
                let mut file =
                    File::create(output_file_name).expect("Failed to create output file");
                file.write_all(content.as_bytes())
                    .expect("Failed to write output file");
            }
        }
        None => {
            let output_content = output
                .values()
                .map(|content| content.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            io::stdout()
                .write_all(output_content.as_bytes())
                .expect("Failed to write to stdout");
        }
    };
}
