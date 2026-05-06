pub fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    // return an shortest lifespan one validity of the reference
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}

// lifetimes with structs

struct User2<'a> {
    name: &'a str,
}

pub fn iml_user() {
    let name = String::from("Alice");
    let user1 = User2 { name: &name };
    println!("User name is {}", user1.name);
}
