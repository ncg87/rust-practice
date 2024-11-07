enum Command {
    Undo,
    Redo,
    AddText(String), // tuple struct variant
    MoveCursor(i32, i32),
    Replace { from: String, to: String }, // struct variant
}

impl Command {
    fn serialize(&self) -> String {
        let json =match self {
            Command::Undo => String::from("{\"command\":\"undo\"}"),
            Command::Redo => String::from("\"command\":\"redo\""),
            // need to bind data in AddText to a variable
            Command::AddText(text) => {
                format!(
                    "{{ \
                        \"command\":\"add_text\", \
                        \"text\":\"{text}\" \
                    }}"
                )
            }
            Command::MoveCursor(x, y) => {
                format!(
                    "{{ \
                        \"command\":\"move_cursor\", \
                        \"line\":{x}, \
                        \"column\":{y} \
                    }}"
                )
            },
            // uses curly braces to bind data since
            Command::Replace { from, to } => {
                format!(
                    // / used to remove newlines
                    "{{ \
                        \"command\":\"replace\", \ 
                        \"from\":\"{from}\", \
                        \"to\":\"{to}\" \
                    }}"
                )
            },
        };
        json
    }
    
}

fn main() {
    let age = 18;
    // match is like switch statement, but must be exhaustive
    match age {
        1 => println!("You are 1 year old"),
        13..=19 => println!("You are a teenager"),
        21 => println!("You are 21 years old"),
        // binds age to x
        x => println!("You are {x} years old"), // variable is catch all, allows it to be exhaustive
    }

    let cmd = Command::AddText("Hello, world!".to_string());
    let cmd2 = Command::MoveCursor(1, 1);
    let cmd3 = Command::Replace { from: "Hello".to_string(), to: "Goodbye".to_string() };
    let cmd4 = Command::Undo;
    let cmd5 = Command::Redo;

    println!("{}", cmd.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
    println!("{}", cmd5.serialize());



}
