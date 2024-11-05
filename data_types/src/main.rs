fn main() {
    // Unsigned integer (postive and zero)
    let i1:u8 = 1;
    let i2:u16 = 1;
    let i3:u32 = 1;
    let i4:u64 = 1;
    let i5:u128 = 1;
    // Platform dependent, 32-bit or 64-bit depending on system
    let i6:usize = 1;

    // Signed integer (positive or negative and zero)
    let i10:i8 = 1;
    let i11:i16 = 1;
    let i12:i32 = 1;
    let i13:i64 = 1;
    let i14:i128 = 1;
    let i15:isize = 1; // Platform dependent signed integer not used often

    // Floating point number
    let f1:f32 = 1.0; // 32-bit floating point number
    let f2:f64 = 1.0; // 64-bit floating point number
    // Only 32-64 bit floating point numbers are well supported

    // Boolean (true or false)
    let t:bool = true;
    let f:bool = false;

    // Character and strings
    let c:char = 'c'; // Single character
    let s:String = String::from("Hello"); // String, can be mutated, stored on the heap
    let h: &str = "Hello"; // String slice, always a reference

    // Arrays, hold multiple values of the same type
    let a1:[i32;3] = [1,2,3]; // Array of 3 integers, size is set at compile time
    let a2 = [1,2,3]; // Array of 3 integers, type and sizeis inferred
    let i1 = a1[0]; // Accessing the first element of the array

    // Tuples, hold multiple values of different types
    let t1:(i32,bool,f64) = (1,true,1.0); // Tuple of 3 values, size is set at compile time
    let t2 = (1,true,1.0); // Tuple of 3 values, type and size is inferred
    // Tuples are accessed by their index, '.number'
    let i2 = t1.0; // Accessing the first element of the tuple
    let i3 = t2.1; // Accessing the second element of the tuple
    
    let unit = (); // Unit type, equivalent to void

    // Type aliases, give an alternative name to a type
    type Inches = i32;
    let length:Inches = 10; // Depends on the context of the codebase/project/program
    let length2 = 15;

}
