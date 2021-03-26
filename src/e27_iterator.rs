// infinite loop
fn test1() {
    let mut f = false;

    for i in 1.. {
        if i == 10 {
            f = true;
            break;
        }
    }

    assert_eq!(f, true);

    /*-----*/

    println!("flatten");
    let nested_vec = vec![vec![1, 2, 3, 4], vec![4, 5, 6]];
    let flattened: Vec<&i32> = nested_vec.iter().flatten().collect();
    println!("\t{:?}", flattened);

    println!("fold");
    // `reduce` in Python
    let numbers = [2, 4, 6, 9, 12];
    let folded = numbers.iter().fold(200, |acc, x| x + acc);
    println!("\t{:?}", folded);

    println!("eq");
    let arr1 = ['a', 'b', 'c'];
    let arr2 = ['a', 'b', 'c'];
    let arr3 = ['a', 'b'];
    println!("\t{:?}", arr1.iter().eq(arr2.iter()));
    println!("\t{:?}", arr1.iter().eq(arr3.iter()));

    println!("ge");
    let arr1 = ['a', 'b', 'c'];
    let arr2 = ['a', 'b', 'c'];
    let arr3 = ['a', 'b'];
    println!("\t{:?}", arr1.iter().ge(arr2.iter()));
    println!("\t{:?}", arr1.iter().ge(arr3.iter()));

    println!("gt");
    let arr1 = ['a', 'b', 'c'];
    let arr2 = ['a', 'b', 'c'];
    let arr3 = ['a', 'b'];
    println!("\t{:?}", arr1.iter().gt(arr2.iter()));
    println!("\t{:?}", arr1.iter().gt(arr3.iter()));

    println!("le");
    let arr1 = ['a', 'b', 'c'];
    let arr2 = ['a', 'b', 'c'];
    let arr3 = ['a', 'b'];
    println!("\t{:?}", arr1.iter().le(arr2.iter()));
    println!("\t{:?}", arr1.iter().le(arr3.iter()));

    println!("lt");
    let arr1 = ['a', 'b', 'c'];
    let arr2 = ['a', 'b', 'c'];
    let arr3 = ['a', 'b'];
    println!("\t{:?}", arr1.iter().lt(arr2.iter()));
    println!("\t{:?}", arr1.iter().lt(arr3.iter()));

    println!("ne");
    let arr1 = ['a', 'b', 'c'];
    let arr2 = ['a', 'b', 'c'];
    let arr3 = ['a', 'b'];
    println!("\t{:?}", arr1.iter().ne(arr2.iter()));
    println!("\t{:?}", arr1.iter().ne(arr3.iter()));

    println!("last");
    // consumes the iterator
    let arr = [1, 2, 3, 4, 5, 6];
    println!("\t{:?}", arr.iter().last().unwrap());

    println!("nth");
    // consumes the iterator up to the nth element
    let arr = [1, 2, 3, 4, 5, 6];
    println!("\t{:?}", arr.iter().nth(3).unwrap());

    println!("map");
    let arr = [1, 2, 3, 4, 5, 6];
    let mapped: Vec<i32> = arr.iter().map(|x| x * 2).collect();
    println!("\t{:?}", mapped);

    println!("max");
    let arr = [1, 2, 3, 4, 5, 6];
    println!("\t{:?}", arr.iter().max().unwrap());

    println!("min");
    let arr = [1, 2, 3, 4, 5, 6];
    println!("\t{:?}", arr.iter().min().unwrap());

    println!("position");
    let arr = ["p", "h", "r", "t"];
    println!("\t{:?}", arr.iter().position(|&x| x == "r"));

    println!("rposition");
    let arr = ["p", "h", "r", "t"];
    println!("\t{:?}", arr.iter().rposition(|&x| x == "r"));

    println!("product");
    let arr = [1, 2, 3, 4, 5, 6];
    println!("\t{:?}", arr.iter().product::<i32>());

    println!("rev");
    let arr = [1, 2, 3, 4, 5, 6];
    let arrit = arr.iter().rev();
    for i in arrit {
        println!("\ti = {:?}", i);
    }

    println!("skip");
    let arr = [1, 2, 3, 4, 5, 6];
    let arrit = arr.iter().skip(2);
    for i in arrit {
        println!("\ti = {:?}", i);
    }

    println!("step_by");
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let arrit = arr.iter().step_by(2);
    for i in arrit {
        println!("\ti = {:?}", i);
    }

    println!("sum");
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    println!("\t{:?}", arr.iter().sum::<i32>());

    println!("take");
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let arrit = arr.iter().take(10);
    for i in arrit {
        println!("\ti = {:?}", i);
    }

    println!("zip");
    let it1 = [1, 2, 3];
    let it2 = [4, 5, 6];
    let it1it2 = it1.iter().zip(it2.iter());
    for (i, j) in it1it2 {
        println!("\t(i, j) = ({:?}, {:?})", i, j);
    }
    let it1it2 = it1.iter().zip(it2.iter());
    for tup in it1it2 {
        println!("\ttup = {:?}", tup);
    }

    println!("unzip");
    let it1 = [1, 2, 3];
    let it2 = [4, 5, 6];
    let it1it2 = it1.iter().zip(it2.iter());
    let (left, right): (Vec<i32>, Vec<i32>) = it1it2.unzip();
    for i in left {
        println!("\tit1(i) = {:?}", i);
    }
    for i in right {
        println!("\tit2(i) = {:?}", i);
    }

    /*println!("zip - new");
    let factors = &[2, 3, 5, 7, 11];
    let limit = 5;
    let zipped = (0..factors.len()).enumerate(1..limit);
    for tup in zipped {
        println!("\ttup = {:?}", tup);
    }*/
}

