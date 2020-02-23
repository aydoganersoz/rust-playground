fn main() {
    println!("Array initialization without type and length");
    let friends = ["birol", "alican", "burak", "cahit", "alpaslan"];
    println!("\t{:?}", friends);

    println!("Array initialization with type and length");
    let numbers: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("\t{:?}", numbers);

    println!("Array initialization with length and initial value");
    let zeros = [0; 4];
    println!("\t{:?}", zeros);

    println!("Accesing an array element");
    let fibonacci_elements = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    println!("\tfibonacci_elements[0] = {}", fibonacci_elements[0]);
    println!("\tfibonacci_elements[4] = {}", fibonacci_elements[4]);
    println!("\tfibonacci_elements[9] = {}", fibonacci_elements[9]);
    // println!("\tfibonacci_elements[45] = {}", fibonacci_elements[45]); // out of index

    println!("Accesing an array element using get(index)");
    let some_letters = ['s', 'c', 'f', 'b'];
    println!(
        "\tsome_letters.get(1).unwrap() = {:?}",
        some_letters.get(1).unwrap()
    );

    println!("2D array");
    let array_2d = [[1, 2], [2, 3], [3, 4], [4, 5]];
    println!("\t{:?}", array_2d);
    println!("\tarray_2d[0][0] = {}", array_2d[0][0]);
    println!("\tarray_2d[2][1] = {}", array_2d[2][1]);

    println!("Iteration of array elements");
    let a = [1, 2, 3];
    for element in a.iter() {
        print!("\t{}", element);
    }
    println!();

    println!("Array length");
    let measure_me = [1, 2, 3, 4, 4, 4, 4, 4, 4];
    println!("\tmeasure_me.len() = {}", measure_me.len());

    println!("Array reverse");
    let mut reverse_me = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    println!("\treverse_me = {:?}", reverse_me);
    reverse_me.reverse();
    println!("\treverse_me.reverse() = {:?}", reverse_me);

    println!("Array sort");
    let mut sort_me = [9, 0, 5, 7, 1, 4, 3, 8, 2, 6];
    println!("\tsort_me = {:?}", sort_me);
    sort_me.sort();
    println!("\tsort_me.sort() = {:?}", sort_me);

    println!("Array swap");
    let mut swap_me = [1, 2, 3, 4];
    println!("\tswap_me = {:?}", swap_me);
    swap_me.swap(1, 2);
    println!("\tswap_me.swap(1, 2) = {:?}", swap_me);

    println!("Array search");
    let search_me = [1, 2, 3, 4, 5, 5, 1, 3, 0];
    println!("\tsearch_me = {:?}", search_me);
    println!("\tsearch_me.contains(&0) = {:?}", search_me.contains(&0));

    println!("Array compare");
    let compare_me_1 = [1, 3];
    let compare_me_2 = [1, 2];
    let compare_me_3 = [1, 3];
    println!("\tcompare_me_1 = {:?}", compare_me_1);
    println!("\tcompare_me_2 = {:?}", compare_me_2);
    println!("\tcompare_me_3 = {:?}", compare_me_3);
    println!(
        "\tcompare_me_1.eq(&compare_me_2) = {:?}",
        compare_me_1.eq(&compare_me_2)
    );
    println!(
        "\tcompare_me_1.eq(&compare_me_3) = {:?}",
        compare_me_1.eq(&compare_me_3)
    );

    println!("Array first element");
    let first_me = [2, 2, 3, 4, 5, 5, 1, 3, 0];
    println!("\tfirst_me = {:?}", first_me);
    println!(
        "\tfirst_me.first().unwrap() = {:?}",
        first_me.first().unwrap()
    );

    println!("Array get element at nth index");
    let get_me = [2, 2, 3, 4, 5, 5, 1, 3, 0];
    println!("\tget_me = {:?}", get_me);
    println!("\tget_me.get(8).unwrap() = {:?}", get_me.get(8).unwrap());

    println!("Is array empty");
    let am_i_empty = [2, 2, 3, 4, 5, 5, 1, 3, 0];
    println!("\tam_i_empty = {:?}", am_i_empty);
    println!("\tam_i_empty.is_empty() = {:?}", am_i_empty.is_empty());

    println!("Array join");
    let join_me = ["one", "two", "three", "four"];
    println!("\tjoin_me = {:?}", join_me);
    println!("\tjoin_me.join(\"-\") = {:?}", join_me.join("-"));

    println!("Array last element");
    let last_me = [2, 2, 3, 4, 5, 5, 1, 3, 0, 8];
    println!("\tlast_me = {:?}", last_me);
    println!("\tlast_me.last().unwrap() = {:?}", last_me.last().unwrap());
}
