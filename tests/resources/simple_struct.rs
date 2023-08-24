/// Represents a person with a name, age, and activity status.
#[derive(Debug)]
struct Person {
    /// The name of the person.
    name: String,
    /// The age of the person.
    age: u32,
    /// Indicates whether the person is currently active or not.
    is_active: bool,
}

impl Person {
    /// Creates a new instance of `Person`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the person as a String.
    /// * `age` - The age of the person as a u32.
    /// * `is_active` - The activity status of the person as a boolean value.
    ///
    /// # Example
    ///
    /// ```
    /// let person = Person::new(String::from("John Doe"), 25, true);
    /// ```
    pub fn new(name: String, age: u32, is_active: bool) -> Self {
        Person {
            name,
            age,
            is_active,
        }
    }

    /// Prints a greeting message introducing the person.
    ///
    /// # Example
    ///
    /// ```
    /// let person = Person::new(String::from("John Doe"), 25, true);
    /// person.introduce();
    /// ```
    fn introduce(&self) {
        println!(
            "Hi, my name is {} and I'm {} years old.",
            self.name, self.age
        );
    }
}
