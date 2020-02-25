#[allow(clippy::unreadable_literal)]

fn main() {
    println!("1. Integer types");

    let i8_max: i8 = std::i8::MAX;
    let i8_min: i8 = std::i8::MIN;
    println!("\ti8_min={}\n\ti8_max={}", i8_min, i8_max);

    let u8_max: u8 = std::u8::MAX;
    let u8_min: u8 = std::u8::MIN;
    println!("\tu8_min={}\n\tu8_max={}", u8_min, u8_max);

    let i16_max: i16 = std::i16::MAX;
    let i16_min: i16 = std::i16::MIN;
    println!("\ti16_min={}\n\ti16_max={}", i16_min, i16_max);

    let u16_max: u16 = std::u16::MAX;
    let u16_min: u16 = std::u16::MIN;
    println!("\tu16_min={}\n\tu16_max={}", u16_min, u16_max);

    let i32_max: i32 = std::i32::MAX;
    let i32_min: i32 = std::i32::MIN;
    println!("\ti32_min={}\n\ti32_max={}", i32_min, i32_max);

    let u32_max: u32 = std::u32::MAX;
    let u32_min: u32 = std::u32::MIN;
    println!("\tu32_min={}\n\tu32_max={}", u32_min, u32_max);

    let i64_max: i64 = std::i64::MAX;
    let i64_min: i64 = std::i64::MIN;
    println!("\ti64_min={}\n\ti64_max={}", i64_min, i64_max);

    let u64_max: u64 = std::u64::MAX;
    let u64_min: u64 = std::u64::MIN;
    println!("\tu64_min={}\n\tu64_max={}", u64_min, u64_max);

    let i128_max: i128 = std::i128::MAX;
    let i128_min: i128 = std::i128::MIN;
    println!("\ti128_min={}\n\ti128_max={}", i128_min, i128_max);

    let u128_max: u128 = std::u128::MAX;
    let u128_min: u128 = std::u128::MIN;
    println!("\tu128_min={}\n\tu128_max={}", u128_min, u128_max);

    let iarch_max: isize = std::isize::MAX;
    let iarch_min: isize = std::isize::MIN;
    println!("\tiarch_min={}\n\tiarch_max={}", iarch_min, iarch_max);

    let uarch_max: usize = std::usize::MAX;
    let uarch_min: usize = std::usize::MIN;
    println!("\tuarch_min={}\n\tuarch_max={}", uarch_min, uarch_max);

    println!("2. Integer literals");

    let i = 16_756_432;
    println!("\t16_756_432 = {} (decimal with visual seperator)", i);

    let i = 16756432;
    println!("\t16756432 = {} (decimal)", i);

    let i = 0xCAFE;
    println!("\t0xCAFE = {} (hexadecimal)", i);

    let i = 0o16;
    println!("\t0o16 = {} (octal)", i);

    let i = 0b1010_0101;
    println!("\t0b1010_0101 = {} (binary with visual seperator)", i);

    let i = 0b10100101;
    println!("\t0b10100101 = {} (binary)", i);

    let i = b'A';
    println!("\tb'A' = {} (byte)", i);

    println!("3. Type suffix");

    let i = 5u8;
    println!("\t5u8 = {}", i);

    let i = -5i8;
    println!("\t-5i8 = {}", i);

    // let i = 555555u8;
    println!("\t555555u8 = not allowed!");

    // let i = -5u8;
    println!("\t-5u8 = not allowed!");

    let i = 5u16;
    println!("\t5u16 = {}", i);

    let i = -5i16;
    println!("\t-5i16 = {}", i);

    let i = 5u32;
    println!("\t5u32 = {}", i);

    let i = -5i32;
    println!("\t-5i32 = {}", i);

    let i = 5u64;
    println!("\t5u64 = {}", i);

    let i = -5i64;
    println!("\t-5i64 = {}", i);

    let i = 5u128;
    println!("\t5u128 = {}", i);

    let i = -5i128;
    println!("\t-5i128 = {}", i);

    let i = 5usize;
    println!("\t5usize = {}", i);

    let i = -5isize;
    println!("\t-5isize = {}", i);

    println!("4. Floating-point types");

    let i = 2.9876543210;
    println!("\tf64 (default) = {:.10}", i);

    let i: f32 = 2.012345;
    println!("\tf32 = {:.6}", i);

    // encoded in 1 byte
    println!("5. Boolean type");

    let i = false; // implicit
    println!("\t{}", i);

    let i: bool = true; // explicit
    println!("\t{}", i);

    // encoded in 4 bytes in rust and a lot more than ASCII
    println!("6. Character type");

    let i = '😻';
    println!("\t{}", i);

    let i = 'z';
    println!("\t{}", i);

    let i = 'ℤ';
    println!("\t{}", i);
}
