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
    println!("1. Struct variable definition");
    let mut s = Student {
        id: 12_345_667,
        name: String::from("name"),
    };
    print_student_info(&s);

    println!("2. Struct method call");
    let surname = " surname";
    let _ = s.add_surname(&surname);
    print_student_info(&s);
}

fn print_student_info(s: &Student) {
    println!("\tstudent id: {}", s.id);
    println!("\tstudent name: {}", s.name);
}
