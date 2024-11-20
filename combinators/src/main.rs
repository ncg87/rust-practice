// combinators are functions that take an iterator and return a new iterator
// they are lazy, meaning they only compute the value when needed/when next is called (.collect() is the one that executes the iterator)
// they are zero abstraction so they are very efficient

#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    let students = vec![
        "Bob 3.2",
        "Alice 3.5",
        "Jim 2.8",
        "Steve 3.9",
        "John 3.1",
        "Jane 3.7",
    ];
    // iterator is a combinator that returns an iterator
    let mut good_students2: Vec<Student> = students.iter()
        .map(|s| {
            let mut s = s.split(" ");
            let name = s.next()?.to_owned(); // ? propagates the none variant
            // parse returns the results type and .ok return an option type
            // ok converts an error into a none variant
            let gpa = s.next()?.parse::<f32>().ok()?; // ? propagates the none variant
            Some(Student { name, gpa })
        })
        .flatten() // create the a new iterator that extracts Some variant and ignores None variant
        // filter takes in a boolean expression and returns a new iterator with the elements that satisfy the expression as true
        .filter(|s| s.gpa > 3.5) // filter out the students with gpa less than 3.5 and returns a new iterator
        .collect(); // turn the iterator into a vector/collection, this is the combinator that executes the iterator since it calls next
    for s in good_students2 {
        println!("{:?}", s);
    }

    let mut good_students3: Vec<Student> = students.iter()
        .filter_map(|s| {
            let mut s = s.split(" ");
            let name = s.next()?.to_owned(); // ? propagates the none variant
            // parse returns the results type and .ok return an option type
            // ok converts an error into a none variant
            let gpa = s.next()?.parse::<f32>().ok()?; // ? propagates the none variant
            // filter_map return the value that is Some variant
            if gpa < 3.5 {return None}

            Some(Student { name, gpa })
        })
        .collect();
    for s in good_students3 {
        println!("{:?}", s);
    }

    let mut good_students = vec![];
    for s in students {
        let mut s = s.split(" ");
        let name = s.next();
        let gpa = s.next();
        // check if both name and gpa are Some variant, returned values
        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            // parse gpa string to f32
            let gpa: f32 = gpa.parse::<f32>().unwrap();
            // check if the parse was successful
            if gpa > 3.5 {
                good_students.push(Student { name, gpa });
            }
        }
    }
    for s in good_students {
        println!("{:?}", s);
    }
}
