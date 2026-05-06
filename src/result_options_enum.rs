// suppose we have a db call to find out user in db so it can have two possible outcome,so varible would have some value or none
//    {
//          1. user is found in db => Some(user)
//          2. user is not found in db => None
//     }

pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

pub fn find_user_db(user_id: u32) -> Option<User> {
    let db = vec![
        User { id: 1, name: "Shivam".to_string(), age: 20 },
        User { id: 2, name: "John".to_string(), age: 30 },
        User { id: 3, name: "Alice".to_string(), age: 25 },
    ];

    for user in db {
        if user.id == user_id {
            return Some(user);
        }
    }
    None
}

// another example using iterate over result 
pub fn find_user_db_iterable(user_id: u32) -> Option<User> {
    let db = vec![
        User { id: 1, name: "Shivam".to_string(), age: 20 },
        User { id: 2, name: "John".to_string(), age: 30 },
        User { id: 3, name: "Alice".to_string(), age: 25 },
    ];

    match db.into_iter().find(|user| user.id == user_id) {
        Some(user) => Some(user),
        None => None,
    }

}





// implimenting result enum => which  give us either success or failure of an operation
// {
//     Ok(value) => value,  //success case
//     Err(error) => {
//         println!("Error: {}", error);
//         return;     // or handle the error as needed
//     }
// }

// we use it mostly in file handling or network, where it can give error

pub fn read_to_string(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}

pub fn run_file_read_example() {
    match read_to_string("src/test.txt"){
        Ok(content) => println!("Content of the file:\n{}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}