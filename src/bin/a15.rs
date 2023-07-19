// Topic:Option
//
// Program requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an optional data

struct Student {
    name: String,
    locker: Option<i16>,
}

impl Student {
    fn new(name: String, locker: Option<i16>) -> Self {
        Self { name, locker }
    }

    fn student_info(self) {
        println!("name: {}", self.name);

        match self.locker {
            Some(locker) => println!("locker: {}", locker),
            None => println!("no locker assigned")
        }
    }
}

fn main() {
    let student_with_no_locker = Student::new(
        "Crystal".to_owned(),
        None,
    );

    let student_with_locker = Student::new(
        "Doe".to_owned(),
        Some(1803)
    );

    Student::student_info(student_with_locker);
    Student::student_info(student_with_no_locker);
}
