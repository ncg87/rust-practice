// Something is wrong with the struct definition. Can you fix it?

struct Manipulator<T>
where
    T: FnMut(), // FnMut - it can take a mutable reference to the variable
{
    operation: T,
}

impl<T> Manipulator<T>
where
    T: FnMut(),
{
    fn manipulate(&mut self) {
        (self.operation)() // call the closure
    }
}
fn main() {
    let mut x = 0;
    let mut y = 100;
    let mut x_manipulator = Manipulator {
        operation: || x += 1,
    };
    let mut y_manipulator = Manipulator {
        operation: || y /= 5,
    };
    x_manipulator.manipulate();
    x_manipulator.manipulate();
    y_manipulator.manipulate();
    y_manipulator.manipulate();
    assert_eq!(x, 2);
    assert_eq!(y, 4);
}
