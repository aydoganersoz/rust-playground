// no argument no return"
fn test1() {
    func01();

    println!("4. Two integer argument integer return");
    let ret = func04(5, -3);
    println!("\tret: {}", ret);

    println!("5. Two integer argument integer return with `return`");
    let ret = func05(5, -3);
    println!("\tret: {}", ret);

    println!("6. String reference argument");
    let s = String::from("hello world");
    println!("\t{}", s);
    let ret = func06(&s);
    println!("\t{}", ret);

    println!("7. String argument");
    let s = String::from("hello world");
    println!("\t{}", s);
    let ret = func07(s);
    println!("\t{}", ret);

    println!("8. Mutable string reference argument");
    let mut s = String::from("hello world");
    println!("\t{}", s);
    func08(&mut s);
    println!("\t{}", s);
}

// one integer argument no return
fn test2() {
    func02(3);
}

// two integer argument no return
fn test3() {
    func03(-1, 999);
}

fn test4() {}

fn test5() {}

fn test6() {}

fn test7() {}

fn test8() {}

fn func08(s: &mut String) {
    s.push_str(" from here");
}

fn func07(s: String) -> usize {
    s.len()
}

fn func06(s: &str) -> usize {
    s.len()
}

#[allow(clippy::needless_return)]
fn func05(x: i8, y: i8) -> i8 {
    println!("\targ1: {}", x);
    println!("\targ2: {}", y);

    return x + y; // look func04 for other possibility
}

fn func04(x: i8, y: i8) -> i8 {
    println!("\targ1: {}", x);
    println!("\targ2: {}", y);

    x + y // look func05 for other possibility
}

fn func03(x: i8, y: u16) {
    assert_eq!(x, -1);
    assert_eq!(y, 999);
}

fn func02(x: u8) {
    assert_eq!(x, 3);
}

fn func01() {
    assert!(true);
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
