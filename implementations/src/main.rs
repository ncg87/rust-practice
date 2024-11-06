struct Product {
    name: String,
    price: f64,
    in_stock: bool,
}

fn main() {
    // creating a instance of the structure, could also use let mut p = Product { ... }
    let mut p = Product::new("Apple".to_string(), 1.20);

    // changing the value of the in_stock field
    p.in_stock = false;

    // clones the price field from the struct
    let price = p.price;

    p.set_price(2.00);
    println!("Product {} used to cost {} and now costs {} and is in stock: {}", p.get_product_name(), price, p.price, p.in_stock);

    println!("Tax is now {}", p.calculate_tax());
    let order_number = p.buy(); // p is dropped from memory after this function
    println!("Order number is {}", order_number);
}

impl Product {
    // classic new associated function
    fn new(name: String, price: f64)->Product{
        Product{name, // dont need to specify the field name since the parameter name and field name are the same
            price, 
            in_stock: true}
    }

    // method since it uses the self keyword
    // immutable since we aren't modifying the struct
    fn calculate_tax(&self) -> f64 {
        // all products have a price field so we can access it directly
        self.price * Product::get_sales_tax() // calling the associated function using ::
    }
    fn get_product_name(&self) -> &str {
        &self.name
    }
    // mutable borrow to self
    fn set_price(&mut self, price: f64) {
        self.price = price;
    }
    // takes ownership of self and returns an i32, and the instance of the struct is dropped from memory
    fn buy(self)->i32{
        let name = self.name;
        println!("You bought {}", name);
        123
    }
    // associated function since it doesn't use the self keyword
    fn get_sales_tax()->f64{ // doesn't take self
        0.1
    }
}
