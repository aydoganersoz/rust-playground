fn main() {
    println!("1. Variable scope");
    {
        let hello_str = "hello";
        println!("\t{}", hello_str);
    }
    // println!("\t{}", hello_str); // not allowed as variable is out of scope

    println!("2. String assign");
    {
        let s1 = String::from("hey");
        // only pointer is copied not the actual data
        // both point to the same data but the owner is now s2
        let s2 = s1;
        // println!("\t{}", s1); // not allowed as s1 is not the owner any longer
        println!("\t{}", s2);
    }

    println!("3. String clone");
    {
        let s1 = String::from("ay");
        // actual data is also copied
        let s2 = s1.clone();
        println!("\t{}", s1);
        println!("\t{}", s2);
    }

    println!("4. String lateral");
    {
        let s1 = "oy";
        // actual data is also copied
        let s2 = s1;
        println!("\t{}", s1);
        println!("\t{}", s2);
    }
}
