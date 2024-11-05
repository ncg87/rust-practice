// Destructure the `cat` tuple so that the println will work.

fn main() {

    let cat = ("Furry McFurson", 3.5);
    // destructure the tuple, cannot directly specify types in tuple
    // let (name: &str, age: f64) = cat; would be invalid, have to specify name and age types before destructuring
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
