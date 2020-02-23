fn main() {
    let mut mutable_var = 0;
    let immutable_var = 100;

    println!("Mutable variable");
    println!("\tbefore = {0}", mutable_var);
    mutable_var += 1;
    println!("\tafter = {0}", mutable_var);

    println!("Immutable variable");
    println!("\tbefore = {0}", immutable_var);
    // immutable_var += 1; // this operation is not allowed
    println!("\tafter = {0}", immutable_var);

    println!("Mutating variable type");
    println!("\tbefore = {0} (i32)", mutable_var);
    // mutable_var = "1";  // type can't be mutated
    println!("\tafter = {0} (i32)", mutable_var);
}
