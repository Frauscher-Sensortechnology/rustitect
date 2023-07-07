use log::{debug};
use ruml::file_parser;

/// A parser for converting Rust code to PlantUML strings.
pub struct PlantumlParser {}

impl PlantumlParser {
    /// Parses the given Rust code into a PlantUML string representation.
    ///
    /// This method takes a `String` of Rust code, reads the contents of the
    /// file, parses the syntax using `syn::parse_file`, and then uses
    /// `ruml` crate to generate a PlantUML string representation of the
    /// entities in the Rust code.
    ///
    /// # Arguments
    ///
    /// - `raw_rust_code`: A `String` representing the Rust code file to parse.
    ///
    /// # Returns
    ///
    /// A `String` representing the PlantUML string generated from the Rust code.
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
    /// In the above example, the `parse_to_string` method is called on a
    /// `PlantumlParser` instance with a `String` of Rust code from a file.
    /// The resulting PlantUML string is then printed to the console.
    pub fn parse_code_to_string(raw_rust_code: &String) -> String {
        debug!("Parsing Rust file '{}' to PlantUML string...", raw_rust_code);
        let entities = file_parser(
            syn::parse_file(raw_rust_code).expect("Unable to parse file")
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
        let actual_puml = PlantumlParser::parse_code_to_string(&rust_code);
        assert_eq!(String::from(expected_puml), actual_puml);
    }

    #[test]
    fn test_parse_code_to_string_is_struct_with_variable() {
        let rust_code = String::from("struct TestStruct { test_variable: i32 }");
        let expected_puml = "@startuml\n\nclass \"TestStruct\" {\n    + test_variable: i32\n}\n\n@enduml";
        let actual_puml = PlantumlParser::parse_code_to_string(&rust_code);
        assert_eq!(
            String::from(expected_puml),
            actual_puml,
        );
    }
}
