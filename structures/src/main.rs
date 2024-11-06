struct Product {
    name: String,
    price: f64,
    in_stock: bool,
}

fn main() {
    // creating a instance of the structure, could also use let mut p = Product { ... }
    let mut p = Product {
        name: String::from("Apple"),
        price: 1.20,
        in_stock: true,
    };

    // changing the value of the in_stock field
    p.in_stock = false;

    // clones the price field from the struct
    let price = p.price;

    println!("Product {} costs {} and is in stock: {}", p.name, price, p.in_stock);
    println!("Tax is {}", calculate_tax(&p));
}
// taking a reference to the product since we aren't modifying it
fn calculate_tax(product: &Product) -> f64 {
    product.price * 0.1
}
