// Add a type alias for our dogs so we can store their names and ages.


// Could use a struct instead of a type alias but type alias is more scope dependent
fn main() {
    // type alias for a tuple of a String and an i32
    type Dog = (String, i32);
    // initialize a Dog tuple
    let dog1: Dog = (String::from("Albert"), 3);
    println!("My dog {} is {} years old.", dog1.0, dog1.1);

    let dog2: Dog = (String::from("Barker"), 5);
    println!("My other dog {} is {} years old.", dog2.0, dog2.1);
}