fn main() {
    // a fn that takes a number as an input and returns if number is even or odd
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("failed to take input");
    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    let answer = is_even_or_odd(num);
    println!("The number is {}", answer);
}

fn is_even_or_odd(num:i32)->&'static str{
    if num % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }

}