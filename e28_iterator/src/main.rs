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
    let less_than_200: Vec<i32> = arr.iter().map(|&x| x * 200).collect();
    println!("\ti = {:?}", less_than_200);

    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count
}
