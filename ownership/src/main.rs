fn main() {
    let s1 = String::from("Rust"); // heap allocated string

    print_value(s1.clone()); // creates a deep copy of s1 and assigns ownership to the function
    // moves ownership of s1 to s2, moves value in s1 to s2
    let s2 = s1; // moves ownership of s1 to s2, moves value in s1 to s2

    // passing ownership to function assigns ownership to the function if we pass the actual heaps value types
    print_string(&s2); 
    // s2 is the only var pointing to the value on heap
    println!("s2 is: {}", s2); // this will print "Rust"

    // Creates a clone of s2 in heap and assigns it to s3, so s3 points to it
    let s3 = s2.clone(); // creates a deep copy of s2 and assigns it to s3
    println!("s3 is: {}", s3); // this will print "Rust"
    
    let s4 = generate_string(); // creates a new string on heap and passes ownership to s4
    println!("s4 is: {}", s4); // this will print "Rust"

    let s5 = add_to_string(s4); // takes ownership of string created in function
    println!("s5 is: {}", s5); // this will print "Rust is great!"
    // s4 is not valid anymore since it was moved to s5

    // primitive types are stored on the stack
    let x = 10; // cheap and fast to clone
    print_integer(x); // the value passed is cloned into the function since it is primitive and stored on stack
    let y = x; // x is cloned into y since it is stored on the stack

    println!("x is: {}", x); // this will print "10"
    
}// everything is dropped at the end of the scope


fn print_integer(p1: i32) {
    println!("{}", p1);
}

// takes the ownership of the string
// need to declare the variable as mutable since we are modifying the string
// takes ownership and mutates the string
fn add_to_string(mut s: String) -> String {
    s.push_str("is great!");
    s // returns the modified string
}

fn generate_string() -> String {
    String::from("Rust")
}

// takes the ownership of the string
fn print_value(p1: String) {
    // string is cleaned up at the end of the scope
    println!("{}", p1);
}

// passing a reference to the string instead of passing ownership
fn print_string(p1: &str) {
    println!("{}", p1);
}
