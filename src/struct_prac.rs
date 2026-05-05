// struct are similar to objects in js and they use to create custom data types in rust adn group related data together
// user.rs
pub struct User {
     name: String,
    age: u32,
}
impl User {
     pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn age(&self) -> u32 {
        self.age
    }
}