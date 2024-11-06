// Fix the code so that it compiles. Modify only one statement.

fn main() {
    let mut my_str = String::from("Example");
    let mut temp;
    while my_str.len() > 0 {
        temp = my_str.clone(); // creates a deep copy of my_str because it is needed for the loop
        println!("Length of temporary string is: {}", temp.len());
        my_str.pop(); // ^^ needs to be cloned since we are mutating the OG my_str
        // a value can only have one owner
    }
}
