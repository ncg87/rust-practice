use std::rc::Rc; // must import this to use Rc, unlike Box
// reference counting, allows multiple owners of the same data
// can only be used on one thread
struct Database {}

struct AuthService {
    db: Rc<Database>
}

struct ContentService {
    db: Rc<Database>
}

fn main() {
    // RC is updated to 1
    let db = Rc::new(Database {});
    let auth_service = AuthService { 
        // RC is updated to 2
        db: db.clone() // must clone the Rc pointer to give another reference to the same data, incrementing the reference count
        // Rc clones the pointer, not the data itself, Box clones the data and the pointer (only ownership can we pass around without cloning the whole data)
    };
    let content_service = ContentService { 
        // RC is updated to 3
        db: db.clone() // must clone the Rc pointer to give another reference to the same data
    };
}