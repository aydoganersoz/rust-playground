fn main() {
    println!("Shadowing same type");

    // We’re effectively creating a new variable when we use the let keyword again
    let x = 5;
    println!("\tThe value of x is: {}", x);

    // We’re effectively creating a new variable when we use the let keyword again
    let x = x + 1;
    println!("\tThe value of x is: {}", x);

    // We’re effectively creating a new variable when we use the let keyword again
    let x = x * 2;
    println!("\tThe value of x is: {}", x);

    println!("Shadowing different types");

    // We’re effectively creating a new variable when we use the let keyword again
    let y = "aaa";
    println!("\tThe value of y is: {} (str)", y);

    // We’re effectively creating a new variable when we use the let keyword again
    let y = y.len();
    println!("\tThe value of y is: {} (i32)", y);
}
