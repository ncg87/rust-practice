// Complete the code by deriving the required traits.

// Debug, PartialEq, and Default traits are derived
#[derive(Debug, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let my_point = Point { x: 20, y: 10 };
    let origin = Point::default();
    println!("Origin: {origin:?}");
    if my_point == origin {
        println!("Selected point is origin!");
    } else {
        println!("Selected point: {my_point:?}");
    }
}
