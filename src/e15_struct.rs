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

// struct variable definition
fn test1() {
    let s = Student {
        id: 12_345_667,
        name: String::from("name"),
    };

    assert_eq!(s.id, 12_345_667);
    assert_eq!(s.name, "name");
}

// struct method call
fn test2() {
    let mut s = Student {
        id: 12_345_667,
        name: String::from("name"),
    };

    let surname = " surname";
    let _ = s.add_surname(&surname);

    assert_eq!(s.id, 12_345_667);
    assert_eq!(s.name, "name surname");
}

pub fn test() {
    test1();
    test2();
}
