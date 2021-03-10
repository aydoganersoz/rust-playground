// mutable variable
fn test1() {
    let mut x = 0;

    x += 1;

    assert_eq!(x, 1);
}

// immutable variable
fn test2() {
    let x = 100;

    // x += 1; // not possible (variable is immutable)

    assert_eq!(x, 100);
}

// mutating variable type
#[allow(unused_mut)]
fn test3() {
    let mut x = 0;

    // x = "a"; // not possible (type can't be mutated)

    assert_eq!(x, 0);
}

pub fn test() {
    test1();
    test2();
    test3();
}
