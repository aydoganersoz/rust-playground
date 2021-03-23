#[allow(dead_code)]
fn add_three(param: i32) -> i32 {
    param + 3
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_positive_parameter() {
        assert_eq!(6, add_three(3));
        assert_eq!(48, add_three(45));
    }

    #[test]
    fn test_negative_parameter() {
        assert_eq!(0, add_three(-3));
        assert_eq!(-30, add_three(-33));
    }

    #[test]
    fn test_zero_parameter() {
        assert_eq!(3, add_three(0));
    }
}

pub fn test() {}
