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

fn test2() {
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(1999), false);
}

fn is_leap_year(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        _ => false,
    }
}

pub fn test() {
    test1();
    test2();
}
