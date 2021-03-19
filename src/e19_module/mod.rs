mod module_1;
mod module_2;
mod module_3;

fn test1() {
    println!("1. Module");
    module_1::func1();
    module_1::func2();
    module_1::inner_module_1::func1();
    module_1::inner_module_1::func2();

    println!("2. Nested module");
    module_2::func1();
    module_2::func2();
    module_2::my_nested_module::func1();
    // module_2::func3(); // private function
    // module_2::my_nested_module::func2(); // private function

    println!("3. Public struct - enum");
    module_3::func1();
    // following struct declaration is not allowed as it has a private field that does not permit to be accessed
    /*
    let mut s = module_3::MyStruct {
        // field_1: 2, // private field
        field_2: -2,
    };*/
    let mut s = module_3::MyStruct::new(); // returns an instance of type MyStruct
    println!("\tStruct initialization (with private field) in main()");
    println!("\t{:?}", s);
    println!("\tStruct modification in main()");
    // s.field_1 = 2; // private field
    s.field_2 = -4;
    println!("\t{:?}", s);
    let pub_enum = module_3::MyPubEnum::Field1;
    println!("\t{:?}", pub_enum);
    // let priv_enum = module_3::MyPrivEnum::Field1; // private enum
}

pub fn test() {
    test1();
}
