// loop (infinite)
fn test1() {
    let mut i = 0;

    loop {
        if i > 2 {
            break;
        }
        i += 1;
    }

    assert_eq!(3, i);
}

// loop returns a value
fn test2() {
    let mut i = 0;

    let ret_val = loop {
        if i > 8 {
            break i * 30;
        }
        i += 1;
    };

    assert_eq!(270, ret_val);
}

// while loop
fn test3() {
    let mut i = 0;

    while i <= 4 {
        i += 1;
    }

    assert_eq!(5, i);
}

// for loop
fn test4() {
    let mut cnt = 0;

    for i in 0..3 {
        cnt = i;
    }

    assert_eq!(2, cnt);
}

// for loop with custom step
fn test5() {
    let mut cnt = 0;

    for i in (0..6).step_by(2) {
        cnt = i;
    }

    assert_eq!(4, cnt);
}

// for loop with iterator
fn test6() {
    let mut ch = "?";
    let names = ["a", "b", "c"];

    for i in names.iter() {
        ch = i;
    }

    assert_eq!("c", ch);
}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}
