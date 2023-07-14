use crate::cli::{Cli, OutputFormat};
use crate::parser::asciidoc_parser::AsciidocParser;
use crate::parser::plantuml_parser::PlantumlParser;
use crate::parser::rust_doc_parser::RustDocParser;

/// Processing struct that handles the processing of input based on the provided arguments.
pub struct Processing {
    pub args: Cli,
}

impl Processing {
    /// Starts the processing of the input based on the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to be processed.
    ///
    /// # Returns
    ///
    /// The processed output as a string.
    pub fn start(&self, input: &String) -> String {
        let output_buffer: String = if is_no_only_flag_set(&self.args) {
            let markdown_output = process_input(input);
            if self.args.format == OutputFormat::Markdown {
                markdown_output
            } else {
                let ascii_doc_parser = AsciidocParser::new(None);
                let asciidoc_output = ascii_doc_parser.parse_from_markdown(&markdown_output).expect("Failed to parse markdown to asciidoc");
                asciidoc_output.replace("[source,plantuml]", "[plantuml]")
            }
        } else {
            process_input_only_flags(input, &self.args)
        };

        output_buffer
    }
}

/// Processes the input content and generates the output content based on the provided only flags.
/// Returns the output content as a [String].
fn process_input_only_flags(input: &String, args: &Cli) -> String {
    let mut output_buffer = String::new();
    let mut output_puml = String::new();
    let mut output_markdown = String::new();

    if args.only_flags.plantuml_only {
        let plantuml_parser = PlantumlParser{ raw_rust_code: String::from(input) };
        let plantuml_string = plantuml_parser.parse_code_to_string();
        output_puml.push_str(&plantuml_string);
        output_buffer.push_str(output_puml.as_str());
    }

    if args.only_flags.markdown_only {
        let markdown_parser = RustDocParser{ raw_rust_code: String::from(input) };
        let markdown_string = markdown_parser.parse_code_doc_to_markdown_string();
        output_markdown.push_str(&markdown_string);
        output_buffer.push_str(output_markdown.as_str());
    }

    output_buffer
}

/// Processes the input content and generates the output content when no only flag is set.
/// Returns the output content as a [String].
fn process_input(input: &String) -> String {
    let mut output_buffer = String::new();
    let plantuml_parser = PlantumlParser{ raw_rust_code: String::from(input) };
    let doc_parser = RustDocParser{ raw_rust_code: String::from(input) };

    let plantuml = plantuml_parser.parse_code_to_string();
    let mut documentation = doc_parser.parse_code_doc();
    documentation.plantuml = plantuml;

    output_buffer.push_str(format!("## {}\n", documentation.name).as_str());
    output_buffer.push_str(format!("```plantuml\n{}\n```\n", documentation.plantuml).as_str());
    output_buffer.push_str(format!("\n{}\n", documentation.documentation).as_str());

    output_buffer
}

/// Returns true if no `only` flag is set.
/// Checks all only flags. If any of them is set, returns false.
fn is_no_only_flag_set(args: &Cli) -> bool {
    if args.only_flags.plantuml_only || args.only_flags.markdown_only {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;
    use crate::cli::OnlyFlags;
    use super::*;

    // Helper function to mock the Cli struct.
    fn create_mock_cli(input_file: Option<String>, output_file: Option<String>, plantuml_only: bool, markdown_only: bool, format: OutputFormat) -> Cli {
        Cli {
            only_flags : OnlyFlags {
                plantuml_only,
                markdown_only,
            },
            input_file,
            output_file,
            format,
            preserve_names: false
        }
    }

    #[test]
    fn only_flag_plantuml() {
        let cli_mock = create_mock_cli(None, None, true, false, OutputFormat::Asciidoc);
        let raw_rust_code = String::from(
            r#"
            struct TestStruct {
                field: String,
            }
            "#,
        );
        let expected_content = "@startuml";
        let not_expected_content = "## ";

        let processing = Processing {
            args: cli_mock,
        };
        let output = processing.start(&raw_rust_code);

        assert!(output.contains(expected_content));
        assert!(!output.contains(not_expected_content));
    }
    #[test]
    fn only_flag_markdown() {
        let cli_mock = create_mock_cli(None, None, false, true, OutputFormat::Asciidoc);
        let raw_rust_code = String::from(
            r#"
            struct TestStruct {
                field: String,
            }
            "#,
        );
        let expected_content = "## ";
        let not_expected_content = "@startuml";

        let processing = Processing {
            args: cli_mock,
        };
        let output = processing.start(&raw_rust_code);

        assert!(output.contains(expected_content));
        assert!(!output.contains(not_expected_content));
    }

    #[test]
    fn test_process_input() {
        let cli_mock = create_mock_cli(None, None, false, false, OutputFormat::Asciidoc);
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = std::path::Path::new(&manifest_dir).join("resources/simple_struct.rs");

        let mut rust_file = fs::File::open(file_path).expect("File not found.");
        // let mut rust_file = fs::File::open("resources/simple_struct.rs").expect("File not found.");
        let mut raw_rust_code = String::new();
        rust_file.read_to_string(&mut raw_rust_code).expect("Could not read file.");
        let expected_headline = " Person";
        let expected_plantuml = "class \"Person\"";

        let processing = Processing {
            args: cli_mock,
        };
        let output = processing.start(&raw_rust_code);

        assert!(output.contains(expected_headline));
        assert!(output.contains(expected_plantuml));
    }

    #[test]
    fn test_process_input_format_markdown() {
        let cli_mock = create_mock_cli(None, None, false, false, OutputFormat::Markdown);
        let raw_rust_code = String::from("struct Person { name: String }");
        let expected_headline = "## Person";

        let processing = Processing {
            args: cli_mock,
        };
        let output = processing.start(&raw_rust_code);

        assert!(output.contains(expected_headline));
    }

    #[test]
    fn test_process_input_format_asciidoc() {
        let cli_mock = create_mock_cli(None, None, false, false, OutputFormat::Asciidoc);
        let raw_rust_code = String::from("struct Person { name: String }");
        let expected_headline = "== Person";

        let processing = Processing {
            args: cli_mock,
        };
        let output = processing.start(&raw_rust_code);

        assert!(output.contains(expected_headline));
    }
}
