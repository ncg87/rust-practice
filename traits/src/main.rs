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
    let car = Car{
        info: VehicleInfo {
            make: "Toyota".to_owned(),
            model: "Prius".to_owned(),
            year: 2008,
        },
    };
    let house = House{};
    paint_red(&car);
    paint_blue(&car);
    paint_green(&car);
    paint_red(&house);

    let object = create_paintable_object();
    object.paint("red".to_string());
    paint_blue(&object);
}

// generic function that can be used for any object that implements the Paint trait
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_string()); // any object passed in will have the paint method
}

// using impl syntax
fn paint_blue(object: &impl Paint) { // point so object doesnt go out of scope
    object.paint("blue".to_string());
}

// using where clause
fn paint_green<T>(object: &T) where T: Paint + Park { // a generic where it implements a trait, good for readability and many traits
    object.paint("green".to_string());
    object.park();
}

// traits can we used as return types
fn create_paintable_object() -> impl Paint {
    House{}
}
