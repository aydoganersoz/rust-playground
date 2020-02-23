#[allow(clippy::approx_constant)]

const PI: f64 = 3.14;
const UNIT: &str = "cm";

fn main() {
    let r = 2; // diameter
    let perimeter = f64::from(2) * PI * f64::from(r); // 2*pi*r

    println!(
        "Perimeter of a circle with a diameter of {0} is {1} {2}",
        r, perimeter, UNIT
    );
}
