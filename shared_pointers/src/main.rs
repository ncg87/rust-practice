use std::rc::Rc; // must import this to use Rc, unlike Box
// reference counting, allows multiple owners of the same data
// can only be used on one thread
// only allow immutable access to the data

use std::cell::RefCell; //refcell allows mutable access to the data
struct Database {
    max_connections: u32
}

struct AuthService {
    db: Rc<RefCell<Database>> // use this when you want to share access and mutable access to the data
}

struct ContentService {
    db: Rc<RefCell<Database>>
}

fn main() {
    // RC is updated to 1
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100
    }));
    let auth_service = AuthService { 
        // RC is updated to 2
        db: Rc::clone(&db) // must clone the Rc pointer to give another reference to the same data, incrementing the reference count
        // Rc clones the pointer, not the data itself, Box clones the data and the pointer (only ownership can we pass around without cloning the whole data)
    }; // both clones do the same thing, clone the pointer
    let content_service = ContentService { 
        // RC is updated to 3
        db: db.clone() // must clone the Rc pointer to give another reference to the same data
    };
    // borrow_mut() returns a mutable reference to the data
    db.borrow_mut().max_connections = 200; // able to break the rust borrow checker since it is all coded in a unsafe API
    // allows for shared access and mutable access to the data
    // can still only have one mutable reference at a time otherwise it will panic
}
