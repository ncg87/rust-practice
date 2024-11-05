fn main() {
    let z = my_function(10);
    println!("my function returned: {}", z);
}

fn my_function(x: i32) -> i32 {
    println!("my_function called with: {}", x);
    let y = 32;
    y // last expresion doesn't need a semicolon to return
}
