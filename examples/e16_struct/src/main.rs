#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
}

impl Student {
    fn add_surname(&mut self, surname: &str) -> usize {
        self.name.push_str(surname);
        self.name.len()
    }
}

fn main() {
    let mut student1 = Student {
        id: 12_345_667,
        name: String::from("jane"),
    };

    print_student_info(&student1);

    let jane_surname = " brown";
    let ret = student1.add_surname(&jane_surname);
    println!("\t{:?}", ret);
    print_student_info(&student1);
}

fn print_student_info(student: &Student) {
    println!("\tstudent id: {}", student.id);
    println!("\tstudent name: {}", student.name);
}
