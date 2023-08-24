use log::debug;
use ruml::file_parser;

/// Represents a parser for converting Rust source code into a format that can be
/// visualized using PlantUML.
///
/// The `PlantumlParser` takes in Rust source code and uses the `ruml` crate
/// to generate a PlantUML-compatible representation. This representation
/// can then be used to generate UML diagrams, giving a visual representation
/// of the Rust source code.
pub struct PlantumlParser {
    /// The raw Rust source code that will be parsed into a PlantUML-compatible format.
    pub(crate) raw_rust_code: String,
}
impl PlantumlParser {
    /// Parses Rust source code and generates a PlantUML string representation.
    ///
    /// # Arguments
    ///
    /// - `raw_rust_code`: A string containing the Rust source code to be parsed.
    ///
    /// # Returns
    ///
    /// A string containing the PlantUML representation of the provided Rust source code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fs;
    /// use crate::PlantumlParser;
    /// let path = "path/to/rust_file.rs";
    /// let rust_code = fs::read_to_string(path).expect("Unable to read file");
    /// let plantuml_string = PlantumlParser::parse_code_to_string(rust_code);
    /// println!("{}", plantuml_string);
    /// ```
    ///
    /// The above example reads a Rust source file, passes its contents to
    /// `parse_to_string`, and then prints the resulting PlantUML string.
    pub fn parse_code_to_string(&self) -> String {
        debug!(
            "Parsing Rust file '{}' to PlantUML string...",
            self.raw_rust_code
        );
        let entities = file_parser(
            syn::parse_file(self.raw_rust_code.as_str()).expect("Unable to parse file"),
        );

        ruml::render_plantuml(entities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_code_to_string_is_empty() {
        let rust_code = String::from("");
        let expected_puml = "@startuml\n\n\n\n@enduml";

        let parser = PlantumlParser {
            raw_rust_code: rust_code,
        };
        let actual_puml = parser.parse_code_to_string();

        assert_eq!(String::from(expected_puml), actual_puml);
    }

    #[test]
    fn test_parse_code_to_string_is_struct_with_variable() {
        let rust_code = String::from("struct TestStruct { test_variable: i32 }");
        let expected_puml =
            "@startuml\n\nclass \"TestStruct\" {\n    - test_variable: i32\n}\n\n@enduml";

        let parser = PlantumlParser {
            raw_rust_code: rust_code,
        };
        let actual_puml = parser.parse_code_to_string();

        assert_eq!(String::from(expected_puml), actual_puml,);
    }
}
