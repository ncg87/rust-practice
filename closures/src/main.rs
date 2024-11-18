// we can use a closure as a type parameter, closure trait is Fn
struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T, // we defined the type T as a closure that takes two &str and returns a boolean
}

impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    // implements the closure trait
    fn is_valid(&self) -> bool {
        // to call closure we must use ()
        (self.validator)(&self.username, &self.password)
    }
}

// depending of the closure, 
// FnMut - it can take a mutable reference to the variable
// Fn - it can take a immutable reference to the variable
// FnOnce - it can take ownership of the variable
// ^^ are infered by the compiler based on how the closure is used
fn main () {
    // we can save an operation in a closure and reuse it and pass it around to variables and functions
    let validator = |username: &str, password: &str| -> bool {
        !username.is_empty() && !password.is_empty()
    };
    let weak_password = "password123!".to_owned();
    // closure can use variables from the outer scope
    // move keyword is used to take ownership of the variable
    let validator2 = |username: &str, password: &str| -> bool {
        !username.is_empty()
        && !password.is_empty()
        && password.contains(['!','@','#','$','%','^','&','*'])
        && password != weak_password // take a immutable reference to the variable
    };
    println!("{:?}", weak_password);

    let creds = Credentials {
        username: "user".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2,
    };
    // this is a closure, we can store and pass closures
    
    println!("{}", creds.is_valid());
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}