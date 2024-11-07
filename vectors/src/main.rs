fn main() {
    let mut v = Vec::new(); // can also explicitly declare the type of the vector
    v.push(String::from("One")); // by pushing an element complier can infer the type of the vector
    v.push(String::from("Two"));
    v.push(String::from("Three"));
    

    let v2 = vec![1, 2, 3]; // can also initialize a vector with values, similar to array

    // has to be address since string doesn't implement copy trait
    let s = &v[0]; // can panic if the index is out of bounds
    println!("{s}");

    let s = v.remove(0); // removes the element at index 0 and returns it
    println!("{s}"); // prints the removed element

    // safer option
    let s = v.get(1); // returns an option enum with the element at index 1
    if let Some(e) = s { // matches to either some or none
        println!("{e}");
    }

    // can also iterate over the vector
    for e in &mut v {
        // we are using a mutable reference to the vector
        e.push_str("!");
    }
    // using shared references
    for s in &v {
        println!("{s}");
    }
    let mut v3 = vec![]
    for s in v { // moving all values from v
        v3.push(s)
    }
} // v is dropped when it goes out of scope
