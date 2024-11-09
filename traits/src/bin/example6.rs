// Make the code compile by completing the function signature of `print_message`.

trait Message {
    fn message(&self) -> String {
        "How are you?".to_string()
    }
}

trait Printer {
    fn print(&self, printable: &impl Message) { // require that the var passed implements message
        println!("Message is: {}", printable.message()); // so that it can print the message
    }
}

struct M;
struct P;

impl Message for M {}
impl Printer for P {}

fn print_message<T, U>(msg: &T, printer: &U)
where T: Message, U: Printer // just combine the two traits to be used within two different variables
{
    printer.print(msg);
}

fn main() {
    let m = M;
    let p = P;
    print_message(&m, &p);
}
