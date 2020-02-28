fn main() {
    println!("1. No argument no return");
    func01();

    println!("2. One integer argument no return");
    func02(3);

    println!("3. Two integer argument no return");
    func03(-1, 999);

    println!("4. Two integer argument integer return");
    let ret = func04(5, -3);
    println!("\tret: {}", ret);

    println!("5. Two integer argument integer return with `return`");
    let ret = func05(5, -3);
    println!("\tret: {}", ret);
}

fn func01() {
    println!("\tI don't take and receive anything.");
}

fn func02(x: u8) {
    println!("\targ1: {}", x);
}

fn func03(x: i8, y: u16) {
    println!("\targ1: {}", x);
    println!("\targ2: {}", y);
}

fn func04(x: i8, y: i8) -> i8 {
    println!("\targ1: {}", x);
    println!("\targ2: {}", y);

    x + y // look func05 for other possibility
}

#[allow(clippy::needless_return)]
fn func05(x: i8, y: i8) -> i8 {
    println!("\targ1: {}", x);
    println!("\targ2: {}", y);

    return x + y; // look func04 for other possibility
}
