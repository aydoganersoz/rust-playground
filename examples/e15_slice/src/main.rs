fn main() {
    println!("1. String slice");
    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
        println!("\t{}", hello);
        println!("\t{}", world);
    }

    println!("2. i32 slice");
    {
        let n = [1, 2, 3, 4, 5];

        let middle = &n[1..4];
        println!("\t{:?}", middle);
    }
}
