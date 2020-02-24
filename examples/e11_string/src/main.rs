#[allow(unused_mut)]
#[allow(unused_variables)]

fn main() {
    println!("1. Declaring an empty string");
    let my_str = String::new();
    println!("\t{}", my_str);

    println!("2. Declaring a string (String) with initial value");
    let my_str = String::from("abcd");
    println!("\t{}", my_str);

    println!("3. Declaring a string (String) with initial value");
    let mut my_str = "1234".to_string();
    println!("\t{}", my_str);

    println!("4. Declaring a string (str) with initial value");
    let my_str = "zxcv";
    println!("\t{}", my_str);

    println!("5. String decomposition into bytes");
    let my_str = String::from("1234");
    println!("\t{:?}", my_str.as_bytes());

    println!("6. Capacity of a string");
    let my_str = String::from("1234567");
    println!("\t{}", my_str.capacity());

    println!("7. String vs str");
    let mut my_str_string = String::from("hello "); // mutable and can be treated
    let mut my_str_2_string = "hello ".to_string(); // mutable and can be treated
    let mut my_str_str = "hello "; // immutable and can't be treated
    println!("\tstring_string = {}", my_str_string);
    println!("\tstring_string_2 = {}", my_str_2_string);
    println!("\tstring_str = {}", my_str_str);
    my_str_string.push_str("rust");
    my_str_2_string.push_str("rust");
    // my_str_str.push_str("rust"); not allowed
    println!("\tstring_string = {}", my_str_string);
    println!("\tstring_string_2 = {}", my_str_2_string);
    println!("\tstring_str = {}", my_str_str);

    println!("8. Concatanating a character into a string");
    let mut my_str = String::from("1234567");
    // let mut my_str = "1234567".to_string(); // String (this would work)
    // let mut my_str = "1234567"; // str (this wouldn't work)
    println!("\t{}", my_str);
    my_str.push('8');
    println!("\t{}", my_str);

    println!("9. Concatanating a string into a string");
    let mut my_str = String::from("hello ");
    println!("\t{}", my_str);
    my_str.push_str("world!");
    println!("\t{}", my_str);

    println!("10. Concatanating a string into a string using +");
    let my_str_1 = String::from("joe");
    let my_str_2 = String::from("jane");
    let my_str_3 = String::from("patrick");
    let whole_string = my_str_1 + "-" + &my_str_2 + "-" + &my_str_3;
    println!("\t{}", whole_string);

    println!("11. String indexing");
    let my_str = String::from("hello");
    // let h = my_str[0]; // not allowed

    println!("12. Internal string representations");
    let my_str = String::from("Hi");
    println!("\t{}", my_str.len()); // 2 because each character is encoded in 1 byte
    let my_str = String::from("Здравствуйте");
    println!("\t{}", my_str.len()); // 24 because each character is encoded in 2 bytes
    println!("\t{}", my_str.chars().count()); // 12 because it's number of characters

    println!("13. String slicing");
    let my_str = "Здравствуйте";
    println!("\t{}", &my_str[0..4]); // because each character is encoded in 2 bytes
    let my_str = "hello";
    println!("\t{}", &my_str[0..4]); // because each character is encoded in 1 byte

    println!("14. Iterating over strings");
    let my_str = "Зд";
    for c in my_str.chars() {
        println!("\t{}", c); // 2 loops
    }
    for b in my_str.bytes() {
        println!("\t{}", b); // 4 loops because each character is encoded in 2 bytes
    }
}
