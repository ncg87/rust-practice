use std::fs::File;

fn main() {
    // usually used for testing and development, not for production code.

    // Open a file, if the file does not exist, return an error message.
    let file = File::open("file.txt").unwrap();// if error type is returned it will panic
    // rather than using match, we can use unwrap, which will panic if the file does not exist.
    let file = File::open("file.txt").expect("Failed to open file");
    // expect is similar to unwrap, but allows you to provide a custom error message.
    //let file = match file {
    //    Ok(file) => file,
    //    Err(e) => {
    //        panic!("Error opening file: {}", e)
    //    }
    //};
}

// results are recoverable errors

// Return a user id if the username is valid, otherwise return an error message.
fn get_user_id(username: &str) -> Result<i32, String> {
    // Check if the username is empty, if so return an error message.
    if username.is_empty() {
        return Err("Username is empty".to_string());
    }
    // If the username is not empty, return a user id.
    Ok(1)
}
