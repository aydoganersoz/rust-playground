fn main() {
    println!("Infinite loop");

    for i in 1.. {
        println!("\tvalue of i is {:?}", i);
        if i == 10 {
            break;
        }
    }
}
