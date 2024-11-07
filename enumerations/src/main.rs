struct Product {
    name: String,
    category: ProductCategory,
    price: f64,
    in_stock: bool,
}

// Enumeration of varience
enum ProductCategory {
    Books,
    Clothing,
    Electronics,
}

fn main() {
    // initializes with double colon
    let category = ProductCategory::Books;
    let product = Product {
        name: "The Pragmatic Programmer".to_string(), // string literal to string
        category,
        price: 45.00,
        in_stock: true,
    };
}
