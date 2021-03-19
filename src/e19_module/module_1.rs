pub mod inner_module_1;

pub fn func1() -> u8 {
    42
}

pub fn func2() -> u8 {
    let f1_res = func1();
    let f2_res = inner_module_1::func2();
    let f2_res_2 = crate::e19_module::module_1::inner_module_1::func2(); // same as above

    assert_eq!(f2_res, f2_res_2);

    f1_res + f2_res // 42 + 23
}
