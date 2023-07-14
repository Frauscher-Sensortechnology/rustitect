use syn::Item;
use crate::model::class_object::Class;

/// RustDocParser struct used for parsing Rust code documentation.
#[derive(Default)]
pub struct RustDocParser {
    pub(crate) raw_rust_code: String,
}
impl RustDocParser {

    /// Parses the code documentation and returns it as a Markdown-formatted string.
    ///
    /// # Returns
    ///
    /// The code documentation formatted as a Markdown string.
    pub fn parse_code_doc_to_markdown_string(&self) -> String {
        let mut markdown = String::new();

        let result = self.parse_code_doc();
        markdown.push_str(&format!("## {}\n\n", result.name));
        markdown.push_str(&result.documentation.to_string());
        markdown
    }

    /// Parses the code documentation and returns the Class struct containing the parsed information.
    ///
    /// # Returns
    ///
    /// The Class struct representing the parsed code documentation.
    pub fn parse_code_doc(&self) -> Class {
        let parsed_file = syn::parse_file(&self.raw_rust_code).unwrap();

        let mut struct_name= String::new();
        let mut struct_documentation= String::new();
        for item in parsed_file.items {
            match item {
                Item::Struct(item_struct) => {
                    struct_name.push_str(&format!("{}", item_struct.ident));
                    for attribute in item_struct.attrs {
                        let meta = attribute.parse_meta().unwrap();

                        if let syn::Meta::NameValue(name_value) = meta {
                            if name_value.path.is_ident("doc") {
                                if let syn::Lit::Str(lit_str) = name_value.lit {
                                    struct_documentation.push_str(&lit_str.value().trim());
                                    struct_documentation.push('\n');
                                }
                            }
                        }
                    }
                    struct_documentation.push('\n');
                },
                _ => {}
            }
        }
        return Class {
            plantuml: String::new(),
            name: struct_name,
            documentation: struct_documentation,
            methods: Vec::new(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Code string to use it in the tests
    fn test_rust_code() -> String {
        String::from(r#"
            /// This is a doc comment
            /// over multiple lines
            struct TestStruct {
                /// This is a doc comment
                field: String,
            }
            "#)
    }

    #[test]
    fn test_parse_code_doc_to_markdown_string_struct_title() {
        let expected_markdown = String::from(
            "## TestStruct\n\nThis is a doc comment\nover multiple lines\n\n"
        );

        let parser = RustDocParser { raw_rust_code: test_rust_code() };
        let markdown = parser.parse_code_doc_to_markdown_string();

        assert_eq!(markdown, expected_markdown);
    }

    #[test]
    fn test_parse_code_doc_name_and_documentation() {
        let expected_struct_name = String::from("TestStruct");
        let expected_struct_documentation = String::from("This is a doc comment\nover multiple lines\n\n");

        let parser = RustDocParser { raw_rust_code: test_rust_code() };
        let class_object = parser.parse_code_doc();

        assert_eq!(class_object.name, expected_struct_name);
        assert_eq!(class_object.documentation, expected_struct_documentation);
    }
}