// Complete the code by addressing the TODO.

#[derive(Debug)] // this line makes the enum variants printable!
enum Message {
    Move { x: i32, y: i32 }, // tuple struct variant
    Echo(String), // tuple variant
    ChangeColor(u8, u8, u8), // tuple variant
    Quit, // unit variant
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
