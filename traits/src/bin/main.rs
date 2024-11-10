
// derive is used to implement the Debug and PartialEq traits
// macros that automatically implement traits
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println!("{:?}", p1); // Debug trait is implemented
    println!("{}", p1 == p2); // PartialEq trait is implemented
    println!("{}", p1 == p3); // PartialEq trait is implemented
}