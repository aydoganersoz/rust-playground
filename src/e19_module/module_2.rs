pub mod nested_module {
    pub fn func1() -> u8 {
        let x = func2();

        assert_eq!(x, 24);

        x + 1 // 25
    }

    fn func2() -> u8 {
        let x = super::func3();

        assert_eq!(x, 13);

        x + 11 // 24
    }
}

pub fn func1() -> u8 {
    4
}

pub fn func2() -> u8 {
    let x = nested_module::func1();
    let y = func3();

    assert_eq!(x, 25);
    assert_eq!(y, 13);

    x + y // 25 + 13 = 38
}

// private function
fn func3() -> u8 {
    13
}
