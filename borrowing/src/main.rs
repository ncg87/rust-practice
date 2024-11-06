fn main() {
    // needs to be mutable to borrow a mutable reference
    let mut s1 = String::from("Rust");
    let r1 = &s1; // cloning is not efficient
    print_string(r1); // immutable reference is out of scope after this function
    let r2 = &mut s1; // to borrow a mutable reference, variable must be mutable
    add_to_string(r2);
    println!("{}",s1);

    let s2 = generate_string();
}

fn generate_string() -> String { // returning a address of the string would be a dangling pointer, not allowed in rust
    String::from("Sparky")
}

// function borrows the string, as a mutable reference, so we can edit and lose ownership / go out of scope
fn add_to_string(s: &mut String) {
    s.push_str(" is great!"); // need to pass ownership out of the scope
}

// function borrows the string, as a sharable reference
fn print_string(s: &String) {
    println!("{s}"); // function doesn't need to take ownership
}
