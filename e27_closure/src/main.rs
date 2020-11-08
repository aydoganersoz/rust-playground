fn main() {
    let is_odd = |num: u32| num % 2 != 0;

    for i in 2..1000 {
        if is_odd(i) {
            println!("{:?} is an odd number", i);
        }
    }

    let is_prime = |num: &u32| !(2..*num).any(|factor| num % factor == 0);

    for i in 2..1000 {
        if is_prime(&i) {
            println!("{:?} is a prime number", i);
        }
    }
}
