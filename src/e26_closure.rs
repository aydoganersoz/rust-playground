fn test1() {
    let is_odd = |num: u32| num % 2 != 0;

    assert_eq!(is_odd(2), false);
    assert_eq!(is_odd(21), true);
}

fn test2() {
    let is_prime = |num: &u32| !(2..*num).any(|factor| num % factor == 0);

    assert_eq!(is_prime(&2), true);
    assert_eq!(is_prime(&21), false);
    assert_eq!(is_prime(&17), true);
}

pub fn test() {
    test1();
    test2();
}
