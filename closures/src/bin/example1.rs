// Define the `double` closure & make the code execute successfully.
// closures are defined as anonymous functions
fn main() {
    let double = |x: i32| x * 2;
    assert_eq!(double(5), 10);
    assert_eq!(double(-10), -20);
}
