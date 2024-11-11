// Fix the code by shifting only one statement.

fn main() {
    let mut my_str = "Old String".to_owned();
    let ref1 = &my_str;
    println!("{ref1}"); // lifetime of ref1 ends here since it isn't used anymore
    let ref2 = &mut my_str;
    ref2.replace_range(0..3, "New");
    
    println!("{ref2}");
}
