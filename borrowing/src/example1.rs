// Fix the code so that it compiles.

fn main() {
    // a mutable structure can be borrowed as a mutable reference or a shared reference
    let mut str1 = String::from("modifiable");
    // a regular variable can only be borrowed as a shared reference
    let str2 = String::from("fixed string");
    let mut str_ptr: &String;
    str_ptr = &str1;
    println!("ptr currently points to {str_ptr}");
    str_ptr = &str2;
    println!("ptr currently points to {str_ptr}");
    str1.push_str(" string");
    str_ptr = &str1;
    println!("ptr currently points to {str_ptr}");
}
