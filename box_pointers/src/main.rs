trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container> // container is a recursive type, so we need to use a box to store it on the heap
    // this allows the compiler to know the size of the Container, simpily a pointer
}

impl UIComponent for Container {}

fn main() {
    let button_a = Button { text: "button a".to_owned() }; // stored on the stack
    let button_b = Box::new(Button { text: "button b".to_owned() }); // stored on the heap since we wrapped it in a Box

    let button_c = button_a; // entire button is copied to transfer ownership
    let button_d = button_b; // button d the pointer is copied to transfer ownership, not the whole button
    // this is more efficient than copying the whole button
    // comp is vector of pointers to UIComponent trait objects
    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d
    ];

}