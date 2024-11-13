// Use `ok` combinator to convert Result to Option.
// Do not add any statements anywhere.

fn add(num1: &str, num2: &str) -> Option<u8> {
    // TODO: only modify the 2 statements below
    // checks for none type and returns none if so
    let num1 = num1.parse::<u8>().ok()?; // Parse the first number and handle any errors
    let num2 = num2.parse::<u8>().ok()?; // Parse the second number and handle any errors
    num1.checked_add(num2) // Add the numbers and handle any overflows
}

fn main() {
    let (num1, num2) = ("4", "5");
    if let Some(sum) = add("4", "5") {
        println!("{num1} + {num2} = {sum}");
    }
}
