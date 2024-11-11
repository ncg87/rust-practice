// Fix the code by addressing the TODO.

fn main() {
    let num_ref;
    {
        // TODO: shift below statement to appropriate location
        let num = 23;
        num_ref = num; // shift ownership to num_ref
    }
    println!("Reference points to {}", num_ref);
}
