// no argument no return
fn test1() {
    func01();
}

// one integer argument no return
fn test2() {
    func02(3);
}

// two integer argument no return
fn test3() {
    func03(-1, 999);
}

// two integer argument integer return
fn test4() {
    assert_eq!(func04(5, -3), 2);
}

// two integer argument integer return with `return`
fn test5() {
    assert_eq!(func05(5, -3), 2);
}

// string reference argument
fn test6() {
    let s = String::from("hello world");

    assert_eq!(func06(&s), s.len());
}

// string argument
fn test7() {
    let s = String::from("hello world");
    let s_len = s.len();

    assert_eq!(func07(s), s_len);
}

// mutable string reference argument
fn test8() {
    let mut s = String::from("hello world");

    func08(&mut s);

    assert_eq!(s, "hello world from here");
}

fn func01() {
    assert!(true);
}

fn func02(x: u8) {
    assert_eq!(x, 3);
}

fn func03(x: i8, y: u16) {
    assert_eq!(x, -1);
    assert_eq!(y, 999);
}

fn func04(x: i8, y: i8) -> i8 {
    assert_eq!(x, 5);
    assert_eq!(y, -3);

    x + y
}

#[allow(clippy::needless_return)]
fn func05(x: i8, y: i8) -> i8 {
    assert_eq!(x, 5);
    assert_eq!(y, -3);

    return x + y;
}

fn func06(s: &str) -> usize {
    s.len()
}

fn func07(s: String) -> usize {
    s.len()
}

fn func08(s: &mut String) {
    s.push_str(" from here");
}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
}
