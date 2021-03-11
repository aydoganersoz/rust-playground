// array initialization without type and length
fn test1() {
    let a = ["joe", "jack"];

    assert_eq!(a, ["joe", "jack"]);
    assert_eq!(a[0], "joe");
    assert_eq!(a[1], "jack");

    println!("8. Array length");
    let my_arr = [1, 2, 3, 4, 4, 4, 4, 4, 4];
    println!("\t{}", my_arr.len());

    println!("9. Array reverse");
    let mut my_arr = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    println!("\t{:?}", my_arr);
    my_arr.reverse();
    println!("\t{:?}", my_arr);

    println!("10. Array sort");
    let mut my_arr = [9, 0, 5, 7, 1, 4, 3, 8, 2, 6];
    println!("\t{:?}", my_arr);
    my_arr.sort();
    println!("\t{:?}", my_arr);

    println!("11. Array swap");
    let mut my_arr = [1, 2, 3, 4];
    println!("\t{:?}", my_arr);
    my_arr.swap(1, 2);
    println!("\t{:?}", my_arr);

    println!("12. Array search");
    let my_arr = [1, 2, 3, 4, 5, 5, 1, 3, 0];
    println!("\t{:?}", my_arr.contains(&0));

    println!("13. Array compare");
    let my_arr_1 = [1, 3];
    let my_arr_2 = [1, 2];
    let my_arr_3 = [1, 3];
    println!("\t{:?}", my_arr_1.eq(&my_arr_2));
    println!("\t{:?}", my_arr_1.eq(&my_arr_3));

    println!("14. Array first element");
    let my_arr = [2, 2, 3, 4, 5, 5, 1, 3, 0];
    println!("\t{:?}", my_arr.first().unwrap());

    println!("15. Array last element");
    let my_arr = [2, 2, 3, 4, 5, 5, 1, 3, 0, 8];
    println!("\t{:?}", my_arr.last().unwrap());

    println!("16. Array get element at nth index");
    let my_arr = [2, 2, 3, 4, 5, 5, 1, 3, 0];
    println!("\t{:?}", my_arr.get(8).unwrap());

    println!("17. Is array empty");
    let my_arr = [2, 2, 3, 4, 5, 5, 1, 3, 0];
    println!("\t{:?}", my_arr.is_empty());

    println!("18. Array join");
    let my_arr = ["one", "two", "three", "four"];
    println!("\t{:?}", my_arr.join("-"));
}

// array initialization with type and length
fn test2() {
    let a: [i32; 4] = [0, 1, 2, 3];

    assert_eq!(a, [0, 1, 2, 3]);
    assert_eq!(a[0], 0);
    assert_eq!(a[3], 3);
}

// array initialization with length and initial value
fn test3() {
    let a = [0; 4];

    assert_eq!(a, [0, 0, 0, 0]);
    assert_eq!(a.len(), 4);
}

// accesing an array element
fn test4() {
    let a = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

    assert_eq!(a[4], 5);
    // assert_eq!(a[45], 5); // not possible (out of index)
}

// accesing an array element using get(index)
fn test5() {
    let a = ['s', 'c', 'f', 'b'];

    assert_eq!(a[3], 'b');
    assert_eq!(a.get(3).unwrap(), &a[3]);
}

// accesing an element of a 2D array
fn test6() {
    let a = [[1, 2], [2, 3], [3, 4], [4, 5]];

    assert_eq!(a[0][1], 2);
    assert_eq!(a[2][0], 3);
    assert_eq!(a.len(), 4);
    assert_eq!(a[1].len(), 2);
}

// iteration through an array
fn test7() {
    let a = [1, 2, 3];
    let mut idx = 0;

    for e in a.iter() {
        assert_eq!(e, &a[idx]);
        idx += 1;
    }
}

// iteration through an array using enumeration
fn test8() {
    let a = [1, 2, 3];

    for (idx, e) in a.iter().enumerate() {
        assert_eq!(e, &a[idx]);
    }
}

fn test9() {}

fn test10() {}

fn test11() {}

fn test12() {}

fn test13() {}

fn test14() {}

fn test15() {}

fn test16() {}

fn test17() {}

fn test18() {}

fn test19() {}

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
    test12();
    test13();
    test14();
    test15();
    test16();
    test17();
    test18();
    test19();
}
