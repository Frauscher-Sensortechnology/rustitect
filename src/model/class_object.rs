/// Represents a class in the code, including its associated methods and documentation.
pub struct Class {
    /// The PlantUML diagram for the class.
    pub plantuml: String,
    /// The name of the class.
    pub name: String,
    /// The documentation for the class.
    pub documentation: String,
    /// The fields associated with the class.
    pub fields: Vec<Method>,
    /// The methods associated with the class.
    pub methods: Vec<Method>,
}

/// Represents a method within a class, including its name and documentation.
#[derive(Debug, PartialEq)]
pub struct Method {
    /// The name of the method.
    pub name: String,
    /// The documentation for the method.
    pub documentation: String,
}
