use std::collections::HashMap;

// use
fn test1() {
    let mut hm = HashMap::new();
    hm.insert("k1", -1);
    hm.insert("k2", 2);

    assert_eq!(hm.is_empty(), false);
}

pub fn test() {
    test1()
}
