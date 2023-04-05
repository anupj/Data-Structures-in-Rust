use std::collections::HashMap;

/// Takes in an array of numbers as an argument. The
/// function should return the maximum sum of non-adjacent
/// elements in the array. There is no limit on how
/// many elements can be taken into the sum as long as
/// they are not adjacent.
///
/// For example, given:
/// [2, 4, 5, 12, 7]
/// The maximum non-adjacent sum is 16, because 4 + 12.
/// 4 and 12 are not adjacent in the array.
pub fn non_adjacent_sum<const N: usize>(nums: [usize; N]) -> usize {
    nas_recursive(nums, 0, &mut HashMap::new())
}

pub fn nas_recursive<const N: usize>(
    nums: [usize; N],
    idx: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    // If the calculated value is in cache
    // then return it
    if let Some(&value) = memo.get(&idx) {
        return value;
    }

    if idx >= nums.len() {
        return 0;
    }

    let include = nums[idx] + nas_recursive(nums, idx + 2, memo);
    let exclude = nas_recursive(nums, idx + 1, memo);

    let result = include.max(exclude);

    memo.insert(idx, result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_adjacent_sum_00() {
        let result = non_adjacent_sum::<5>([2, 4, 5, 12, 7]);
        assert_eq!(result, 16);
    }

    #[test]
    fn non_adjacent_sum_01() {
        let result = non_adjacent_sum::<4>([7, 5, 5, 12]);
        assert_eq!(result, 19);
    }

    #[test]
    fn non_adjacent_sum_02() {
        let result = non_adjacent_sum::<6>([7, 5, 5, 12, 17, 29]);
        assert_eq!(result, 48);
    }

    #[test]
    fn non_adjacent_sum_03() {
        let result = non_adjacent_sum::<28>([
            72, 62, 10, 6, 20, 19, 42, 46, 24, 78, 30, 41, 75, 38, 23, 28, 66, 55, 12, 17, 9, 12,
            3, 1, 19, 30, 50, 20,
        ]);
        assert_eq!(result, 488);
    }

    #[test]
    fn non_adjacent_sum_04() {
        let result = non_adjacent_sum::<61>([
            72, 62, 10, 6, 20, 19, 42, 46, 24, 78, 30, 41, 75, 38, 23, 28, 66, 55, 12, 17, 83, 80,
            56, 68, 6, 22, 56, 96, 77, 98, 61, 20, 0, 76, 53, 74, 8, 22, 92, 37, 30, 41, 75, 38,
            23, 28, 66, 55, 12, 17, 72, 62, 10, 6, 20, 19, 42, 46, 24, 78, 42,
        ]);
        assert_eq!(result, 1465);
    }
}
