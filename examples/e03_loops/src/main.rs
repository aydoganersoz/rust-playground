fn main() {
    println!("infinite loop with `loop`");
    let mut t = 0;
    loop {
        if t > 2 {
            break;
        }
        println!("\t{0}", t);
        t += 1;
    }

    println!("return a value from `loop`");
    let mut q = 0;
    let ret_val = loop {
        if q > 8 {
            break q * 30;
        }
        q += 1;
    };
    println!("\t`loop` returned {0}", ret_val);

    println!("`while` loop");
    let mut j = 0;
    while j <= 4 {
        println!("\t{0}", j);
        j += 1;
    }

    println!("`for` loop");
    for i in 0..3 {
        println!("\t{0}", i);
    }

    println!("`for` loop with custom step");
    for i in (0..6).step_by(2) {
        println!("\t{0}", i);
    }

    println!("`for` loop with iterator");
    let names = ["Me", "You", "Him"];
    for name in names.iter() {
        println!("\t{0}", name);
    }
}
