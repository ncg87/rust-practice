// Fix the code so that it compiles.

fn main() {
    // mutable since we borrow them as mutable references
    let mut str1 = String::from("Rust");
    let mut str2 = String::from("Golang");
    let ref1 = &mut str1;
    let mut ref2 = &mut str2; // mutable since we swap the mutable references

    println!("First string: {ref1}");
    println!("Second string: {ref2}");
    ref1.push('🦀');
    ref2.push('🦫');
    println!("Modified first string: {ref1}");
    println!("Modified second string: {ref2}");
    // only one mutable reference allowed at a time, ref1 is no longer valid
    ref2 = &mut str1; // this is why ref2 is mutable
    ref2.pop();
    println!("Original first string: {ref2}");
}
