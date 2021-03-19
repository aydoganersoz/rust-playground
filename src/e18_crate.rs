use rand::Rng;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn test1() {
    let mut rng = rand::thread_rng();

    assert_eq!(type_of(rng.gen::<u8>()), "u8");
    assert_eq!(type_of(rng.gen::<u16>()), "u16");
    assert_eq!(type_of(rng.gen::<bool>()), "bool");
    assert_eq!(type_of(rng.gen::<f64>()), "f64");
}

pub fn test() {
    test1();
}
