// Make the test compile and pass successfully.
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(1,1); // checks if the two values are equal
    }
}
