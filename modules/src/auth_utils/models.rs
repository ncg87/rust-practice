pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    // constructor function
    pub fn new(username: String, password: String) -> Self {
        Credentials { username, password }
    }
}
