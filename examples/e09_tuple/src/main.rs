fn main() {
    // tuples once declared, they cannot grow or shrink in size
    println!("1. Tuple initialization");
    let mut my_tup: (i32, f64, u8, &str) = (500, 6.4, 1, "hello");
    println!("\t{:?}", my_tup);

    // but their values can change
    println!("2. Tuple mutation");
    my_tup.0 = 501;
    my_tup.1 = 6.3;
    my_tup.2 = 2;
    my_tup.3 = "this is me";
    println!("\t{:?}", my_tup);

    println!("3. Tuple destruction");
    let my_tup: (i32, i32, i32) = (1, 2, 3);
    let (x, y, z) = my_tup;
    println!("\t{}:{}:{}", x, y, z);

    println!("4. Tuple of tuples");
    let my_tup_inner: (i32, bool) = (-1, true);
    let my_tup_parent: (f64, (i32, bool)) = (0.003, my_tup_inner);
    println!("\t{:?}", my_tup_parent);
}
