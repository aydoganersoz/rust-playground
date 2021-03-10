// if-else
fn test1() {
    let temperature = 23;
    let weather;

    if temperature < 21 {
        weather = "cold weather"
    } else if temperature >= 21 && temperature < 31 {
        weather = "mild weather"
    } else {
        weather = "hot weather"
    }

    assert_eq!(weather, "mild weather");
}

// if-let
fn test2() {
    let temperature = 23;

    let weather = if temperature >= 28 {
        "hot weather"
    } else {
        "cold weather"
    };

    assert_eq!(weather, "cold weather");
}

pub fn test() {
    test1();
    test2();
}
