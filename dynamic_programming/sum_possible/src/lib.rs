use std::collections::HashMap;

/// Takes in an amount and an array of positive
/// numbers. The function should return a boolean
/// indicating whether or not it is possible to
/// create the amount by summing numbers of the
/// array. You may reuse numbers of the array as
/// many times as necessary.
///
/// You may assume that the target amount is non-negative.
pub fn sum_possible_with_cache<const N: usize>(
    amount: isize,
    numbers: [isize; N],
    cache: &mut HashMap<isize, bool>,
) -> bool {
    // If the calculated value is in `cache`
    // then return it
    if let Some(&value) = cache.get(&amount) {
        return value;
    }

    // Base condition for exiting recursion
    if amount == 0 {
        return true;
    }

    // If the amount is negative means
    // sum cannot be found
    if amount < 0 {
        return false;
    }

    for num in numbers {
        if sum_possible_with_cache(amount - num, numbers, cache) {
            cache.insert(amount - num, true);
            return true;
        }
    }

    // if we have reached here means sum
    // cannot be found for this `amount`
    // so return false after recording it
    // in `cache`
    cache.insert(amount, false);
    false
}

pub fn sum_possible<const N: usize>(amount: isize, numbers: [isize; N]) -> bool {
    let mut cache = HashMap::new();
    sum_possible_with_cache(amount, numbers, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_possible_00() {
        let result = sum_possible::<3>(8, [5, 12, 4]);
        assert_eq!(result, true);
    }

    #[test]
    fn sum_possible_01() {
        let result = sum_possible::<4>(15, [6, 2, 10, 19]);
        assert_eq!(result, false);
    }

    #[test]
    fn sum_possible_02() {
        let result = sum_possible::<3>(13, [6, 2, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn sum_possible_03() {
        let result = sum_possible::<3>(103, [6, 20, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn sum_possible_04() {
        let result = sum_possible::<0>(12, []);
        assert_eq!(result, false);
    }

    #[test]
    fn sum_possible_05() {
        let result = sum_possible::<1>(12, [12]);
        assert_eq!(result, true);
    }

    #[test]
    fn sum_possible_06() {
        let result = sum_possible::<0>(0, []);
        assert_eq!(result, true);
    }

    #[test]
    fn sum_possible_07() {
        let result = sum_possible::<4>(271, [10, 8, 265, 24]);
        assert_eq!(result, false);
    }

    #[test]
    fn sum_possible_08() {
        let result = sum_possible::<3>(2017, [4, 2, 10]);
        assert_eq!(result, false);
    }

    #[test]
    fn sum_possible_09() {
        let result = sum_possible::<2>(13, [3, 5]);
        assert_eq!(result, true);
    }
}
