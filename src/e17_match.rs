// match expression
fn test1() {
    let degree = 8;
    let result = match degree {
        1 | 2 => "winners",    // or
        3..=7 => "runner ups", // between
        _ => "losers",         // rest (placeholder)
    };

    assert_eq!(result, "losers");
}

pub fn test() {
    test1();
}
