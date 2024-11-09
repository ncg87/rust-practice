// Implement the add method for Pair<i32> type.
use std::ops::Add;


struct Pair<T>(T, T); // tuple must have two element of same type
impl<T> Pair<T> // we are implementing for any type T
where T: Add<Output = T> + Copy // T must implement Add trait and be copyable
// ^^ both are std traits, where is used to specify the trait bounds
// says where type T implements the traits add and copy
{
    fn add(&self) -> T {
        self.0 + self.1
    }             
}

fn main() {
    let p1 = Pair(10, 23);
    let addition = p1.add();
    assert_eq!(addition, 33);
}
