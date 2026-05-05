pub fn get_string_lenght(s: &str) -> usize {
    let mut count = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}
