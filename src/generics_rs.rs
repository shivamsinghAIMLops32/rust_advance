// a generic function which adds either two numbers or concatenates two strings
// dont overcomplciate syntax make it jsut simple as possible
use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// compartor
fn return_bigger<T: PartialOrd + Clone>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
