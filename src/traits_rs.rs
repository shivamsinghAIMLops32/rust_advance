pub trait Animalsfeatures {
    fn bark(&self); // we can write this func here too and later whoever is implementing this trait will have to implement this func as well
    fn color(&self) -> &str;
}

// struct dog
pub struct Dog {
    name: String,
    age: u8,
    color: String,
}
// implement a constructor for dog
impl Dog {
    pub fn new(name: String, age: u8, color: String) -> Self {
        Dog { name, age, color }
    }

    // function to change the name of dog
    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn getter_name(&self) -> &str {
        &self.name
    }
    pub fn getter_age(&self) -> u8 {
        self.age
    }
}
// implement the bark trait for dog
impl Animalsfeatures for Dog {
    fn bark(&self) {
        println!("{} says: Woof!", self.name);
    }
    fn color(&self) -> &str {
        &self.color
    }
}
