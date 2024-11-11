fn main() {
    let s1 = String::from("hello");
    println!("s1: {}", s1);
    let mut s2 = s1; // lifetime of s1 ends here, since s2 takes ownership of s1
    {
        let s4 = &mut s2;
        // lifetime of s3 starts here
        let s3 = String::from("world");
        println!("s3: {}", s3);
        // lifetime of s3 ends here
        s4.push_str(" world");
        // lifetime of s4, mutable reference ends here
    }
    let s5 = &mut s2;

    println!("s2: {}", s5); // dereferencing is automatic handled

    // lifetime of s6 starts here
    println!("s6: {}", s2);
}
