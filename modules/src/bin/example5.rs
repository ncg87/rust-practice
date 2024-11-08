// Make the code compile by addressing the TODO.

mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit; // public use statement to re export the module
    pub use self::veggies::CUCUMBER as veggie; // public use statement to make it available to outside modules

    mod fruits { // 'static just implies that reference will be valid throughout program execution
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
