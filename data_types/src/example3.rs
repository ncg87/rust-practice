// Make this program compile by replacing the variable type.

fn main() {
    let number_of_stars: i64;
    // number below is too big for i32 (2,147,483,647), so we need to use i64
    number_of_stars = 400_000_000_000; // The Milky Way has more stars than can fit in a 32-bit integer type!
    // '_' is used to make the number more readable, it is ignored by the compiler in numbers
    println!("There are about {} stars in the Milky Way galaxy!", number_of_stars);
}