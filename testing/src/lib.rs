/// A savings account (Doc comments are used for documentation)
pub struct SavingsAccount {
    balance: i32,
}
/// Creates a 'SavingsAccount' with a balance of 0
/// 
/// # Examples // this is a code block that shows how to use the function and is testesd to work
/// ```
/// use testing::SavingsAccount;
/// let account = SavingsAccount::new();
/// assert_eq!(account.get_balance(), 0);
/// ```
impl SavingsAccount {
    /// Creates a `SavingsAccount` with a balance of 0
    /// 
    pub fn new() -> SavingsAccount {
        SavingsAccount {
            balance: 0,
        }
    }

    pub fn get_balance(&self) -> i32 { self.balance }

    pub fn deposit(&mut self, amount: i32) { 
        if amount < 0 {
            // usually want to return a Result type, but for testing purposes, panic! is used
            panic!("Can not deposit a negative amount!");
        }
        self.balance += amount 
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }
}

// unit tests can test private functions since it is a child module
#[cfg(test)]
mod tests {
    use super::*; // use all the functions in the module, put them into the scope

    #[test]
    fn should_have_a_starting_balance_of_0() {
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0); // assert equal
        assert_ne!(account.get_balance(), 1); // assert not equal
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        // can add custom message to all asserts
        assert_eq!(account.get_balance(), 100, "Balance should be 100!"); // assert equal with custom message
        assert!(account.get_balance() != 1); // regular assert checks if true
    }

    #[test]
    #[should_panic] // only works if the function panics, does work if returns result
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingsAccount::new();
        account.deposit(-1);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> { // return a Result type, tests to see if the function returns Ok
        let mut account = SavingsAccount::new();
        account.deposit(100);
        account.transfer(123456, 100)?;
        Ok(())
    }
}