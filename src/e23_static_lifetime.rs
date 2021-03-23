static MY_STATIC_I32: i32 = 42;
static MY_STATIC_STR: &'static str = "hey!";

fn test1() {
    assert_eq!(return_static_string(), "yay!");
}

fn test2() {
    assert_eq!(return_static_i32(), &42);
}

fn test3() {
    assert_eq!(return_static_string_2(), "hey!");
}

fn test4() {
    assert_eq!(argument_static_i32(&MY_STATIC_I32), 43);
}

fn return_static_string() -> &'static str {
    "yay!"
}

fn return_static_string_2() -> &'static str {
    MY_STATIC_STR
}

fn return_static_i32() -> &'static i32 {
    &MY_STATIC_I32
}

fn argument_static_i32(arg: &'static i32) -> i32 {
    arg + 1
}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
}
