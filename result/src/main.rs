fn main() {
    let username = get_username(1);
    if let Some(name) = username {
        println!("Username: {}", name);
    }
}

// Option is used for value or no value
fn get_username(user_id: u32) -> Option<String> {
    let query = String::from("GET username FROM users WHERE id = {user_id}");
    let db_result = query_db(&query);
    db_result.ok() // Converts the result to an option enum
}

// Result is used for value or error
fn query_db(query: &str) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Empty query"))
    } else {
        Ok(String::from("John"))
    }
}