// advance_by (not available in release yet)
fn test2() {
    /*let arr = [1, 2, 3, 4, 5, 6];
    let mut arr_iter = arr.iter();
    arr_iter.advance_by(2);*/
}

// all
fn test3() {
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(arr.iter().all(|&x| x % 2 == 0), false);
    assert_eq!(arr.iter().all(|&x| x < 10), true);
}

// any
fn test4() {
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(arr.iter().any(|&x| x == 3), true);
    assert_eq!(arr.iter().any(|&x| x > 6), false);
}

// chain
fn test5() {
    let arr1 = [1, 2];
    let arr2 = [4];
    let mut arr3 = arr1.iter().chain(arr2.iter());

    assert_eq!(arr3.next(), Some(&arr1[0]));
    assert_eq!(arr3.next(), Some(&arr1[1]));
    assert_eq!(arr3.next(), Some(&arr2[0]));
}

// cmp
fn test6() {
    let str1 = "abc";
    let str2 = "aba";

    assert_eq!(str1.cmp(str2), std::cmp::Ordering::Greater);
}

// collect
fn test7() {
    let arr = [1, 2];
    let times_200: Vec<i32> = arr.iter().map(|&x| x * 200).collect();

    assert_eq!(times_200[0], 200);
    assert_eq!(times_200[1], 400);
}

// count
fn test8() {
    // consumes the iterator
    let arr = [1, 2, 3, 4, 5];

    assert_eq!(arr.iter().count(), 5);
}

// cycle
fn test9() {
    let arr = [1, 2, 3, 4, 5];
    let mut cnt = 0;

    // repeats an iterator endlessly
    for _ in arr.iter().cycle() {
        cnt += 1;
        if cnt > 12 {
            break;
        }
    }

    assert_eq!(cnt, 13);
}

// enumerate
fn test10() {
    let arr = ['a', 'b', 'z'];
    let mut e = arr.iter().enumerate();

    assert_eq!(e.next(), Some((0, &'a')));
    assert_eq!(e.next(), Some((1, &'b')));
    assert_eq!(e.next(), Some((2, &'z')));
}

// filter
fn test11() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 13];

    assert_eq!(
        arr.iter().filter(|&&x| x >= 12).collect::<Vec<&i32>>(),
        [&12, &12, &13]
    );
}

// find
fn test12() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 13];

    assert_eq!(arr.iter().find(|&&x| x == 7), Some(&7));
    assert_eq!(arr.iter().find(|&&x| x == 17), None);
}

fn test13() {}

fn test14() {}

fn test15() {}

fn test16() {}

fn test17() {}

fn test18() {}

fn test19() {}

fn test20() {}

fn test21() {}

fn test22() {}

fn test23() {}

fn test24() {}

fn test25() {}

fn test26() {}

fn test27() {}

fn test28() {}

fn test29() {}

fn test30() {}

fn test31() {}

fn test32() {}

fn test33() {}

fn test34() {}

fn test35() {}

fn test36() {}

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
    test20();
    test21();
    test22();
    test23();
    test24();
    test25();
    test26();
    test27();
    test28();
    test29();
    test30();
    test31();
    test32();
    test33();
    test34();
    test35();
    test36();
}
