pub fn slices_from_string() {
    // slices are refrence part of string from an index to another
    // here original borrowing concept ticks as you will refrence it from original string then you cant mutate or chnage original string

    let s = String::from("HELLO WORLD");
    let slice1 = &s[0..5]; // HELLO
    let slice2 = &s[6..11]; // WORLD
    // saves memory as we are not creating new string but just referencing part of original string
    println!("Slice 1: {}", slice1);
    println!("Slice 2: {}", slice2);
}

pub fn find_first_a(s: &str) -> &str {
    // find first "a" in a string and return the slice from that index to end of string
    for (i, c) in s.char_indices() {
        if c == 'a' {
            return &s[i..];
        }
    }
    return "No 'a' found in the string.";
}
