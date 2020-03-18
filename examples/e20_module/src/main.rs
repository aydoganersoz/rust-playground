mod my_module_1;
mod my_module_2;

fn main() {
    println!("1. Module");
    my_module_1::func1();
    my_module_1::func2();
    my_module_1::my_inner_module_1::func1();
    my_module_1::my_inner_module_1::func2();

    println!("2. Nested module");
    my_module_2::func1();
    my_module_2::func2();
    my_module_2::my_nested_module::func1();
    // my_module_2::func3(); // private function
    // my_module_2::my_nested_module::func2(); // private function
}
