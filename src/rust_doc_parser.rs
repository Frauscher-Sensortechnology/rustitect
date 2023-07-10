use syn::Item;
use syn::visit::{self, Visit};

pub struct RustDocParser {}

impl RustDocParser {
    pub fn parse_code_doc_to_markdown_string(raw_rust_code: &String) -> String {
        let parsed_file = syn::parse_file(raw_rust_code).unwrap();
        let mut markdown = String::new();

        for item in parsed_file.items {
            match item {
                Item::Struct(item_struct) => {
                    markdown.push_str(&format!("## {}\n\n", item_struct.ident));
                    for attribute in item_struct.attrs {
                        let meta = attribute.parse_meta().unwrap();

                        if let syn::Meta::NameValue(name_value) = meta {
                            if name_value.path.is_ident("doc") {
                                if let syn::Lit::Str(lit_str) = name_value.lit {
                                    markdown.push_str(&lit_str.value().trim());
                                    markdown.push('\n');
                                }
                            }
                        }
                    }
                    markdown.push('\n');
                }
                _ => {}
            }
        }
        return markdown;
    }
}

//Test parse_code_doc_to_markdown_string
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_code_doc_to_markdown_string_struct_title() {
        let raw_rust_code = String::from(
            r#"
            /// This is a doc comment
            /// over multiple lines
            struct TestStruct {
                /// This is a doc comment
                field: String,
            }
            "#,
        );
        let expectedMarkdown = String::from(
            "## TestStruct\n\nThis is a doc comment\nover multiple lines\n\n\
         ");

        let markdown = RustDocParser::parse_code_doc_to_markdown_string(&raw_rust_code);

        assert_eq!(markdown, expectedMarkdown);
    }
}