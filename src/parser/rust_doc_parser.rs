//! A module for parsing Rust code documentation and generating Markdown documentation.

use syn::__private::quote::quote;
use syn::{Fields, FieldsNamed, ImplItem, Item, Meta};

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
        let mut methods_vector = Vec::new();
        let mut fields_vector = Vec::new();
        for item in parsed_file.items {
            match item {
                Item::Struct(item_struct) => {
                    struct_name.push_str(&format!("{}", item_struct.ident));
                    for attribute in item_struct.attrs {
                        let meta = attribute.parse_meta().unwrap();
                        add_name_value_to_documentation(&mut struct_documentation, meta);
                    }
                    struct_documentation.push('\n');

                    // Collect information about fields and their documentation
                    if let Fields::Named(fields) = &item_struct.fields {
                        fields_vector = collect_fields(fields.clone());
                    }
                }
                Item::Impl(item_impl) => {
                    if item_impl.trait_.is_none() {
                        let collected_methods: Vec<Method> = collect_methods(item_impl.items);
                        methods_vector.extend(collected_methods);
                    }
                }
                _ => {}
            }
        }
        Class {
            plantuml: String::new(),
            name: struct_name,
            documentation: struct_documentation,
            fields: fields_vector,
            methods: methods_vector,
        }
    }
}
fn collect_fields(fields: FieldsNamed) -> Vec<Method> {
    let mut fields_vector = Vec::new();
    for field in &fields.named {
        let method_name = field.ident.as_ref().unwrap().to_string();
        let mut fields_documentation = String::new();

        for attribute in &field.attrs {
            let meta = attribute.parse_meta().unwrap();
            add_name_value_to_documentation(&mut fields_documentation, meta);
        }

        let method = Method {
            name: method_name,
            documentation: fields_documentation,
        };

        fields_vector.push(method);
    }
    fields_vector
}

fn collect_methods(impl_items: Vec<ImplItem>) -> Vec<Method> {
    impl_items
        .into_iter()
        .filter_map(|item| {
            if let ImplItem::Method(method) = item {
                let method_name = method.sig.ident.to_string();
                let parameters: Vec<String> = method
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|input| match input {
                        syn::FnArg::Typed(pat_type) => {
                            let parameter_name = match *pat_type.pat.clone() {
                                syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                                _ => return None,
                            };
                            let parameter_type = pat_type.ty.clone();
                            let parameter_type_string = quote!(#parameter_type).to_string();
                            Some(format!("{}: {}", parameter_name, parameter_type_string))
                        }
                        _ => None,
                    })
                    .collect();
                let method_name = format!("{}({})", method_name, parameters.join(", "));

                let mut method_documentation = String::new();
                for attribute in &method.attrs {
                    let meta = attribute.parse_meta().unwrap();
                    add_name_value_to_documentation(&mut method_documentation, meta);
                }

                Some(Method {
                    name: method_name,
                    documentation: method_documentation,
                })
            } else {
                None
            }
        })
        .collect()
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
    use crate::model::class_object;

    use super::*;

    /// Code string to use it in the tests
    fn test_rust_code() -> String {
        String::from(
            r#"
            /// This is a doc comment
            /// over multiple lines
            struct TestStruct {
                /// This is a doc comment of field1
                field1: String,
                /// This is a doc comment of field2
                field2: String,
            }

            impl TestStruct {
                /// Create a new TestStruct
                pub fn new(field1: String, field2: String,) -> Self {
                    TestStruct {
                        field1,
                        field2,
                    }
                }
                /// Another method
                pub fn another_method() -> Self {
                    do_something()
                }
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

    #[test]
    fn test_parse_code_field_name_and_documentation() {
        let expected_fields = vec![
            class_object::Method {
                name: "field1".to_string(),
                documentation: "This is a doc comment of field1\n".to_string(),
            },
            class_object::Method {
                name: "field2".to_string(),
                documentation: "This is a doc comment of field2\n".to_string(),
            },
        ];
        let expected_amount_of_fields = expected_fields.len();

        let parser = RustDocParser {
            raw_rust_code: test_rust_code(),
        };
        let class_object = parser.parse_code_doc();

        assert_eq!(class_object.fields.len(), expected_amount_of_fields);
        assert_eq!(class_object.fields, expected_fields);
    }

    #[test]
    fn test_parse_code_method_name_and_documentation() {
        let expected_methods = vec![
            class_object::Method {
                name: "new(field1: String, field2: String)".to_string(),
                documentation: "Create a new TestStruct\n".to_string(),
            },
            class_object::Method {
                name: "another_method()".to_string(),
                documentation: "Another method\n".to_string(),
            },
        ];
        let expected_amount_of_fields = expected_methods.len();

        let parser = RustDocParser {
            raw_rust_code: test_rust_code(),
        };
        let class_object = parser.parse_code_doc();

        assert_eq!(class_object.methods.len(), expected_amount_of_fields);
        assert_eq!(class_object.methods, expected_methods);
    }
}
