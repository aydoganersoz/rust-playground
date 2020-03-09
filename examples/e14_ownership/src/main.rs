#[allow(unused_mut)]
fn main() {
    println!("1. Variable scope");
    {
        let hello_str = "hello";
        println!("\t{}", hello_str);
    }
    // println!("\t{}", hello_str); // not allowed as variable is out of scope

    println!("2. String type");
    {
        let s1 = String::from("hey");
        // only pointer is copied not the actual data
        // both point to the same data but the owner is now s2
        let s2 = s1;
        // println!("\t{}", s1); // not allowed as s1 is not the owner any longer
        println!("\t{}", s2);
    }

    println!("3. String lateral");
    {
        let s1 = "oy";
        // actual data is also copied
        let s2 = s1;
        println!("\t{}", s1);
        println!("\t{}", s2);
    }

    println!("4. String clone");
    {
        let s1 = String::from("ay");
        // actual data is also copied
        let s2 = s1.clone();
        println!("\t{}", s1);
        println!("\t{}", s2);
    }

    println!("5. String borrowed by function");
    {
        let s = String::from("hello world");
        println!("\t{}", s);
        // function doesn't take the ownership of s (borrowing)
        let _ = func_ref(&s);
        println!("\t{}", s); // s1 is still the owner
    }

    println!("6. String owned by function");
    {
        let s = String::from("hello world");
        println!("\t{}", s);
        // function takes the ownership of s (owning)
        let _ = func_str_own(s);
        // println!("\t{}", s1); // s is no longer the owner
    }

    println!("7. Mutable string borrowed by function");
    {
        let mut s = String::from("hello world");
        println!("\t{}", s);
        // function doesn't take the ownership of s (borrowing)
        func_mut_ref(&mut s);
        println!("\t{}", s); // s is still the owner
    }

    println!("8. Mutable immutable references");
    {
        // having immutable references while having a mutable one
        // is allowed
        let mut s = String::from("hello rust");
        {
            let r1 = &mut s; // r1 takes the ownership of s
            let r2 = &r1;
            let r3 = &r1;
            // let r4 = &s; // not allowed as s is no longer the owner

            println!("\t{}", r1);
            println!("\t{}", r2);
            println!("\t{}", r3);
        }
        println!("\t{}", s); // s is the owner of itself again
    }

    println!("9. Multiple mutable and immutable reference");
    {
        // having a mutable reference while having an immutable one
        // is not allowed
        let mut s = String::from("bye");

        let r1 = &s;
        let r2 = &s;
        //let r3 = &mut s; // not allowed
        println!("\t{}, {}", r1, r2);
        let r3 = &mut s; // allowed as r1 and r2 are no longer in scope
        println!("\t{}", r3);
    }

    println!("10. Dangling reference");
    {
        // let s = func_dangling_ref(); // not allowed
    }
}

// fn func_dangling_ref() -> &String {
//     let s = String::from("hello world");
//     &s
// }
// lifetime of s is over

fn func_mut_ref(s: &mut String) {
    s.push_str(" from here");
}

fn func_str_own(s: String) -> usize {
    s.len()
}

fn func_ref(s: &str) -> usize {
    s.len()
}
