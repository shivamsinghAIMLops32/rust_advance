// implemetn custom iterator for an array of object or struct

use std::string;

struct User {
    name:String,
    age:u32,
}

struct UserList {
    users:Vec<User>,
    index:usize,
}

impl Iterator for UserList {
    type Item = User;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.users.len() {
            let user = self.users[self.index].name.clone();
            self.index += 1;
            Some(User { name: user, age: self.users[self.index].age })
        } else {
            None
        }
    }
}