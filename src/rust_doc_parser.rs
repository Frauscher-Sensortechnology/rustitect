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
                    //TODO iterate over attributes and get doc comments and add them to markdown
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
            struct TestStruct {
                /// This is a doc comment
                field: String,
            }
            "#,
        );
        let markdown = RustDocParser::parse_code_doc_to_markdown_string(&raw_rust_code);
        assert_eq!(markdown, "## TestStruct\n\n\n");
    }
}