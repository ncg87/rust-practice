use modules::Credentials;
fn main() {
    let cred = Credentials::new("user".to_owned(), "pass".to_owned());
    modules::authenticate(cred);
}

