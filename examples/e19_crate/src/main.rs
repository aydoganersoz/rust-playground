use rand::Rng;

fn main() {
    println!("1. Crate");

    let mut rng = rand::thread_rng();

    let r1: u8 = rng.gen();
    let r2: u16 = rng.gen();
    println!("\tu8:{}", r1);
    println!("\tu16:{}", r2);
    println!("\tbool:{}", rng.gen::<bool>());
    println!("\tu32:{}", rng.gen::<u32>());
    println!("\ti32:{}", rng.gen::<i32>());
    println!("\tfloat:{}", rng.gen::<f64>());
}
