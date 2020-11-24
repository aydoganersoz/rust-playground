fn main() {
    println!("Infinite loop");
    for i in 1.. {
        println!("\ti = {:?}", i);
        if i == 10 {
            break;
        }
    }

    /* not available in stable release (only nightly for now)
    println!("advance_by");
    let arr = [1, 2, 3, 4, 5, 6];
    let mut arr_iter = arr.iter();
    arr_iter.advance_by(2);
    println!("arr_iter.next() = {:?}", arr_iter.next().unwrap()); */

    println!("all");
    let arr = [1, 2, 3, 4, 5, 6];
    println!("\t{:?}", arr.iter().all(|&x| x >= 1 && x <= 6));

    println!("any");
    let arr = [1, 2, 3, 4, 5, 6];
    println!("\t{:?}", arr.iter().any(|&x| x == 3));

    println!("chain");
    let it1 = [1, 2, 3];
    let it2 = [4, 5, 6];
    let it1it2 = it1.iter().chain(it2.iter());
    for i in it1it2 {
        println!("\ti = {:?}", i);
    }

    println!("cmp");
    let str1 = "abc";
    let str2 = "aba";
    let ordering = str1.cmp(str2);
    println!("\ti = {:?}", ordering);

    println!("collect");
    let arr = [1, 2, 3, 4, 5];
    let times_200: Vec<i32> = arr.iter().map(|&x| x * 200).collect();
    println!("\ti = {:?}", times_200);

    println!("count");
    // consumes the iterator
    let arr = [1, 2, 3, 4, 5];
    println!("\t{:?}", arr.iter().count());

    println!("cycle");
    let arr = [1, 2, 3, 4, 5];
    let mut cnt = 0;
    for i in arr.iter().cycle() {
        cnt += 1;
        println!("\t(cnt, i) = ({:?}, {:?})", cnt, i);
        if cnt > 12 {
            break;
        }
    }

    println!("enumerate");
    let arr = ['a', 'b', 'z', 'w'];
    for (idx, val) in arr.iter().enumerate() {
        println!("\t({:?}, {:?})", idx, val);
    }

    println!("filter");
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 13];
    let filtered_arr: Vec<&i32> = arr.iter().filter(|&&x| x >= 12).collect();
    println!("\t{:?}", filtered_arr);

    println!("find");
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 13];
    let found = arr.iter().find(|&&x| x == 7);
    println!("\t{:?}", found);

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

    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.ge
}
