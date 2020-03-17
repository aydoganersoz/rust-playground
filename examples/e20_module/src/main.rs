mod my_module_1;

fn main() {
    println!("1. Module");
    my_module_1::func1();
    my_module_1::func2();
    my_module_1::my_inner_module_1::func1();
    my_module_1::my_inner_module_1::func2();
}
