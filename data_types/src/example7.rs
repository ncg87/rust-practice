// Create an array with at least 10 elements in it.

fn main() {
    // array intializations
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // .len() returns the number of elements in the array, is defined in the standard library
    if a.len() >= 10 {
        println!("Wow, that's a big array!");
    } 
}