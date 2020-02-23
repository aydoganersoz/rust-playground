#[allow(unused_mut)]

fn main() {
    println!("Declaring an empty string");
    let empty_string = String::new();
    println!("\tlet empty_string = String::new();");
    println!("\tempty_string = {}", empty_string);

    println!("Declaring a string (String) with initial value");
    let init_string_string = String::from("initial value");
    println!("\tlet init_string_string = String::from(\"initial value\");");
    println!("\tinit_string_string = {}", init_string_string);

    println!("Declaring a string (String) with initial value");
    let mut init_string_string_2 = "initial value 3".to_string();
    println!("\tlet init_string_string_2 = \"initial value 3\";");
    println!("\tinit_string_string_2 = {}", init_string_string_2);

    println!("Declaring a string (str) with initial value");
    let init_string_str = "initial value 2";
    println!("\tlet init_string_str = \"initial value 2\";");
    println!("\tinit_string_str = {}", init_string_str);

    println!("String decomposition to bytes");
    let decompose_me = String::from("1234");
    let bytes = decompose_me.as_bytes();
    println!("\tlet decompose_me = String::from(\"1234\");");
    println!("\tlet bytes = decompose_me.as_bytes();");
    println!("\tbytes = {:?}", bytes);

    println!("Capacity of a string");
    let measure_me = String::from("1234567");
    println!("\tlet measure_me = String::from(\"1234567\");");
    println!("\tmeasure_me.capacity() = {}", measure_me.capacity());

    println!("String vs str");
    let mut string_string = String::from("hello "); // mutable and can be treated
    let mut string_string_2 = "hello ".to_string(); // mutable and can be treated
    let mut string_str = "hello rust"; // immutable and can't be treated
    println!("\tlet mut string_string = String::from(\"hello \");");
    println!("\tlet mut string_string_2 = \"hello \".to_string();");
    println!("\tlet mut string_str = \"hello rust\";");
    println!("\tstring_string = {}", string_string);
    println!("\tstring_string_2 = {}", string_string_2);
    println!("\tstring_str = {}", string_str);
    string_string.push_str("joe");
    string_string_2.push_str("jane");
    println!("\tstring_string.push_str(\"joe\");");
    println!("\tstring_string_2.push_str(\"jane\");");
    println!("\t// string_str.push_str(\"!\"); // not allowed");
    // string_str.push_str("!"); not allowed even though it's mutable
    println!("\tstring_string = {}", string_string);
    println!("\tstring_string_2 = {}", string_string_2);
    println!("\tstring_str = {}", string_str);

    println!("Concatanating a character to a string");
    let mut concat_me = String::from("1234567");
    // let mut concat_me = "1234567".to_string(); // String (this would work)
    // let mut concat_me = "1234567"; // str (this wouldn't work)
    println!("\tlet mut concat_me = String::from(\"1234567\");");
    println!("\tconcat_me = {}", concat_me);
    concat_me.push('8');
    println!("\tconcat_me.push('8');");
    println!("\tconcat_me = {}", concat_me);

    println!("Concatanating a string to a string");
    let mut concat_me = String::from("hello ");
    println!("\tlet mut concat_me = String::from(\"hello \");");
    println!("\tconcat_me = {}", concat_me);
    concat_me.push_str("world!");
    println!("\tconcat_me.push_str(\"world!\");");
    println!("\tconcat_me = {}", concat_me);

    println!("Concatanating a string to a string using +");
    let concat_me_1 = String::from("joe");
    let concat_me_2 = String::from("jane");
    let concat_me_3 = String::from("patrick");
    println!("\tlet concat_me_1 = String::from(\"joe\");");
    println!("\tlet concat_me_2 = String::from(\"jane\");");
    println!("\tlet concat_me_3 = String::from(\"patrick\");");
    let whole_string = concat_me_1 + "-" + &concat_me_2 + "-" + &concat_me_3;
    println!("\tlet whole_string = concat_me_1 + \"-\" + &concat_me_2 + \"-\" + &concat_me_3;");
    println!("\twhole_string = {}", whole_string);
}
