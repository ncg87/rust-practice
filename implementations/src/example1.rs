// Complete the method signatures by providing appropriate arguments.

struct Student {
    first_name: String,
    last_name: String,
    roll_no: u16,
}

impl Student {
    // immutable since we aren't modifying the struct
    fn get_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // mutable since we are modifying the struct
    fn set_roll_no(&mut self, new_roll_no: u16) {
        self.roll_no = new_roll_no;
    }

    // should take ownership since we are returning the struct formatted as a string
    fn convert_to_string(self) -> String {  // this is all up to the design
        format!(
            "Name: {} {}, Roll no: {}",
            self.first_name, self.last_name, self.roll_no
        )
    }
}

fn main() {
    let mut student = Student {
        first_name: "Harry".to_string(),
        last_name: "Potter".to_string(),
        roll_no: 42,
    };
    println!("Student is: {}", student.get_name());
    student.set_roll_no(50);
    let student_details = student.convert_to_string();
    println!("{student_details}");
}
