use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice".to_owned(), 10);
    scores.insert("Bob".to_owned(), 20);
    scores.insert("Jim".to_owned(), 30);

    // rust infers that the iterator should be immutable
    for (key, value) in &scores { // if we borrow mutably we get mutable references
        println!("{}: {}", key, value);
    }
    // mut_iter returns a mutable iterator with mutable references to the values
    let scores_iter3 = scores.iter_mut();
    for (key, value) in scores_iter3 {
        *value += 1;
        println!("{}: {}", key, value);
    }

    // iter does not take ownership of the values
    let score_iter = scores.iter();
    for (key, value) in score_iter {
        println!("{}: {}", key, value);
    }
    // into_iter takes ownership of the values
    let score_iter2 = scores.into_iter();
    for (key, value) in score_iter2 {
        println!("{}: {}", key, value);
    }

}
