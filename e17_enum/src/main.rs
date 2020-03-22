#[allow(dead_code)]
#[derive(Debug)]
enum Vegetables {
    Cabbage,
    Lettuce,
    Cucumber,
}

#[derive(Debug)]
// any kind of data can be put inside an enum
enum FruitCodes {
    Strawberry(u8),  // u8 data associated
    Pear(String),    // string data associated
    Orange(u16, i8), // u16 and i8 data associated
    Peach,           // no data associated
}

#[repr(u8)]
#[derive(Debug)]
enum Nuts {
    Nut = 1,
    Peanut = 2,
    Hazelnut = 3,
}

fn main() {
    println!("1. Integer enum");
    let cab = Vegetables::Cabbage;
    println!("\t{:?}", cab);

    println!("2. Enum with value");
    // enums can carry data beside
    let straw = FruitCodes::Strawberry(1);
    let pear = FruitCodes::Pear(String::from("2"));
    let orange = FruitCodes::Orange(3, -1);
    let orange2 = FruitCodes::Orange(1, -2);
    let peach = FruitCodes::Peach;
    println!("\t{:?}", straw);
    println!("\t{:?}", pear);
    println!("\t{:?}", orange);
    println!("\t{:?}", orange2);
    println!("\t{:?}", peach);

    println!("3. Converting an enum to its value");
    let nut = Nuts::Nut;
    let peanut = Nuts::Peanut;
    let hazelnut = Nuts::Hazelnut;
    println!("\t{:?}", nut as u8);
    println!("\t{:?}", peanut as u8);
    println!("\t{:?}", hazelnut as u8);
}
