#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum Vegetables {
    Cabbage,
    Lettuce,
    Cucumber,
}

// any kind of data can be put inside an enum
#[derive(PartialEq, Debug)]
enum Fruits {
    Strawberry(u8),  // u8 data associated
    Pear(String),    // string data associated
    Orange(u16, i8), // u16 and i8 data associated
    Peach,           // no data associated
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
enum Nuts {
    Nut = 1,
    Peanut = 2,
    Hazelnut = 3,
}

// integer enum
fn test1() {
    let cab = Vegetables::Cabbage;

    assert_eq!(cab, Vegetables::Cabbage);
}

// enum with value
fn test2() {
    // enums can carry data beside
    let straw = Fruits::Strawberry(1);
    let pear = Fruits::Pear(String::from("2"));
    let orange = Fruits::Orange(3, -1);
    let orange2 = Fruits::Orange(1, -2);
    let peach = Fruits::Peach;

    assert_eq!(straw, Fruits::Strawberry(1));
    assert_eq!(pear, Fruits::Pear(String::from("2")));
    assert_eq!(orange, Fruits::Orange(3, -1));
    assert_eq!(orange2, Fruits::Orange(1, -2));
    assert_eq!(peach, Fruits::Peach);
    assert_ne!(orange, orange2);
}

// converting an enum to its value
fn test3() {
    let nut = Nuts::Nut;
    let peanut = Nuts::Peanut;
    let hazelnut = Nuts::Hazelnut;

    assert_eq!(nut as u8, 1);
    assert_eq!(peanut as u8, 2);
    assert_eq!(hazelnut as u8, 3);
}

pub fn test() {
    test1();
    test2();
    test3();
}
