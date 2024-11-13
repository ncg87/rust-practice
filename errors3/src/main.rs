use std::{io::{self, Read}, fs::File};

fn main() {
    let content = read_file("file.txt");
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    // ? propagates error to the caller
    let mut content = String::new();
    // unwraps ok values and returns error if not ok
    File::open(filename)?.read_to_string(&mut content)?;
    Ok(content)
}

struct User {
    first_name: String,
    last_name: String,
}
fn get_initials(user: &User) -> Option<String> {
    let first_initial = user.first_name.chars().next()?; // if none is returned, None is returned early
    let last_initial = user.last_name.chars().next()?;
    Some(format!("{}{}", first_initial, last_initial))
}
