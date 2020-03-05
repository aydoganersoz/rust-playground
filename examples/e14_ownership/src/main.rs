#[allow(unused_mut)]
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

    println!("8. Multiple mutable reference");
    {
        // only one mutable reference is allowed at a time
        let mut s = String::from("hello rust");
        {
            let r1 = &mut s; // r1 takes the ownership of s

            // println!("\t{}", s); // not allowed as s has no longer the ownership
            println!("\t{}", r1);
            // r1 releases the ownership of s
        }
        println!("\t{}", s); // s is the owner of itself again
        let r2 = &mut s; // r2 takes the ownership of s

        // println!("\t{}", s); // not allowed as s has no longer the ownership
        println!("\t{}", r2);
    }

    println!("9. Multiple mutable and immutable reference");
    {
        // we can't have a mutable reference while we have an immutable one
        let mut s = String::from("bye");

        // multiple immutable references are ok
        let r1 = &s;
        // multiple immutable references are ok
        let r2 = &s;
        //let r3 = &mut s; // not allowed
        println!("\t{}, {}", r1, r2);
    }

    println!("10. Multiple mutable and immutable reference - 2");
    {
        let mut s = String::from("bye");

        // multiple immutable references are ok
        let r1 = &s;
        // multiple immutable references are ok
        let r2 = &s;
        println!("\t{}, {}", r1, r2);
        let r3 = &mut s; // allowed as r1 and r2 are no longer in scope
        println!("\t{}", r3);
    }

    println!("11. Multiple mutable and immutable reference - 3");
    {
        // we can't have an immutable reference while we have a mutable one
        let mut s = String::from("bye");

        let r1 = &mut s;
        // let r2 = &s; // not allowed as we have already a mutable one
        println!("\t{}", r1);
        let r2 = &s; // allowed as r1 is no longer in scope
        println!("\t{}", r2);
    }

    println!("12. Dangling reference");
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
