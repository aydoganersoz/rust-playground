#[allow(clippy::unreadable_literal)]

// integer type
fn test1() {
    let i8_max: i8 = std::i8::MAX;
    let i8_min: i8 = std::i8::MIN;
    assert_eq!(i8_max, 127);
    assert_eq!(i8_min, -128);

    let u8_max: u8 = std::u8::MAX;
    let u8_min: u8 = std::u8::MIN;
    assert_eq!(u8_max, 255);
    assert_eq!(u8_min, 0);

    let i16_max: i16 = std::i16::MAX;
    let i16_min: i16 = std::i16::MIN;
    assert_eq!(i16_max, 32767);
    assert_eq!(i16_min, -32768);

    let u16_max: u16 = std::u16::MAX;
    let u16_min: u16 = std::u16::MIN;
    assert_eq!(u16_max, 65535);
    assert_eq!(u16_min, 0);

    let i32_max: i32 = std::i32::MAX;
    let i32_min: i32 = std::i32::MIN;
    assert_eq!(i32_max, 2147483647);
    assert_eq!(i32_min, -2147483648);

    let u32_max: u32 = std::u32::MAX;
    let u32_min: u32 = std::u32::MIN;
    assert_eq!(u32_max, 4294967295);
    assert_eq!(u32_min, 0);

    let i64_max: i64 = std::i64::MAX;
    let i64_min: i64 = std::i64::MIN;
    assert_eq!(i64_max, 9223372036854775807);
    assert_eq!(i64_min, -9223372036854775808);

    let u64_max: u64 = std::u64::MAX;
    let u64_min: u64 = std::u64::MIN;
    assert_eq!(u64_max, 18446744073709551615);
    assert_eq!(u64_min, 0);

    let i128_max: i128 = std::i128::MAX;
    let i128_min: i128 = std::i128::MIN;
    assert_eq!(i128_max, 170141183460469231731687303715884105727);
    assert_eq!(i128_min, -170141183460469231731687303715884105728);

    let u128_max: u128 = std::u128::MAX;
    let u128_min: u128 = std::u128::MIN;
    assert_eq!(u128_max, 340282366920938463463374607431768211455);
    assert_eq!(u128_min, 0);

    let iarch_max: isize = std::isize::MAX;
    let iarch_min: isize = std::isize::MIN;
    assert_eq!(iarch_max, 9223372036854775807);
    assert_eq!(iarch_min, -9223372036854775808);

    let uarch_max: usize = std::usize::MAX;
    let uarch_min: usize = std::usize::MIN;
    assert_eq!(uarch_max, 18446744073709551615);
    assert_eq!(uarch_min, 0);
}

// integer literals
fn test2() {
    let i = 16_756_432; // decimal with visual separator
    assert_eq!(i, 16756432);

    let i = 16756432; // decimal
    assert_eq!(i, 16756432);

    let i = 0xCAFE; // hexadecimal
    assert_eq!(i, 51966);

    let i = 0o16; // octal
    assert_eq!(i, 14);

    let i = 0b1010_0101; // binary with visual separator
    assert_eq!(i, 165);

    let i = 0b10100101; // binary
    assert_eq!(i, 165);

    let i = b'A'; // byte
    assert_eq!(i, 65);
}

// type suffix
fn test3() {
    let i = 5u8;
    assert_eq!(i, 5);

    let i = -5i8;
    assert_eq!(i, -5);

    // let i = 555555u8; // not allowed

    // let i = -5u8; // not allowed

    let i = 5u16;
    assert_eq!(i, 5);

    let i = -5i16;
    assert_eq!(i, -5);

    let i = 5u32;
    assert_eq!(i, 5);

    let i = -5i32;
    assert_eq!(i, -5);

    let i = 5u64;
    assert_eq!(i, 5);

    let i = -5i64;
    assert_eq!(i, -5);

    let i = 5u128;
    assert_eq!(i, 5);

    let i = -5i128;
    assert_eq!(i, -5);

    let i = 5usize;
    assert_eq!(i, 5);

    let i = -5isize;
    assert_eq!(i, -5);
}

// floating-point type
fn test4() {
    let i = 2.9876543210;
    assert_eq!(i, 2.9876543210);

    let i: f32 = 2.9876543210;
    assert_eq!(i, 2.9876543210);
}

// boolean type (encoded in 1 byte)
fn test5() {
    let i = false;
    assert_eq!(i, false);

    let i: bool = true;
    assert_eq!(i, true);
}

// character type (encoded in 4 bytes)
fn test6() {
    let i = '😻';
    assert_eq!(i, '😻');

    let i = 'z';
    assert_eq!(i, 'z');

    let i = 'ℤ';
    assert_eq!(i, 'ℤ');
}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}
