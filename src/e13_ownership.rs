// variable scope
fn test1() {
    {
        let s = "hello";

        assert_eq!(s, "hello");
    }
    // assert_eq!(s, "hello"); // not allowed as variable is out of scope
}

// string type
fn test2() {
    let s1 = String::from("hey");
    // only pointer is copied not the actual data
    // both point to the same data but the owner is now s2
    let s2 = s1;

    // assert_eq!(s1, "hey"); // not allowed as value is borrowed
    assert_eq!(s2, "hey");
}

// string lateral
fn test3() {
    let s1 = "oy";
    // actual data is also copied
    let s2 = s1;

    assert_eq!(s1, "oy");
    assert_eq!(s2, "oy");
}

// string clone
fn test4() {
    let s1 = String::from("ay");
    // actual data is also copied
    let s2 = s1.clone();

    assert_eq!(s1, "ay");
    assert_eq!(s2, "ay");
}

// string borrowed by function
fn test5() {
    let s = String::from("ey");

    assert_eq!(s, "ey");

    // function doesn't take the ownership of s (just borrowing)
    let _ = func_ref(&s);

    assert_eq!(s, "ey"); // s is not owned by the above function
}

// string owned by function
fn test6() {
    let s = String::from("ey");

    assert_eq!(s, "ey");

    // function takes the ownership of s (owning)
    let _ = func_str_own(s);

    // assert_eq!(s, "ey"); // not possible as s is owned by the above function
}

// mutable string borrowed by function
fn test7() {
    let mut s = String::from("hello world");

    assert_eq!(s, "hello world");

    // function doesn't take the ownership of s (borrowing)
    func_mut_ref(&mut s);

    assert_eq!(s, "hello world from here");
}

// mutable immutable references
fn test8() {
    // having immutable references while having a mutable one
    // is allowed
    let mut s = String::from("hello rust");

    {
        let r1 = &mut s; // r1 takes the ownership of s
        let r2 = &r1;
        let r3 = &r1;
        // let r4 = &s; // not allowed as s is owned by something else

        assert_eq!(r1, "hello rust");
        assert_eq!(r2, &mut "hello rust");
        assert_eq!(r3, &mut "hello rust");
    }

    assert_eq!(s, "hello rust");
}

// multiple mutable and immutable reference
fn test9() {
    // having a mutable reference while having an immutable one
    // is not allowed
    let mut s = String::from("bye");

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; // not allowed

    assert_eq!(r1, "bye");
    assert_eq!(r2, "bye");

    let r3 = &mut s; // allowed as r1 and r2 are no longer in scope

    assert_eq!(r3, "bye");
}

// dangling reference
fn test10() {
    // let s = func_dangling_ref(); // not allowed
}

fn test11() {
    let list = vec!["hello", "world"];

    assert_eq!(join_str(&list), "helloworld");
}

fn join_str(list: &[&str]) -> String {
    let mut new_str = String::new();

    new_str.push_str(list[0]);
    new_str.push_str(list[1]);

    new_str
}

// fn func_dangling_ref() -> &String {
//     let s = String::from("hello world");
//     &s
// }
// lifetime of s is over

fn func_ref(s: &str) -> usize {
    s.len()
}

fn func_str_own(s: String) -> usize {
    s.len()
}

fn func_mut_ref(s: &mut String) {
    s.push_str(" from here");
}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
    test9();
    test10();
    test11();
}
