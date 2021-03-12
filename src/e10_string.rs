#[allow(unused_mut)]
#[allow(unused_variables)]

// declaring an empty string
fn test1() {
    let a = String::new();

    assert_eq!(a, "");

    println!("7. String vs str");
    let mut my_str_string = String::from("hello "); // mutable and can be treated
    let mut my_str_2_string = "hello ".to_string(); // mutable and can be treated
    let mut my_str_str = "hello "; // immutable and can't be treated
    println!("\tstring_string = {}", my_str_string);
    println!("\tstring_string_2 = {}", my_str_2_string);
    println!("\tstring_str = {}", my_str_str);
    my_str_string.push_str("rust");
    my_str_2_string.push_str("rust");
    // my_str_str.push_str("rust"); not allowed
    println!("\tstring_string = {}", my_str_string);
    println!("\tstring_string_2 = {}", my_str_2_string);
    println!("\tstring_str = {}", my_str_str);

    println!("8. Concatanating a character into a string");
    let mut my_str = String::from("1234567");
    // let mut my_str = "1234567".to_string(); // String (this would work)
    // let mut my_str = "1234567"; // str (this wouldn't work)
    println!("\t{}", my_str);
    my_str.push('8');
    println!("\t{}", my_str);

    println!("9. Concatanating a string into a string");
    let mut my_str = String::from("hello ");
    println!("\t{}", my_str);
    my_str.push_str("world!");
    println!("\t{}", my_str);

    println!("10. Concatanating a string into a string using +");
    let my_str_1 = String::from("joe");
    let my_str_2 = String::from("jane");
    let my_str_3 = String::from("patrick");
    let whole_string = my_str_1 + "-" + &my_str_2 + "-" + &my_str_3;
    println!("\t{}", whole_string);

    println!("11. String indexing");
    let my_str = String::from("hello");
    // let h = my_str[0]; // not allowed

    println!("12. Internal string representations");
    let my_str = String::from("Hi");
    println!("\t{}", my_str.len()); // 2 because each character is encoded in 1 byte
    let my_str = String::from("Здравствуйте");
    println!("\t{}", my_str.len()); // 24 because each character is encoded in 2 bytes
    println!("\t{}", my_str.chars().count()); // 12 because it's number of characters

    println!("13. String slicing");
    let my_str = "Здравствуйте";
    println!("\t{}", &my_str[0..4]); // because each character is encoded in 2 bytes
    let my_str = "hello";
    println!("\t{}", &my_str[0..4]); // because each character is encoded in 1 byte

    println!("14. Iterating over strings");
    let my_str = "Зд";
    for c in my_str.chars() {
        println!("\t{}", c); // 2 loops
    }
    for b in my_str.bytes() {
        println!("\t{}", b); // 4 loops because each character is encoded in 2 bytes
    }
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

fn test7() {}

fn test8() {}

fn test9() {}

fn test10() {}

fn test11() {}

fn test12() {}

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
}
