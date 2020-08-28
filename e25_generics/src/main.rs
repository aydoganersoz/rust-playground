use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
struct Student {
    id: u32,
    name: &'static str,
}

fn main() {
    println!("1. Generic function argument");

    let st = Student {
        id: 2,
        name: "antoine",
    };

    print_generic("I am a string");
    print_generic(1);
    print_generic(st);

    println!("2. Generic function argument and return");

    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest_number = return_largest(&number_list);
    let largest_char = return_largest(&char_list);
    println!("\t{:?}", largest_number);
    println!("\t{:?}", largest_char);
}

fn print_generic<T: Debug>(arg: T) {
    println!("\t{:?}", arg);
}

fn return_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
