fn main() {
    // Integer can be signed, i32, (positive or negative) or unsigned, u32 (positive)

// Creation
let a:i32 = 10;

// Mutable variable
let mut b:i32 = 50;

// Shadowing
let c = 100; // Can only be access if there is a reference to it

let y = &c; // y is a reference to first value of c

let c = 200; // Doesn't replace the previous value of c but leave it in memory, only accessable through reference

println!("The value of c is: {c}");

let d = 400;
//scope
{                   
    let d = 300; // d is only accessable within this scope
    println!("The value of d is: {d} in the scope"); 

    let e = 500; // e is only accessable within this scope

}
println!("The value of d is: {d} outside the scope");
}
