use testing::SavingsAccount;
mod utils; // add this as a submodule

// integration tests are stored in the tests folder
// unit tests are stored in the src folder with the code
#[test]
fn should_have_a_starting_balance_of_0() {
    utils::common_setup();
    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0);
}