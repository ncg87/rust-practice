// Make this program compile without changing the variable type!

fn main() {

    let answer: String = "Blue".to_string() /* Your favorite color here */;
    // Need to use .to_string() to convert the string literal(&str) to a String type
    println!("My current favorite color is {}", answer);

    let ans:&str = "Blue"; // single quotes are for characters, double quotes are for strings slices
    println!("My current favorite color is {}", ans);
}