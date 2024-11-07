fn main() {
    let username = get_username(0);
    match username {
        // binds the username to the name variable
        Some(name) => println!("Username: {}", name),
        None => println!("No username found"),
    }
    let username = get_username(1); // need to reassign the variable since ownership is moved from before
    // if let syntax uses pattern matching, creates a variable name from the Some value
    if let Some(name) = username { // if it matches the pattern, some variant the variable name will be assigned
        println!("Username: {}", name);
    }
}

fn get_username(user_id: i32) -> Option<String> {
    let db_results = String::from("John");
    if user_id == 1 {
        return Some(db_results);
    }
    None

}
