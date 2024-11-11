use rand::Rng;

fn main() {
    // Example 1
    let player1 = String::from("player 1");
    let player2 = String::from("player 2");

    let result = first_turn(player1.as_str(), player2.as_str()); // reference returned is based on lifetimes of arguements, which are defined above

    // How does the borrow checker know result is not a dangling reference?
    println!("Player going first is: {}", result);

    // Example 2
    let player1 = String::from("player 1");
    {
        let player2 = String::from("player 2");
        let result = first_turn(player1.as_str(), player2.as_str());
        println!("Player going first is: {}", result);
    }

    // Example 3
    let player1 = String::from("player 1");
    let player2 = String::from("player 2");
    let result;
    {
        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

// generic lifetime, lifetime of return value is same as the lifetime of the arguments
// passes in the shortest life time of the arguments specified by 'a
// lifetime is always related to an input arguments
// static lifetimes are defined by 'static, last as long as the program runs
fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}