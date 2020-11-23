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

    println!("last");
    // consumes the iterator
    let arr = [1, 2, 3, 4, 5, 6];
    println!("\t{:?}", arr.iter().last().unwrap());

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

    println!("eq");
    let arr = ['a', 'b', 'z', 'w'];
    let arr2 = ['b', 'a', 'z', 'w'];
    let arr3 = [1, 2, 3];
    let arr4 = [1, 2, 3];
    println!("\t{:?}", arr.iter().eq(arr2.iter()));
    println!("\t{:?}", arr2.iter().eq(arr.iter()));
    println!("\t{:?}", arr3.iter().eq(arr4.iter()));

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

    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.ge
}
