// Orphan rule: A trait cannot be implemented for a type that is defined outside of it
use orphan_rule::Point;
// can only get around this by useing a wrapper

struct PointWrapper(Point);

// we can implement the trait for the wrapper since it is defined in our crate/scope
impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}
// wrapper is sort of redefining an object in a new scope
fn main() {
    let p1 = PointWrapper(Point { x: 1, y: 2 });
    let p2 = PointWrapper(Point { x: 1, y: 2 });

    println!("{}", p1 == p2);
}
