// declaring an empty string
fn test1() {
    let a = String::new();

    assert_eq!(a, "");
}

// declaring a string (String) with initial value
fn test2() {
    let a = String::from("abcd");

    assert_eq!(a, "abcd");
}

// declaring a string (String) with initial value
fn test3() {
    let a = "1234".to_string();

    assert_eq!(a, "1234");
}

// declaring a string (str) with initial value
fn test4() {
    let a = "zxcv";

    assert_eq!(a, "zxcv");
}

// string decomposition into bytes
fn test5() {
    let a = String::from("hello");

    assert_eq!(a.as_bytes(), [b'h', b'e', b'l', b'l', b'o']);
    assert_eq!(a.as_bytes(), [104, 101, 108, 108, 111]);
}

// capacity of a string
fn test6() {
    let a = String::from("1234567");
    let b = String::with_capacity(10);

    assert_eq!(a.capacity(), 7);
    assert_eq!(a.len(), 7);

    assert!(b.capacity() >= 10);
    assert!(b.capacity() < 11);
}

// string vs str
#[allow(unused_mut)]
#[allow(unused_variables)]
fn test7() {
    let mut a = String::from("hello "); // mutable and can be treated
    let mut b = "hello ".to_string(); // mutable and can be treated
    let mut c = "hello "; // immutable and can't be treated

    a.push_str("rust");
    b.push_str("rust");
    // c.push_str("rust"); // not allowed

    assert_eq!(a, "hello rust");
    assert_eq!(b, "hello rust");
}

// concatanating a character into a string
fn test8() {
    let mut s = String::from("1234567");

    s.push('8');

    assert_eq!(s, "12345678");
}

// concatanating a string into a string
fn test9() {
    let mut a = String::from("hello ");

    a.push_str("world!");

    assert_eq!(a, "hello world!");
}

// concatanating a string into a string using +
fn test10() {
    let a = String::from("joe");
    let b = String::from("jane");
    let c = String::from("patrick");
    let s = a + "-" + &b + "-" + &c;

    assert_eq!(s, "joe-jane-patrick");
}

// string indexing
#[allow(unused_variables)]
fn test11() {
    let s = String::from("hello");
    // let h = s[0]; // not allowed
}

// internal string representations
fn test12() {
    let a = String::from("Hi");
    let b = String::from("Здравствуйте");

    assert_eq!(a.len(), 2); // 2 because each character is encoded in 1 byte
    assert_eq!(b.len(), 24); // 24 because each character is encoded in 2 bytes
    assert_eq!(b.chars().count(), 12); // 12 because it's number of characters
}

// string slicing
fn test13() {
    let a = "Здравствуйте";
    let b = "hello";

    assert_eq!(&a[0..4], "Зд");
    assert_eq!(&b[0..4], "hell");
}

// iterating over strings
fn test14() {
    let s = "Зд";
    let mut cnt1 = 0;
    let mut cnt2 = 0;

    for _ in s.chars() {
        cnt1 += 1;
    }

    assert_eq!(cnt1, 2); // 2 loops because there are two characters in the string

    for _ in s.bytes() {
        cnt2 += 1;
    }

    assert_eq!(cnt2, 4); // 4 loops because each character is encoded in 2 bytes
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
    test9();
    test10();
    test11();
    test12();
    test13();
    test14();
}
