// Fix the code so that it compiles.

fn main() {
    let names = vec!["Alice", "Bob", "Cindy"];
    let index = 2;
    // using option and pattern matching with get
    if let Some(name) = names.get(index) {
        println!("{name} is present at index {index}");
    } else {
        println!("invalid index {index}");
    }
}
