// string slice
fn test1() {
    let s = String::from("hello world");

    assert_eq!(&s[0..5], "hello");
    assert_eq!(&s[6..11], "world");
}

// i32 slice
fn test2() {
    let n = [1, 2, 3, 4, 5];

    assert_eq!(&n[1..4], [2, 3, 4]);
}

pub fn test() {
    test1();
    test2();
}
