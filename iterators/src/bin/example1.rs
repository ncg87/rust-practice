// Provide the trait implementations and make the code execute successfully.

struct Employee {
    first_name: String,
    last_name: String,
    id: String,
}

struct EmployeeIter {
    state: Vec<String>,
}
// Implement the Iterator trait for the EmployeeIter struct
// tells the compiler that the struct can be iterated over
impl Iterator for EmployeeIter {
    type Item = String; // implemented the associated type
    fn next(&mut self) -> Option<Self::Item> {
        self.state.pop()
    }
}
// Convert the struct into an iterator
// tells the compiler that the struct can be converted into an iterator
impl IntoIterator for Employee {
    type Item = String; // implemented the associated type
    type IntoIter = EmployeeIter; // implemented the associated type of into iter
    fn into_iter(self) -> Self::IntoIter {
        EmployeeIter {
            state: vec![self.id, self.last_name, self.first_name],
        }
    }
}

fn main() {
    let employee = Employee {
        first_name: "Alice".to_owned(),
        last_name: "Smith".to_owned(),
        id: "ab123".to_owned(),
    };

    let mut emp_iter = employee.into_iter();
    println!("First name: {}", emp_iter.next().unwrap());
    println!("Last name: {}", emp_iter.next().unwrap());
    println!("ID: {}", emp_iter.next().unwrap());
    assert_eq!(emp_iter.next(), None);
}
