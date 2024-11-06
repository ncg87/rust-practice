// Fix the code so that it compiles.

fn main() {
    let mut s = String::from("Hello, ");
    let s_ref = &mut s; // need to specify that it is a mutable reference
    change_string(s_ref);
    println!("{s_ref}");
}

fn change_string(s: &mut String) { // same things
    s.push_str(" world!");
}