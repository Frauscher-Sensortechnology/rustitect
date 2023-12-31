use clap::{Args, Parser, ValueEnum};

/// This struct represents the command-line arguments for the Rustitect.
///
/// It provides options for specifying the input Rust source code file,
/// the output filename, and the ability to skip other steps and generate only
/// the PlantUML representation of the code.
/// The struct is derived from `clap::Parser`, which enables automatic parsing
/// of command-line arguments based on the provided attributes.
///
/// # Examples
///
/// ```rust
///
///     let args = Cli::parse();
///
///     if let Some(input_file) = args.input_file {
///         println!("Input file: {}", input_file);
///     } else {
///         println!("Input file not specified. Reading from stdin.");
///     }
///
///     if let Some(output_file) = args.output_file {
///         println!("Output file: {}", output_file);
///     } else {
///         println!("Output file not specified. Printing to stdout.");
///     }
///
///     if args.plantuml_only {
///         println!("Only generating PlantUML.");
///     } else {
///         println!("Generating full documentation.");
///     }
/// ```
///
/// In the above example, the `Cli` struct is used to parse the command-line
/// arguments, and then the parsed values are printed to the console.
/// The `input_file` field represents the input Rust source code file, the
/// `output_file` field represents the output filename, and the `plantuml_only`
/// field determines whether to skip other steps and generate only the PlantUML
/// representation.
/// If any of the options are not specified, default values will be assigned
/// (e.g., `None` for file paths, `false` for boolean flags).
///
/// # Command-Line Usage
///
/// The command-line usage of the Rustitect is as follows:
///
/// ```plaintext
/// rustitect [OPTIONS] [INPUT_FILE] [OUTPUT_FILE]
/// ```
///
/// The available options are:
/// - `-o, --output-file`: Give an output filename. If not specified, the
/// output will be printed to stdout.
/// - `-p, --plantuml-only`: Skips the other steps and generates only the
/// PlantUML representation of the code.
///
/// Note: This documentation assumes that the `clap` crate is available and provides the necessary functionality for parsing command-line arguments.
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub only_flags: OnlyFlags,
    /// Input Rust source code file. If not specified, the input will be read from stdin
    #[arg(group = "input")]
    pub input_file: Option<String>,

    /// Output filename. If not specified, the output will be printed to stdout.
    #[arg(short, long, group = "output")]
    pub output_file: Option<String>,

    /// Format for the output. If not specified, asciidoc will be used.
    /// If 'asciidoc-plantuml' is specified, the output will be in asciidoc
    /// format including the PlantUML as file.
    #[arg(short, long, default_value = "asciidoc")]
    pub format: OutputFormat,

    /// Preserve names will get the name of the input file and put the same to
    /// the output file including the output format
    #[arg(long)]
    pub preserve_names: bool,

    /// Define a prefix for the output filename. Relly useful in combination
    /// with --preserve-names flag.
    #[arg(short = 'p', long = "prefix", default_value = "")]
    pub file_name_prefix: Option<String>,
}

#[derive(Args, Clone)]
#[group(required = false, multiple = false)]
pub struct OnlyFlags {
    /// Skip the other steps and just generate the PlantUML of the code.
    #[arg(long)]
    pub plantuml_only: bool,

    /// Skip the other steps and just generate markdown.
    #[arg(long)]
    pub markdown_only: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OutputFormat {
    Asciidoc,
    AsciidocPlantuml,
    Markdown,
    Plantuml,
}
