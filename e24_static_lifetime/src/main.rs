static MY_STATIC_I32: i32 = 42;
static MY_STATIC_STR: &'static str = "hey!";

fn main() {
    println!("1. Return static types");

    // these variables will live for the entire lifetime of the program
    let retval_1 = return_static_string();
    let retval_2 = return_static_i32();
    let retval_3 = return_static_string_2();
    let retval_4 = argument_static_i32(&MY_STATIC_I32);

    println!("\t{:?}", retval_1);
    println!("\t{:?}", retval_2);
    println!("\t{:?}", retval_3);
    println!("\t{:?}", retval_4);
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
