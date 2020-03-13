fn main() {
    println!("1. Match expression");
    let degree = 8;
    let result = match degree {
        1 | 2 => "winners",    // or
        3..=7 => "runner ups", // between
        _ => "losers",         // rest (placeholder)
    };
    println!("\t{}", result);
}
