#![allow(dead_code)]

use std::{collections::HashMap, io, num::ParseIntError, error::Error};
use std::fmt::Display;
 // debug is for developer facing error messages
struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String,
}
// used for developer facing error messages, for debugging
impl std::fmt::Debug for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self} \n\t{}", self.msg);

        // get reference of the error rather than taking ownership
        if let Some(e) = self.source.as_ref() {
            write!(f, "\n\nCaused by: \n\t{e:?}")?; // prints the cause
        }
        Ok(())
    }
}

// for user facing error messages
impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}

// From impl Converts a value of one type into another type

// works with error handling infrastructure
impl Error for ParsePaymentInfoError {
    // provided context about the underlying error, allows for chaining errors
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref() // has to deref because it's returning a reference to a box object
    } 
}


fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ") // splits the string into a list of strings
        .map(|s| { // maps each string to a u32 otherwise returns an error
            s.parse().map_err(|_| ParsePaymentInfoError {
                source: None,
                msg: format!("{s:?} could not be pased as a u32"),
            }) // parse to u32, it knows based on the collection method after the map
        }) // collect the results into a vector?
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymentInfoError {
            source: Some(Box::new(e)),
            msg: format!("Error parsing input as numbers. Input: {card}"),
        })?; // propagates the error from the parse, so we must return a result
        // ^^ Return a vector of u32 or returns an error, ? automatically converts the error to a ParsePaymentInfoError
    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}
                            // returns a ok or err, any type can be stored in these
fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    // map_err converts the error to a string, since the results return a string on error
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    // custom error message, returned as a string
    if len != expected_len {
        return Err(ParsePaymentInfoError {
            source: None,
            msg: format!(
                "Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"
            ),
        });
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap(); 

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}

enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn Error>, String),
}

impl std::fmt::Debug for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditCardError::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
            // self is the display string
            CreditCardError::Other(e, msg) => write!(f, "{self}\n{msg}\n\t{e:?}"),
        }
    }
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Credit card error: Could not retrieve card")
    }
}

impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CreditCardError::InvalidInput(_) => None,
            CreditCardError::Other(e, _) => {
                Some(e.as_ref()) // converts to a shared reference
            },
        }
    }
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    // ok_or returns an error string if the key is not found
    // transforms an option into a result
    // . get returns an option, so we use ok_or to return an error if the key is not found
    let card_string = credit_cards.get(name).ok_or(
        CreditCardError::InvalidInput(format!("No credit card was found for {name}."))
    )?;

    let card = parse_card(card_string)
        .map_err(|e| {
            CreditCardError::Other(Box::new(e), format!("Error parsing card for {name}."))
        })?;

    Ok(card)
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 12 16 123"),
        ("Tim", "1234567 0616 123"),
        ("Bob", "1234567 Dec 08 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match &err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_, _) => eprintln!("Something went wrong try again!"),
            }
            log::error!("Error: {:?}", err);
        },
    }
}