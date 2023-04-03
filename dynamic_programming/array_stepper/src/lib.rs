use std::collections::HashMap;

/// Takes in an array of numbers as an argument. You
/// start at the first position of the array. The
/// function should return a boolean indicating whether or
/// not it is possible to reach the last position of the
/// array. When situated at some position of the array, you
/// may take a maximum number of steps based on the
/// number at that position.
///
/// For example, given:
/// ``    
///     idx =  0  1  2  3  4  5
/// numbers = [2, 4, 2, 0, 0, 1]
/// ``
/// The answer is true.
/// We start at idx 0, we could take 1 step or 2 steps forward.
/// The correct choice is to take 1 step to idx 1.
/// Then take 4 steps forward to the end at idx 5.
pub fn array_stepper<const N: usize>(nums: [usize; N]) -> bool {
    array_stepper_with_cache(nums, 0, &mut HashMap::new())
}

pub fn array_stepper_with_cache<const N: usize>(
    nums: [usize; N],
    idx: usize,
    cache: &mut HashMap<usize, bool>,
) -> bool {
    // If the calculated value is in cache
    // then return it
    if let Some(&value) = cache.get(&idx) {
        return value;
    }

    if idx >= nums.len() - 1 {
        return true;
    }

    let max_step = nums[idx];
    for step in 1..=max_step {
        if array_stepper_with_cache(nums, idx + step, cache) {
            cache.insert(idx, true);
            return true;
        }
    }

    cache.insert(idx, false);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_stepper_00() {
        let result = array_stepper([2, 4, 2, 0, 0, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn array_stepper_01() {
        let result = array_stepper([2, 3, 2, 0, 0, 1]);
        assert_eq!(result, false);
    }

    #[test]
    fn array_stepper_02() {
        let result = array_stepper([3, 1, 3, 1, 0, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn array_stepper_03() {
        let result = array_stepper([4, 1, 5, 1, 1, 1, 0, 4]);
        assert_eq!(result, true);
    }

    #[test]
    fn array_stepper_04() {
        let result = array_stepper([4, 1, 2, 1, 1, 1, 0, 4]);
        assert_eq!(result, false);
    }

    #[test]
    fn array_stepper_05() {
        let result = array_stepper([1, 1, 1, 1, 1, 0]);
        assert_eq!(result, true);
    }

    #[test]
    fn array_stepper_06() {
        let result = array_stepper([1, 1, 1, 1, 0, 0]);
        assert_eq!(result, false);
    }

    #[test]
    fn array_stepper_07() {
        let result = array_stepper([
            31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
            9, 8, 7, 6, 5, 3, 2, 1, 0, 0, 0,
        ]);
        assert_eq!(result, false);
    }
}
