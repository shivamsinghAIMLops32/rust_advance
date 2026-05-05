// stack is allocated at compile time => number,bool etc data which wont change at run time or dynamically allocation

// heap is allocated at run time => string, vector, hashmap etc data which can change at run time or dynamically allocation

pub fn stack_heaper() {
    let x = 5; // allocated on stack
    println!("Value of x: {}", x);
    let mut s = String::from("Hello, world!"); // allocated on heap
    println!("Value of s: {}", s);
    s.push_str(" Welcome to Rust!"); // modifying the string on heap
    println!("Modified string: {}", s);
}
