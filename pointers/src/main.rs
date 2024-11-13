use std::ops::{Deref, DerefMut};
// returns a reference to the value it is pointing to or a mutable reference to the value it is pointing to

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> Self {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T; // have to specify the type because T is a generic type, Target is the type of the value it is pointing to
    fn deref(&self) -> &Self::Target {
        &self.value // returns a reference to the value it is pointing to
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value // returns a mutable reference to the value it is pointing to
    }
}

fn main() {
    let mut s = MySmartPointer::new(Box::new("Hello, world!".to_owned()));
    // &MySmartPointer -> &Box -> &String -> &str, works because they implement the Deref trait

    //let s = &(***s); // dereferencing the pointer 3 times

    print_string(&mut s); // works because of deref coercion. which works implictly
    // can pass a mutable reference to the an immutable reference in function but not vice versa
}

fn print_string(s: &str) {
    println!("{}", s);
}
