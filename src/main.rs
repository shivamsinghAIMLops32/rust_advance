use crate::struct_prac::User;
use std::{fs::read_to_string, vec};

mod collections_rust;
mod enum_prac;
mod generics_rs;
mod impl_iterator;
mod interator_prac;
mod lifetime_rust;
mod stack_heap;
mod string_lenght;
mod string_slices;
mod struct_prac;
mod traits_rs;
mod multithreading;
mod display_trait;
fn main() {
    stack_heap::stack_heaper();
    println!("Fibonacci of 10 is: {}", fib(10));
    println!("Fibonacci of 10 is: {}", fib_dp(10));
    let s = String::from("Hello, world!");
    println!(
        "String length of '{}' is: {}",
        s,
        string_lenght::get_string_lenght(&s)
    );
    let u = User::new("Shivam".to_string(), 20);
    println!("{}", u.name());
    println!("{}", u.age());

    // enum_prac::Direction::North;
    let direction = enum_prac::Direction::North;

    println!("Direction: {:?}", direction);

    let circle = enum_prac::Shape::Circle(7.0);
    let rectangle = enum_prac::Shape::Rectangle(5.0, 10.0);
    let _square = enum_prac::Shape::Square(4.0);
    println!("Area of Circle: {:.2}", circle.area());
    println!("Area of Rectangle: {:.2}", rectangle.area());

    // find first "a" in a  string
    let s3 = String::from("Hello, world!");
    let _ans3: () = match string_lenght::find_first_a(&s3) {
        Some(index) => println!("First 'a' found at index: {}", index),
        None => println!("No 'a' found in the string."),
    };

    let content_from_file: Result<String, std::io::Error> = read_to_string("src/test.txt");
    match content_from_file {
        Ok(content) => println!("Content of the file:\n{}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    let arr = vec![1, 2, 3, 4, 5, 6];
    let even_numbers = collections_rust::even_filter(arr);
    println!("Even numbers: {:?}", even_numbers);

    // let _ans;
    let s10 = String::from("Hello");
    {
        let s11 = String::from("World!");
        // ans = lifetime_rust::longest(&s10, &s11);
    }
    // println!("Longest string: {}", ans); // this tool lesser lifetime of s11 and s10 but it is valid because it is in the same scope as s10 and s11
}

fn fib(n: u32) -> u32 {
    return if n <= 1 { n } else { fib(n - 1) + fib(n - 2) };
}

fn fib_dp(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut prev = 0; // fib(0)
    let mut curr = 1; // fib(1)

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}
