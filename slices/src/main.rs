fn main() {
    let tweet = String::from("Great day in the park!");
    // slicing the string, denoted by the address, similar to C, since they are contiguous
    let trimmed_start: &str = &tweet[..12]; // can omit the start or end index if contained
    let middle: & str = &tweet[12..17];
    let end: & str = &tweet[17..];
    // string slices are immutable seqences
    println!("{trimmed_start}...");
    println!("...{middle}...");
    println!("...{end}");
    
    // string literal is a slice
    let s = "Hello, world!";
    let s2 = String::from("Hello, world!");
    let trimmed = trim_tweet(s);
    println!("{trimmed}");
    // A slice can be taken from a String, but not vice versa
    // Technically any address to a string is a slice, but not vice versa
    // works because of dref coercion (???)
    let trimmed2 = trim_tweet(&s2);
    println!("{trimmed2}");


    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];
    println!("{:?}", slice); // needs to use the debug trait to print in array form
}

// we arent modifying the string, so we can take a slice
fn trim_tweet(tweet: &str) -> &str {
    &tweet[..5] // return a slice of the first 5 characters
}