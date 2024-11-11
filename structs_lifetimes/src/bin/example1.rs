// Something is missing from our struct definition. Can you fix it?

struct Book <'a> {
    author: &'a str, // both of these lifetimes are tied to the lifetime of the struct
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
