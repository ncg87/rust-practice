// Assign the correct data types to the variables.

fn main() {
    let pi32: f32;
    pi32 = 3.141592653589793; // since only f32 decimals are cut off
    println!("Pi is {}, correct to 2 decimal places.", pi32);

    // What if we want to be more precise with our floating-point numbers?

    let pi15:f64; /* Give this variable a data type! */;
    pi15 = 3.141592653589793;
    // need to use f64 to be more precise/ print all the decimals
    println!("Pi is {}, correct to 15 decimal places.", pi15);
}