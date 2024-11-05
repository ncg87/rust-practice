fn main() {
    // if / else
    let a = 5;
    if a > 5 { // If statements must be boolean
        println!("a is greater than 5")
    } else if a > 3{
        println!("a is greater than 3")
    } else {
        println!("a is less than or equal to 3")
    }

    // if statements can be used to assign values
    let b:u32 = if a > 5 {
        1 // the return values must be of the same type
    } else {
        0
    }; // semicolon is needed to end the if statement
    println!("b is {}", b);


    // loops and named loops
    'outer: loop { // can label loops to specify breaking out of them
        println!("looping forever!");
        'inner: loop { // specified using explict lifetimes
            break 'outer; // break is used to exit the loop
            // this additionally breaks the inner loop since it is nested within
        }
    }

    // loop can return a value
    let x = loop {
        break 99;
    }; // need a semicolon to signify the return value and end of let
    println!("x is {}", x);

    println!("done");

    // while loop
    let mut a = 0;
    while  a < 5 {
        println!("a is {}", a);
        a += 1;
    }

    // for loop(used to iterate through a collection)

    for i in 0..5 { // way to iterate over a range
        println!("i is {}", i);
    }

    // for loop with a collection
    let mut collection = [1, 2, 3, 4, 5];
    for i in &mut collection { // can mutate the element in the collection, using mutable reference
        *i += 1; // need to dereference i to mutate it
        println!("i is {}", i);
    }
}