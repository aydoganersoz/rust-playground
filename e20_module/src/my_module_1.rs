pub mod my_inner_module_1;

pub fn func1() {
    println!("\tI am func1 in my_module_1");
}

pub fn func2() {
    func1();
    println!("\tI am func2 in my_module_1");
    my_inner_module_1::func2();
    crate::my_module_1::my_inner_module_1::func2(); // same as above
}
