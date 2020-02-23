fn main() {
    // tuples once declared, they cannot grow or shrink in size
    println!("tuple initialization");
    let mut tup: (i32, f64, u8, &str) = (500, 6.4, 1, "hello");
    println!("\ttuple (i32, f64, u8, &str) = {:?}", tup);

    // but their values can change
    println!("tuple mutation");
    println!("\ttuple (i32, f64, u8, &str) = {:?}", tup);
    tup.0 = 501;
    tup.1 = 6.3;
    tup.2 = 2;
    tup.3 = "aydus";
    println!("\ttuple (i32, f64, u8, &str) = {:?}", tup);

    // tuple destruction
    println!("tuple destruction");
    let tup: (i32, i32, i32) = (1, 2, 3);
    let (x, y, z) = tup;
    println!("\tx={} y={} z={}", x, y, z);

    // tuple of tuples
    println!("tuple of tuples");
    let tup_inner: (i32, bool) = (-1, true);
    let tup_parent: (f64, (i32, bool)) = (0.003, tup_inner);
    println!("\t(f64, (i32, bool)) = {:?}", tup_parent);
}
