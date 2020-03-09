#[derive(Debug)]
struct Student {
    id: u32,
}

fn main() {
    let student1 = Student { id: 12_345_667 };

    println!("\t{:?}", student1);
}
