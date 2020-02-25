fn main() {
    println!("1. Shadowing same type");

    // We’re effectively creating a new variable when we use the let keyword again
    let x = 5;
    println!("\t{}", x);

    // We’re effectively creating a new variable when we use the let keyword again
    let x = x + 1;
    println!("\t{}", x);

    println!("2. Shadowing different types");

    // We’re effectively creating a new variable when we use the let keyword again
    let y = "aaa";
    println!("\t{}", y);

    // We’re effectively creating a new variable when we use the let keyword again
    let y = y.len();
    println!("\t{}", y);
}
