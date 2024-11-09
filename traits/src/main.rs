trait Park {
    // defining a interface to be implemented
    fn park(&self);
}

trait Paint { // a base trait that can be overridden, like inheritance
    fn paint(&self, color: String){
        println!("Painting the vehicle {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

struct Truck {
    info: VehicleInfo,
}

// must implement all the methods defined in the trait
impl Park for Car {
    fn park(&self) {
        println!("Parking the car");
    }
}

impl Paint for Car {}
impl Park for Truck {
    fn park(&self) {
        println!("Parking the truck");
    }
}
impl Paint for Truck {}

struct House{}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting the house {}", color);
    }
}

fn main() {
    println!("Hello, world!");
}
