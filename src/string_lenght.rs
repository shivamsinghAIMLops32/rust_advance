pub fn get_string_lenght(s: &str) -> usize {
    let mut count = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}


pub fn find_first_a(s: &str) -> Option<usize> {
    for (i, c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(i);
        }
    }
    None
}