#[allow(dead_code)]
#[derive(Debug)]
enum Vegetables {
    Cabbage,
    Lettuce,
    Cucumber,
}

#[allow(dead_code)]
#[derive(Debug)]
enum FruitCodes {
    Strawberry(u8),
    Pear(String),
    Orange(u16),
}

fn main() {
    println!("1. Integer enum");
    let cab = Vegetables::Cabbage;
    println!("\t{:?}", cab);

    println!("2. Enum with value");
    let straw = FruitCodes::Strawberry(1);
    let pear = FruitCodes::Pear(String::from("2"));
    let orange = FruitCodes::Orange(3);
    println!("\t{:?}", straw);
    println!("\t{:?}", pear);
    println!("\t{:?}", orange);
}
