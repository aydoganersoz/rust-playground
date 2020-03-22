pub mod my_nested_module {
    pub fn func1() {
        println!("\tI am func1 in my_nested_module");
        func2();
    }

    fn func2() {
        println!("\tI am func2 in my_nested_module");
        super::func3();
    }
}

pub fn func1() {
    println!("\tI am func1 in my_module_2");
}

pub fn func2() {
    println!("\tI am func2 in my_module_2");
    my_nested_module::func1();
    func3();
}

// private function
fn func3() {
    println!("\tI am func3 in my_module_2");
}
