// shadowing same type
fn test1() {
    let x = 5;

    assert_eq!(x, 5);

    let x = 4;

    assert_eq!(x, 4);
}

// shadowing different types
fn test2() {
    let y = "aaa";

    assert_eq!(y, "aaa");

    let y = y.len();

    assert_eq!(y, 3);
}

pub fn test() {
    test1();
    test2();
}
