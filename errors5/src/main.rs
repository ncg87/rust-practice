use std::{io, fs, error};
fn main() {
    let i = parse_file("example.txt");
    match i {
        Ok(num) => println!("{}", num),
        Err(e) => {
            match e {
                ParseFileError::File => {
                    println!("File not found");
                },
                ParseFileError::Parse(e) => {
                    println!("Error parsing file: {:?}", e);
                },
            }
        }
    }
}

enum ParseFileError {
    File,
    Parse(std::num::ParseIntError),
}

// Box<dyn Error> is a trait object that can hold any error that implements the Error trait, can return this but doesn't allow for calling function to do anything with the error
// implement this when the error type doesn't matter, or when you want to return the error to the caller




fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    // Functions can now determine the type of error that will be returned, since we specified a mapping for each error type
    let s = fs::read_to_string(filename)
                        .map_err(|e| ParseFileError::File)?; // propagates the error from reading file
    let num: i32 = s.parse()
                        .map_err(|e| ParseFileError::Parse(e))?; // propagates the error from parsing string to i32
    Ok(num)
}
