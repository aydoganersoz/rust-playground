mod clock;

fn main() {
    let my_clock = clock::Clock {
        hours: 23,
        minutes: 59,
    };

    println!("1. ToString trait");
    println!("my_clock.to_string() = {:?}", my_clock.to_string());

    println!("2. From trait");
    println!("String::from(my_clock) = {:?}", String::from(my_clock));
}
