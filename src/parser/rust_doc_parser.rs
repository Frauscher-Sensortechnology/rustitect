//! A module for parsing Rust code documentation and generating Markdown documentation.

use syn::{Fields, Item, Meta};

use crate::model::class_object::{Class, Method};

/// RustDocParser struct used for parsing Rust code documentation.
#[derive(Default)]
pub struct RustDocParser {
    pub(crate) raw_rust_code: String,
}

impl RustDocParser {
    /// Parses the given Rust code documentation and returns it in Markdown format.
    ///
    /// This function primarily focuses on extracting documentation of structs
    /// and their named fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate_name::RustDocParser;
    /// let rust_code = r#"
    ///     /// A sample struct.
    ///     struct Sample {
    ///         /// A sample field.
    ///         field: String,
    ///     }
    /// "#;
    /// let parser = RustDocParser { raw_rust_code: rust_code.to_string() };
    /// let markdown = parser.parse_code_doc_to_markdown_string();
    /// assert!(markdown.contains("A sample struct."));
    /// ```
    ///
    /// # Returns
    ///
    /// A string containing the code documentation formatted as Markdown.
    pub fn parse_code_doc_to_markdown_string(&self) -> String {
        let mut markdown = String::new();

        let result = self.parse_code_doc();
        markdown.push_str(&format!("## {}\n\n", result.name));
        markdown.push_str(&result.documentation.to_string());
        markdown
    }

    /// Parses the Rust code documentation and returns a representation in the form of a `Class` object.
    ///
    /// This function will extract the name and documentation of structs, and for each named field
    /// inside a struct, it will treat it as a "method" with its respective documentation.
    ///
    /// # Returns
    ///
    /// A `Class` instance representing the parsed Rust documentation.
    pub fn parse_code_doc(&self) -> Class {
        let parsed_file = syn::parse_file(&self.raw_rust_code).unwrap();

        let mut struct_name = String::new();
        let mut struct_documentation = String::new();
        let mut methods = Vec::new();
        for item in parsed_file.items {
            if let Item::Struct(item_struct) = item {
                struct_name.push_str(&format!("{}", item_struct.ident));
                for attribute in item_struct.attrs {
                    let meta = attribute.parse_meta().unwrap();
                    add_name_value_to_documentation(&mut struct_documentation, meta);
                }
                struct_documentation.push('\n');

                // Collect information about methods and their documentation
                if let Fields::Named(fields) = &item_struct.fields {
                    for field in &fields.named {
                        let method_name = field.ident.as_ref().unwrap().to_string();
                        let mut method_documentation = String::new();

                        for attribute in &field.attrs {
                            let meta = attribute.parse_meta().unwrap();
                            add_name_value_to_documentation(&mut method_documentation, meta);
                        }

                        let method = Method {
                            name: method_name,
                            documentation: method_documentation,
                        };

                        methods.push(method);
                    }
                }
            }
        }
        Class {
            plantuml: String::new(),
            name: struct_name,
            documentation: struct_documentation,
            methods,
        }
    }
}

fn add_name_value_to_documentation(documentation: &mut String, meta: Meta) {
    if let Meta::NameValue(name_value) = meta {
        if name_value.path.is_ident("doc") {
            if let syn::Lit::Str(lit_str) = name_value.lit {
                documentation.push_str(lit_str.value().trim());
                documentation.push('\n');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Code string to use it in the tests
    fn test_rust_code() -> String {
        String::from(
            r#"
            /// This is a doc comment
            /// over multiple lines
            struct TestStruct {
                /// This is a doc comment
                field: String,
            }
            "#,
        )
    }

    #[test]
    fn test_parse_code_doc_to_markdown_string_struct_title() {
        let expected_markdown =
            String::from("## TestStruct\n\nThis is a doc comment\nover multiple lines\n\n");

        let parser = RustDocParser {
            raw_rust_code: test_rust_code(),
        };
        let markdown = parser.parse_code_doc_to_markdown_string();

        assert_eq!(markdown, expected_markdown);
    }

    #[test]
    fn test_parse_code_doc_name_and_documentation() {
        let expected_struct_name = String::from("TestStruct");
        let expected_struct_documentation =
            String::from("This is a doc comment\nover multiple lines\n\n");

        let parser = RustDocParser {
            raw_rust_code: test_rust_code(),
        };
        let class_object = parser.parse_code_doc();

        assert_eq!(class_object.name, expected_struct_name);
        assert_eq!(class_object.documentation, expected_struct_documentation);
    }
}
