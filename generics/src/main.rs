struct BrowserCommand<PayloadType> /*have to declare the generic type parameter before the struct*/ {
    name: String,   
    payload: PayloadType, // T is a generic type parameter
}
// add generic after impl and struct so rust knows the impl is also generic
impl<PayloadType> BrowserCommand<PayloadType> {
    fn new(name: String, payload: PayloadType) -> Self {
        BrowserCommand { name,
                        payload
        }
    }
    fn get_payload(&self) -> &PayloadType {
        &self.payload
    }
}
// implement for when T is a String
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand::new("navigate".to_owned(), "https://www.google.com".to_owned());
    cmd1.print_payload();
    let cmd2 = BrowserCommand::new("zoom".to_owned(), 200);
    println!("{}", cmd2.get_payload());
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();
    let serialized_p1 = serialize_payload(p1);
    let serialized_p2 = serialize_payload(p2);
    println!("{}", serialized_p1);
    println!("{}", serialized_p2);


}
                    //declare generic and then use it in the parameter type 
fn serialize_payload<T>(payload: T) -> String { // allows me to pass in any type
    // convert to json
   "placeholder".to_owned()
} 
// rust creates a new type of function for each type of T at compile time and then calls it 