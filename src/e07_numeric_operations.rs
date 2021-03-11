// basic numeric operations
fn test1() {
    let ret = 5 + 10;
    assert_eq!(ret, 15);

    let ret = 95.5 - 4.3;
    assert_eq!(ret, 91.2);

    let ret = 4 * 30;
    assert_eq!(ret, 120);

    let ret = 56.7 / 32.2;
    assert_eq!(ret, 1.7608695652173911);

    let ret = 43 % 5;
    assert_eq!(ret, 3);
}

pub fn test() {
    test1();
}
