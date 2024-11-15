#![allow(dead_code)]

use std::{collections::HashMap, io, num::ParseIntError};

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {
    let numbers = card
        .split(" ") // splits the string into a list of strings
        .map(|s| { // maps each string to a u32 otherwise returns an error
            s.parse() // parse to u32, it knows based on the collection method after the map
        }) // collect the results into a vector?
        .collect::<Result<Vec<u32>, _>>()?; // propagates the error from the parse, so we must return a result
        // ^^ Return a vector of u32 or returns an error
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
fn parse_card(card: &str) -> Result<Card, String> {
    // map_err converts the error to a string, since the results return a string on error
    let mut numbers = parse_card_numbers(card).map_err(|e| e.to_string())?;

    let len = numbers.len();
    let expected_len = 4;

    // custom error message, returned as a string
    if len != expected_len {
        return Err(format!(
            "Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"
        ));
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

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other(String, String),
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
            CreditCardError::Other(e.to_string(), format!("Error parsing card for {name}."))
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