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

// flatten
fn test13() {
    let arr = vec![vec![1, 2, 3, 4], vec![4, 5, 6]];

    assert_eq!(
        arr.iter().flatten().collect::<Vec<&i32>>(),
        [&1, &2, &3, &4, &4, &5, &6]
    );
}

// fold (`reduce` in Python)
fn test14() {
    let arr = [2, 4, 6];

    assert_eq!(arr.iter().fold(200, |acc, x| x + acc), 212); // 200+2+4+6
}

// eq - ge - gt - le - lt - ne
fn test15() {
    let arr1 = ['a', 'b', 'c'];
    let arr2 = ['a', 'b', 'c'];
    let arr3 = ['a', 'b'];

    assert_eq!(arr1.iter().eq(arr2.iter()), true);
    assert_eq!(arr1.iter().eq(arr3.iter()), false);
    assert_eq!(arr1.iter().ge(arr2.iter()), true);
    assert_eq!(arr1.iter().ge(arr3.iter()), true);
    assert_eq!(arr1.iter().gt(arr2.iter()), false);
    assert_eq!(arr1.iter().gt(arr3.iter()), true);
    assert_eq!(arr1.iter().le(arr2.iter()), true);
    assert_eq!(arr1.iter().le(arr3.iter()), false);
    assert_eq!(arr1.iter().lt(arr2.iter()), false);
    assert_eq!(arr1.iter().lt(arr3.iter()), false);
    assert_eq!(arr1.iter().ne(arr2.iter()), false);
    assert_eq!(arr1.iter().ne(arr3.iter()), true);
}

// last
fn test16() {
    // consumes the iterator
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(arr.iter().last(), Some(&6));
}

// nth
fn test17() {
    // consumes the iterator up to the nth element
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(arr.iter().nth(3), Some(&4));
}

// map
fn test18() {
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(
        arr.iter().map(|x| x * 2).collect::<Vec<i32>>(),
        [2, 4, 6, 8, 10, 12]
    );
}

// max - min
fn test19() {
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(arr.iter().min(), Some(&1));
    assert_eq!(arr.iter().max(), Some(&6));
}

// position - rposition
fn test20() {
    let arr = ["p", "h", "r", "t"];

    assert_eq!(arr.iter().position(|&x| x == "r"), Some(2));
    assert_eq!(arr.iter().rposition(|&x| x == "r"), Some(2));
}

// product
fn test21() {
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(arr.iter().product::<i32>(), 720);
}

// rev
fn test22() {
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(
        arr.iter().rev().collect::<Vec<&i32>>(),
        [&6, &5, &4, &3, &2, &1]
    );
}

// skip
fn test23() {
    let arr = [1, 2, 3, 4, 5, 6];

    assert_eq!(arr.iter().skip(4).collect::<Vec<&i32>>(), [&5, &6]);
}

// step_by
fn test24() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    assert_eq!(
        arr.iter().step_by(2).collect::<Vec<&i32>>(),
        [&1, &3, &5, &7, &9, &11, &13]
    );
}

// sum
fn test25() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    assert_eq!(arr.iter().sum::<i32>(), 91);
}

// take
fn test26() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    assert_eq!(arr.iter().take(4).collect::<Vec<&i32>>(), [&1, &2, &3, &4]);
}

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
