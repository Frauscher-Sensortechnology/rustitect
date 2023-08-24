use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, io};

use log::error;

/// Utility for parsing and converting text formats, primarily focused on converting
/// from Markdown to AsciiDoc.
pub struct AsciidocParser {
    pandoc_path: String,
}

impl AsciidocParser {
    /// Creates a new instance of `AsciidocParser`.
    ///
    /// # Arguments
    ///
    /// `pandoc_path` - An optional path to the `pandoc` executable.
    /// If `None`, it will look for the `PANDOC_PATH` environment variable.
    /// If the environment variable is also not set, it defaults to "pandoc".
    pub fn new(pandoc_path: Option<String>) -> Self {
        let pandoc_path = pandoc_path
            .unwrap_or_else(|| env::var("PANDOC_PATH").unwrap_or_else(|_| String::from("pandoc")));

        AsciidocParser { pandoc_path }
    }

    /// Converts the provided Markdown text to AsciiDoc format.
    ///
    /// # Arguments
    /// * `markdown_text` - A string slice that holds the Markdown text to be converted.
    ///
    /// # Returns
    /// * `Ok(String)` - The converted AsciiDoc text.
    /// * `Err(Box<dyn Error>)` - An error occurred during the conversion process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate_name::AsciidocParser;
    /// let parser = AsciidocParser::new(None);
    /// let markdown_text = "# Title";
    /// let asciidoc_text = parser.parse_from_markdown(markdown_text);
    /// assert!(asciidoc_text.is_ok());
    /// ```
    pub fn parse_from_markdown(&self, markdown_text: &str) -> Result<String, Box<dyn Error>> {
        match self.convert_with_pandoc(markdown_text, Format::Markdown, Format::Asciidoc) {
            Ok(result) => {
                let result = result.replace("[source,plantuml]", "[plantuml]");
                Ok(result)
            }
            Err(e) => {
                error!("Error while converting Markdown to AsciiDoc: {}", e);
                Err(e.into())
            }
        }
    }

    /// Converts the provided text from one format to another using the `pandoc` command.
    /// The `pandoc` command must be available in the system path.
    /// You can provide the path to the `pandoc` command using the `PANDOC_PATH` environment variable.
    /// If the `PANDOC_PATH` environment variable is not set, the `pandoc` command is assumed to be
    /// available in the system path.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the text to be converted.
    /// * `input_format` - The [Format] of the input text.
    /// * `output_format` - The desired format of the output text.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The converted text.
    /// * `Err(io::Error)` - An error occurred during the conversion process.
    fn convert_with_pandoc(
        &self,
        input: &str,
        input_format: Format,
        output_format: Format,
    ) -> io::Result<String> {
        let mut child = Command::new(self.pandoc_path.as_str())
            .arg("-f")
            .arg(input_format.as_str())
            .arg("-t")
            .arg(output_format.as_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        {
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(input.as_bytes())?;
        }

        let output = child.wait_with_output()?;

        if output.status.success() {
            Ok(std::str::from_utf8(&output.stdout).unwrap().to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                std::str::from_utf8(&output.stderr).unwrap(),
            ))
        }
    }
}

/// `Format` is an enum that represents the supported text formats for
/// the [convert_with_pandoc] function.
#[derive(Debug)]
enum Format {
    Markdown,
    Asciidoc,
}
impl Format {
    fn as_str(&self) -> &'static str {
        match *self {
            Format::Markdown => "markdown",
            Format::Asciidoc => "asciidoc",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_markdown() {
        let parser = AsciidocParser::new(None);
        let markdown_text = "# Title\n\n## Subtitle\n\nSome text";

        let result = parser.parse_from_markdown(markdown_text);

        assert!(result.is_ok());
        let result_text = result.unwrap().replace("\r\n", "\n");
        assert_eq!(result_text, "== Title\n\n=== Subtitle\n\nSome text\n");
    }

    #[test]
    fn test_parse_from_markdown_error() {
        //Save the environment variable PANDOC_PATH before changing it
        let invalid_pandoc_path = "/invalid/path/to/pandoc";

        let parser = AsciidocParser::new(Some(String::from(invalid_pandoc_path)));
        let markdown_text = "# Title\n\n## Subtitle\n\nSome text";
        let result = parser.parse_from_markdown(markdown_text);
        assert!(result.is_err());
    }
}
