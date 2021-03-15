use std::collections::HashMap;

// hash map declaration
fn test1() {
    let mut my_hm = HashMap::new();

    println!("3. Hash maps and ownership");
    let new_key = String::from("team_3");
    let new_value = 20;
    my_hm.insert(new_key, new_value); // element 3 declaration
    println!("\t{:?}", my_hm);
    // println!("\t{}", new_key); // we are no longer the owner of this variable
    println!("\t{}", new_value); // we are still the owner of this variable

    println!("4. Accessing an hash map element");
    let key = String::from("team_1");
    let value = my_hm.get(&key);
    println!("\t{:?}", value.unwrap());

    println!("5. Iterate over an hash map");
    for (key, value) in &my_hm {
        println!("\t<{:?}, {:?}>", key, value);
    }

    println!("6. Updating an hash map element");
    println!("\t{:?}", my_hm);
    my_hm.insert(String::from("team_1"), 11); // element 1 update
    println!("\t{:?}", my_hm);

    println!("7. Updating an hash map element if no value exists");
    println!("\t{:?}", my_hm);
    my_hm.entry(String::from("team_4")).or_insert(50); // element 4 declaration
    my_hm.entry(String::from("team_1")).or_insert(12); // no effect as value already exists
    println!("\t{:?}", my_hm);
}

// key-value insertion
fn test2() {
    let mut h = HashMap::new();

    h.insert(String::from("team_1"), 10); // element 1 declaration (key, value)
    h.insert(String::from("team_2"), 50); // element 2 declaration (key, value)

    assert_eq!(h.get("team_1"), Some(&10));
    assert_eq!(h.get("team_2"), Some(&50));
}

fn test3() {}

fn test4() {}

fn test5() {}

fn test6() {}

fn test7() {}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
}
