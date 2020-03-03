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

    println!("5. String borrowed by function");
    {
        let s1 = String::from("hello world");
        println!("\t{}", s1);
        // function doesn't take the ownership of s1 (borrowing)
        let _ = func_ref(&s1);
        println!("\t{}", s1); // s1 is still in scope
    }

    println!("6. String owned by function");
    {
        let s1 = String::from("hello world");
        println!("\t{}", s1);
        // function takes the ownership of s1 (owning)
        let _ = func_str_own(s1);
        // println!("\t{}", s1); // s1 is no longer in scope
    }

    println!("7. Mutable string borrowed by function");
    {
        let mut s1 = String::from("hello world");
        println!("\t{}", s1);
        // function doesn't take the ownership of s1 (borrowing)
        func_mut_ref(&mut s1);
        println!("\t{}", s1); // s1 is still in scope
    }
}

fn func_mut_ref(s: &mut String) {
    s.push_str(" from here");
}

fn func_str_own(s: String) -> usize {
    s.len()
}

fn func_ref(s: &str) -> usize {
    s.len()
}
