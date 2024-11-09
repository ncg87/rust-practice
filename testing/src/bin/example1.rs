// Make the test compile and pass successfully.
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true); // checks if the condition is true
    }
}
