use std::fs;
use std::io::{self, Read};

fn main() {
    let first_line = read_first_line("file.txt");
}
// Option and results are both enums that represent optional values so we get them a lot
fn read_first_line(filename: &str) -> Result<String, std::io::Error> {
    // closure get the results of the previous operation and returns a new result
    fs::read_to_string(filename).and_then(|s| {
        // return an iterator over the lines of the string, gets the first line, and returns an error if the file is empty
        // map return a new iterator, ok_or returns an error if the iterator is empty
        // maps just transforms the value give, since we give it an option, it returns an option
        // if we give it an iterator, it returns an iterator
        // just .ok() converts the option to a result and .and_then() converts the result to an option
        s.lines().next().map(|s| s.to_owned()).ok_or(std::io::Error::new(std::io::ErrorKind::Other, "File empty"))
        // option and results both contain a lot of combinators, which are methods that return a new option or result and they chain together nicely
    })
}
