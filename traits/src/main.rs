// super trait, anything implementing Vehicle must also implement Paint
trait Vehicle : Paint {
    // defining a interface to be implemented
    fn park(&self);
    fn get_default_color() -> String{
        "black".to_string()
    }
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
impl Vehicle for Car {
    fn park(&self) {
        println!("Parking the car");
    }
}

impl Paint for Car {}
impl Vehicle for Truck {
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

    let object = create_paintable_object(true); // returns a smart pointer to a heap allocated object
    
    // vector of trait objects, hold pointers to objects that implement the Paint trait
    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];
    
    object.paint("red".to_string());
    paint_blue(&*object); // dereferencing the smart pointer to get the object
    paint_blue(object.as_ref()); // also dereferencing the smart pointer to get the object
}

// generic function that can be used for any object that implements the Paint trait
// generic is known at compile time
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_string()); // any object passed in will have the paint method
}

// impl is know at compile time, dyn is not known at compile time it is determined at runtime still allows any object that implements the Paint trait
// creates slower runtimes since it can't be determined by compiler
// if performance is an issue, use impl since it is statically dispatched
fn paint_blue(object: &dyn Paint) { // point so object doesnt go out of scope
    // accepts trait objects now
    object.paint("blue".to_string());
}

// using where clause                   able to remove the Paint trait from the where clause since Vehicle is a super trait that includes Paint
fn paint_green<T>(object: &T) where T: Vehicle { // a generic where it implements a trait, good for readability and many traits
    object.paint("green".to_string());
    object.park();
}

// traits can we used as return types, box is returning a trait object
// dyn is a dynamic dispatch, it is a pointer to a heap allocated object, which can be any type that implements the Paint trait
fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {  // returns an paint object, gives flexibilty
        // Box is a smart pointer, pointing to a heap allocated object
        Box::new(Car{
            info: VehicleInfo{
                make: "Toyota".to_owned(),
                model: "Prius".to_owned(),
                year: 2008,
            },
        })
    } else {
        Box::new(House{})
    }
}
