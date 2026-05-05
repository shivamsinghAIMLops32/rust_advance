use crate::struct_prac::User;

mod string_lenght;
mod struct_prac;
    
fn main(){
println!("Fibonacci of 10 is: {}", fib(10));
println!("Fibonacci of 10 is: {}", fib_dp(10));
let s = String::from("Hello, world!");
println!("String length of '{}' is: {}", s, string_lenght::get_string_lenght(&s));
let u = User::new("Shivam".to_string(), 20);
println!("{}", u.name());
println!("{}", u.age());
}

fn fib(n:u32)->u32{
   return if n <= 1 { n } else { fib(n-1) + fib(n-2) };
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

