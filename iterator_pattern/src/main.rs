trait Iterator {
    type Item; // associated type
    // Only necessary method for an iterator
    fn next(&mut self) -> Option<Self::Item>;
}

trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter {
        
    }
}

struct MyStruct{}
// if the iterator is restricted to a single type use associated type
// if you want to iterate over multiple types, don't use associated type use generics
impl Iterator for MyStruct {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    let mut m = MyStruct{};
    let item = m.next();
}
