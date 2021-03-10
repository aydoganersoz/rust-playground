#[allow(clippy::approx_constant)]

const PI: f64 = 3.14;
const UNIT: &str = "cm";

fn test1() {
    let diameter = 2;
    let perimeter = f64::from(2) * PI * f64::from(diameter); // 2*pi*r

    assert_eq!(perimeter, 12.56);
    assert_eq!(UNIT, "cm");
}

pub fn test() {
    test1();
}
