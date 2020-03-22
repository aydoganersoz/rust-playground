fn main() {
    println!("1. Array initialization without type and length");
    let my_arr = ["joe", "jack", "tom", "ivan", "christian"];
    println!("\t{:?}", my_arr);

    println!("2. Array initialization with type and length");
    let my_arr: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("\t{:?}", my_arr);

    println!("3. Array initialization with length and initial value");
    let my_arr = [0; 4];
    println!("\t{:?}", my_arr);

    println!("4. Accesing an array element");
    let my_arr = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    println!("\t{}", my_arr[0]);
    println!("\t{}", my_arr[4]);
    // println!("\t{}", my_arr[45]); // out of index

    println!("5. Accesing an array element using get(index)");
    let my_arr = ['s', 'c', 'f', 'b'];
    println!("\t{:?}", my_arr.get(1).unwrap());

    println!("6. Accesing an element of a 2D array");
    let my_arr = [[1, 2], [2, 3], [3, 4], [4, 5]];
    println!("\t{:?}", my_arr);
    println!("\t{}", my_arr[0][0]);
    println!("\t{}", my_arr[2][1]);

    println!("7. Iteration through an array");
    let my_arr = [1, 2, 3];
    for e in my_arr.iter() {
        print!("\t{}", e);
    }
    println!();

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
