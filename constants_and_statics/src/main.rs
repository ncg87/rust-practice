
const MAX_PLAYERS:u8 = 10; //cant be mutated, must be known at compile time
 // Constants types must be explicitly specified

// Static variables can be mutated, but must be done in a unsafe block (What is a unsafe block?)
static mut CASINO_NAME:&str = "The Casino";
 // Also must be explicitly specified type
fn main() {

    // Constants don't occupy stack, they are stored in the binary file
    let a = MAX_PLAYERS;
    let b = MAX_PLAYERS;    
    println!("{}", a);
    println!("{}", b);
    // Statics do occupy stack, and are associated with the module
    // Only one instance of a static variable exists in the binary file
    unsafe { // need to be in unsafe block to mutate static variable (why?)
        let c = CASINO_NAME;
        let d = CASINO_NAME;
    }
}