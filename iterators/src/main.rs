// struct that will be converted into an iterator
struct Person {
    first: String,
    last: String,
    occupation: String,
}

// struct that implements the iterator trait
struct PersonIter {
    state: Vec<String>,
}

// implements the iterator trait on the PersonIter struct
impl Iterator for PersonIter {
    // associated type for the return type
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.state.pop()
    }
}

// struct that implements the IntoIterator trait
// converts the struct into an iterator
impl IntoIterator for Person {
    // associated type of the iterator
    // change the associated type to a vec::IntoIter if we want to avoid creating a iter struct
    type IntoIter = PersonIter;
    fn into_iter(self) -> Self::IntoIter {
        // can avoid creating a struct by returning a vec![blah, blah, blah].into_iter()
        PersonIter {
            state: vec![
                format!("First name: {}", self.first),
                format!("Last name: {}", self.last),
                format!("Occupation: {}", self.occupation)],
        }
    }
}

fn main() {
    let person = Person {
        first: "John".to_owned(),
        last: "Doe".to_owned(),
        occupation: "Engineer".to_owned(),
    };
    // convert the struct into an iterator and iterate over it
    for item in person {
        println!("{item}");
    }
}
