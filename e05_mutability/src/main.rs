fn main() {
    let mut mutable_var = 0;
    let immutable_var = 100;

    println!("1. Mutable variable");
    println!("\t{}", mutable_var);
    mutable_var += 1;
    println!("\t{}", mutable_var);

    println!("2. Immutable variable (mutation not allowed)");
    println!("\t{}", immutable_var);
    // immutable_var += 1; // immutable variable can't be mutated
    println!("\t{}", immutable_var);

    println!("3. Mutating variable type (mutation not allowed)");
    println!("\t{}", mutable_var);
    // mutable_var = "1";  // type can't be mutated
    println!("\t{}", mutable_var);
}
