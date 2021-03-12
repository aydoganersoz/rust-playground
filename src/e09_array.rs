// array initialization without type and length
fn test1() {
    let a = ["joe", "jack"];

    assert_eq!(a, ["joe", "jack"]);
    assert_eq!(a[0], "joe");
    assert_eq!(a[1], "jack");
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

// array length
fn test9() {
    let a = [1, 2, 3, 4, 4, 4, 4, 4, 4];

    assert_eq!(a.len(), 9);
}

// array reverse
fn test10() {
    let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    let a_rev = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    a.reverse();

    assert_eq!(a, a_rev);
}

// array sort
fn test11() {
    let mut a = [9, 0, 5, 7, 1, 4, 3, 8, 2, 6];
    let a_sorted = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    a.sort();

    assert_eq!(a, a_sorted);
}

// array swap
fn test12() {
    let mut a = ['a', 'b', 'c', 'd'];
    let a_swap = ['c', 'b', 'a', 'd'];

    a.swap(0, 2);

    assert_eq!(a, a_swap);
}

// array search
fn test13() {
    let a = [1, 2, 3, 4, 5, 5, 1, 3, 0];

    assert_eq!(a.contains(&0), true);
    assert_eq!(a.contains(&9), false);
}

// array compare
fn test14() {
    let a = [1, 3];
    let b = [1, 2];
    let c = [1, 3];

    assert_eq!(a.eq(&b), false);
    assert_eq!(a.eq(&c), true);
}

// array first, last and nth element
fn test15() {
    let a = [2, 2, 3, 4, 5, 5, 1, 3, 0];

    assert_eq!(a.first().unwrap(), &2);
    assert_eq!(a.last().unwrap(), &0);
    assert_eq!(a.get(4).unwrap(), &5);
}

// array empty check
fn test16() {
    let a = [2, 2, 3, 4, 5, 5, 1, 3, 0];

    assert_eq!(a.is_empty(), false);
}

// array join
fn test17() {
    let a = ["11", "22", "33", "44"];

    assert_eq!(a.join("-"), "11-22-33-44");
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
    test12();
    test13();
    test14();
    test15();
    test16();
    test17();
}
