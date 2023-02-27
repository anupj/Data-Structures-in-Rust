use std::f32;

/// Find the maximum value from the array
/// `nums`
pub fn max_value(nums: &[f32]) -> f32 {
    // seed the maximum with the
    // `MIN` of f32
    let mut maximum = f32::MIN;
    for num in nums {
        if num > &maximum {
            maximum = *num;
        }
    }
    maximum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_value() {
        let max_val = max_value(&[4.0, 7.0, 2.0, 8.0, 10.0, 9.0]);
        assert_eq!(max_val, 10.0);
        let max_val = max_value(&[10.0, 5.0, 40.0, 40.3]);
        assert_eq!(max_val, 40.3);
        let max_val = max_value(&[-5.0, -2.0, -1.0, -11.0]);
        assert_eq!(max_val, -1.0);
        let max_val = max_value(&[42.0]);
        assert_eq!(max_val, 42.0);
        let max_val = max_value(&[1000.0, 8.0]);
        assert_eq!(max_val, 1000.0);
        let max_val = max_value(&[1000.0, 8.0, 9000.0]);
        assert_eq!(max_val, 9000.0);
        let max_val = max_value(&[2.0, 5.0, 1.0, 4.0]);
        assert_eq!(max_val, 5.0);
    }
}
