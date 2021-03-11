// tuple declaration (they can't grow or shrink in size)
fn test1() {
    let x: (i32, f64, u8, &str) = (500, 6.4, 1, "hello");

    // x.4 = 1; // not possible (size can't change)

    assert_eq!(x.0, 500);
    assert_eq!(x.1, 6.4);
    assert_eq!(x.2, 1);
    assert_eq!(x.3, "hello");
}

// mutation of tuple elements
fn test2() {
    let mut x: (i32, f64, u8, &str) = (500, 6.4, 1, "hello");

    x.0 = 501;
    x.1 = 6.3;
    x.2 = 2;
    x.3 = "hi";

    assert_eq!(x.0, 501);
    assert_eq!(x.1, 6.3);
    assert_eq!(x.2, 2);
    assert_eq!(x.3, "hi");
}

// tuple destruction
fn test3() {
    let t: (i32, i32, i32) = (1, 2, 3);
    let (x, y, z) = t;

    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);
}

// tuple of tuples
fn test4() {
    let t_child: (i32, bool) = (-1, true);
    let t_parent: (f64, (i32, bool)) = (0.003, t_child);

    assert_eq!(t_parent.0, 0.003);
    assert_eq!(t_parent.1, (-1, true));
    assert_eq!((t_parent.1).0, -1);
    assert_eq!((t_parent.1).1, true);
}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
}
