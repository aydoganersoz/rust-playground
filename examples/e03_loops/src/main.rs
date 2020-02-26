fn main() {
    println!("1. Infinite loop with `loop`");
    let mut i = 0;
    loop {
        if i > 2 {
            break;
        }
        println!("\t{}", i);
        i += 1;
    }

    println!("2. Return a value from `loop`");
    let mut i = 0;
    let ret_val = loop {
        if i > 8 {
            break i * 30;
        }
        i += 1;
    };
    println!("\t{}", ret_val);

    println!("3. `while` loop");
    let mut i = 0;
    while i <= 4 {
        println!("\t{}", i);
        i += 1;
    }

    println!("4. `for` loop");
    for i in 0..3 {
        println!("\t{}", i);
    }

    println!("5. `for` loop with custom step");
    for i in (0..6).step_by(2) {
        println!("\t{}", i);
    }

    println!("6. `for` loop with iterator");
    let names = ["a", "b", "c"];
    for i in names.iter() {
        println!("\t{}", i);
    }
}
