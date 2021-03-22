mod module_1;
mod module_2;
mod module_3;

// module
fn test1() {
    assert_eq!(module_1::func1(), 42);
    assert_eq!(module_1::func2(), 65);
    assert_eq!(module_1::inner_module_1::func1(), 17);
    assert_eq!(module_1::inner_module_1::func2(), 23);
}

// nested module
fn test2() {
    assert_eq!(module_2::func1(), 4);
    assert_eq!(module_2::func2(), 38);
    assert_eq!(module_2::nested_module::func1(), 25);
    // assert_eq!(module_2::func3(), 13); // not possible (private function)
    // assert_eq!(module_2::nested_module::func2(), 24); // not possible (private function)
}

// struct and enum
fn test3() {
    assert_eq!(module_3::func1(), -1);

    // following struct declaration is not allowed as it has a private field that does not permit to be accessed
    /*
    let mut s = module_3::MyStruct {
        // field_1: 2, // private field
        field_2: -2,
    };*/

    let mut s = module_3::MyStruct::new(); // returns an instance of type MyStruct
    assert_eq!(s.field_2, -3);
    // assert_eq!(s.field_1, 0); not possible (field_1 is private)

    // s.field_1 = 2; // not possible (private field)
    s.field_2 = -4;
    assert_eq!(s.field_2, -4);

    let pub_enum = module_3::MyPubEnum::Field1;
    assert_eq!(pub_enum, module_3::MyPubEnum::Field1);

    // let priv_enum = module_3::MyPrivEnum::Field1; // not possible (private enum)
}

pub fn test() {
    test1();
    test2();
    test3();
}
