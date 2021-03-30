fn test1() {
    assert_eq!(format!("hello {}", "world"), "hello world");
}

pub fn test() {
    test1();
}
