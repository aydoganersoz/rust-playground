use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
struct Student {
    id: u32,
    name: &'static str,
}

// generic function argument
fn test1() {
    let st = Student {
        id: 2,
        name: "antoine",
    };

    assert_eq!(format_generic("I am a string"), "\"I am a string\"");
    assert_eq!(format_generic(1), "1");
    assert_eq!(format_generic(st), "Student { id: 2, name: \"antoine\" }");
}

// generic function argument and return
fn test2() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    assert_eq!(return_largest(&number_list), 100);
    assert_eq!(return_largest(&char_list), 'y');
}

fn format_generic<T: Debug>(arg: T) -> String {
    format!("{:?}", arg)
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

pub fn test() {
    test1();
    test2();
}
