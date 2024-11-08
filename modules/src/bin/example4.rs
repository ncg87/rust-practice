// Complete the code by use declarations above main.

mod student {
    pub mod operations {
        use super::Student; // using super to refer to parent module

        pub fn assign_grade(student: &mut Student) {
            if student.marks >= 80 {
                student.grade = 'A';
            } else if student.marks >= 60 {
                student.grade = 'B';
            } else {
                student.grade = 'C';
            }
        }

    }

pub struct Student {
    pub name: String, // struct fields can also be made public
    pub marks: u8,
    pub grade: char,
}

impl Student {
    // make methods/associated functions public in order to access from outside the module
    pub fn new(name: &str, marks: u8) -> Self {
        Self {
            name: name.to_string(),
            marks,
            grade: 'X',
        }
    }
}

}

fn main() {
    use student::operations::assign_grade; // bring Student and assign_grade into scope
    let mut student = student::Student::new("Alice", 75);
    assign_grade(&mut student);
    println!("{} got {} grade", student.name, student.grade);
}
