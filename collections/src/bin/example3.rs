// Fix the code to make it compile.

use std::collections::HashMap;

fn main() {
    // marks scored out of 50
    let mut marks = HashMap::from([("Harry", 40.0), ("Hermoine", 50.0), ("Ron", 35.5)]);
    // convert marks into percentage
    for (_, marks) in marks.iter_mut() {
        *marks = (*marks * 100.0) / 50.0;
    }
    // need to convert to an iterator to for_each
    // for each executes the closure for each element in the iterator, returns unit ()
    // map returns a new iterator, used for transforming elements
    marks.into_iter().for_each(|(student, marks)| println!("{student} scored {marks}%"));
}
