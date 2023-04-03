use std::collections::HashMap;

/// Takes a target number as an argument. The
/// function should return the minimum number of
/// perfect squares that sum to the target. A
/// perfect square is a number of the form
/// (i*i) where i >= 1.
///
/// For example: 1, 4, 9, 16 are perfect squares, but 8 is not perfect square.
pub fn summing_squares(num: isize) -> isize {
    summing_squares_with_cache(num, &mut HashMap::new())
}

pub fn summing_squares_with_cache(num: isize, cache: &mut HashMap<isize, isize>) -> isize {
    // If the calculated value is in cache
    // then return it
    if let Some(&value) = cache.get(&num) {
        return value;
    }

    if num == 0 {
        return 0;
    }

    let mut min_squares = isize::MAX;

    let sqrt_num = (num as f64).sqrt() as isize;

    for i in 1..=sqrt_num {
        let square = i * i;
        let num_squares = 1 + summing_squares_with_cache(num - square, cache);
        min_squares = min_squares.min(num_squares);
    }

    cache.insert(num, min_squares);

    min_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summing_squares_00() {
        let result = summing_squares(8);
        assert_eq!(result, 2);
    }

    #[test]
    fn summing_squares_01() {
        let result = summing_squares(9);
        assert_eq!(result, 1);
    }

    #[test]
    fn summing_squares_02() {
        let result = summing_squares(12);
        assert_eq!(result, 3);
    }

    #[test]
    fn summing_squares_03() {
        let result = summing_squares(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn summing_squares_04() {
        let result = summing_squares(31);
        assert_eq!(result, 4);
    }

    #[test]
    fn summing_squares_05() {
        let result = summing_squares(50);
        assert_eq!(result, 2);
    }

    #[test]
    fn summing_squares_06() {
        let result = summing_squares(68);
        assert_eq!(result, 2);
    }

    #[test]
    fn summing_squares_07() {
        let result = summing_squares(87);
        assert_eq!(result, 4);
    }
}
