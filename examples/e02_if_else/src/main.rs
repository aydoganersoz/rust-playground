fn main() {
    let temperature = 23;

    println!("`if`-`else if`-`else`");
    if temperature < 21 {
        println!("\tThe temperature is low");
    } else if temperature >= 21 && temperature < 31 {
        println!("\tThe temperature is normal");
    } else {
        println!("\tThe temperature is high");
    }

    println!("`if` in a `let` statement");
    let season = if temperature >= 28 {
        "summer"
    } else {
        "not summer"
    };
    println!("\tThe season is {0}", season);
}
