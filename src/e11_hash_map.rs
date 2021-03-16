use std::collections::HashMap;

// hash map declaration
fn test1() {
    let mut h = HashMap::new();
    let k = 1;
    let v = "val";

    h.insert(k, v);
}

// key-value insertion
fn test2() {
    let mut h = HashMap::new();

    h.insert(String::from("k1"), 10); // element 1 declaration (key, value)
    h.insert(String::from("k2"), 50); // element 2 declaration (key, value)

    assert_eq!(h.get("k1"), Some(&10));
    assert_eq!(h.get("k2"), Some(&50));
}

// hash map and ownership
fn test3() {
    let mut h = HashMap::new();
    let k = String::from("k3");
    let v = 20;

    h.insert(k, v);

    // assert_eq!(k, "k3"); // not possible (we are no longer the owner of k)
}

// accessing an hash map element
fn test4() {
    let mut h = HashMap::new();

    h.insert(String::from("k1"), -1);

    assert_eq!(h.get(&String::from("k1")), Some(&-1));
}

// iterate over an hash map
fn test5() {
    let mut h = HashMap::new();

    h.insert(String::from("k1"), 10);
    h.insert(String::from("k2"), 50);

    for (key, value) in &h {
        if key == "k1" {
            assert_eq!(value, &10);
        } else {
            assert_eq!(value, &50);
        }
    }
}

// updating an hash map element
fn test6() {
    let mut h = HashMap::new();

    h.insert(1, 10);
    assert_eq!(h.get(&1), Some(&10));

    h.insert(1, 11); // update
    assert_eq!(h.get(&1), Some(&11));
}

// updating an hash map element if no value exists
fn test7() {
    let mut h = HashMap::new();

    h.insert(1, 10);

    h.entry(2).or_insert(50); // insert if not exists
    h.entry(1).or_insert(-4); // no effect as the key exists already

    assert_eq!(h.get(&1), Some(&10));
    assert_eq!(h.get(&2), Some(&50));
}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
}
