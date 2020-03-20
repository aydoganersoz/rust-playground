use std::collections::HashMap;

fn main() {
    println!("1. Use");
    let mut hm = HashMap::new();
    hm.insert("k1", -1);
    hm.insert("k2", 2);
    println!("\t{:?}", hm)
}
