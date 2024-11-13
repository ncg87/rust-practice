// Make the code compile by completing the add function.

use std::num::ParseIntError;

enum AddError {
    ParseError(ParseIntError),
    OverFlow,
}

// parse input strings to u8's and return their sum
// Result<u8, AddError> is a Result that returns a u8 and an AddError
// we mapped all errors to AddError so we can return a error in that enum
fn add(num1: &str, num2: &str) -> Result<u8, AddError> {
    let num1 = num1.parse::<u8>().map_err(|e| AddError::ParseError(e))?;
    let num2 = num2.parse::<u8>().map_err(|e| AddError::ParseError(e))?;
    // ok_or() transforms Option<T> -> Result<T,E> and takes value of type E as input
    let sum = num1.checked_add(num2).ok_or(AddError::OverFlow)?;
    Ok(sum)
}

fn main() {
    let (user_input1, user_input2) = ("23", "45");
    match add(user_input1, user_input2) {
        Ok(sum) => println!("Sum = {sum}"),
        Err(e) => match e {
            AddError::OverFlow => println!("Sum > {}", u8::MAX),
            AddError::ParseError(e) => println!("Invalid input, parse error: {e:?}"),
        },
    }
}
