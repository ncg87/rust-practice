// Enums can have variants that are different types, like structs or tuples
enum Command {
    Undo,
    Redo,
    AddText(String), // tuple struct variant
    MoveCursor(i32, i32),
    Replace { from: String, to: String }, // struct variant
}

// Can implement methods on enums
impl Command {
    // could serialize base of matching
    fn serialize(&self) -> String {
        String::from("JSON string")
    }
}

fn main() {
    let cmd = Command::Undo;
    let add_text = Command::AddText("Hello, world!".to_string());
    let move_cursor = Command::MoveCursor(1, 1); // tuple syntax
    let replace = Command::Replace { // struct syntax
        from: "Hello".to_string(),
        to: "Goodbye".to_string(),
    };

    let json = cmd.serialize();
}
