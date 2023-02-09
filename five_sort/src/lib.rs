/// Five_sort takes in an array of numbers as an
/// argument. The function should rearrange
/// elements of the array such that all 5s
/// appear at the end. It should
/// perform this operation in-place by
/// mutating the original array.
/// It should return the array.
/// Elements that are not 5 can appear in
/// any order in the output, as long as
/// all 5s are at the end of the array.
pub fn five_sort(nums: &mut [u32]) -> &[u32] {
    // i is the pointer that traverses
    // the slice from the left
    let mut i: usize = 0;

    // and j traverses the array/slice
    // from the right
    let mut j: usize = nums.len() - 1;

    // loop until i & j meet in the middle
    while i <= j {
        // if `j` is pointing to 5
        // then move the pointer left
        if nums.get(j) == Some(&5) {
            j -= 1;
        } else if nums.get(i) == Some(&5) {
            // if `i` is pointing to 5
            // and `j` is NOT pointing to 5
            // then swap i and j values
            nums.swap(i, j);
            i += 1;
        } else {
            // otherwise just increment
            // i
            i += 1;
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_sort() {
        assert_eq!(
            five_sort(&mut [12, 5, 1, 5, 12, 7]),
            &[12, 7, 1, 12, 5, 5]
        );
        assert_eq!(
            five_sort(&mut [5, 2, 5, 6, 5, 1, 10, 2, 5, 5]),
            &[2, 2, 10, 6, 1, 5, 5, 5, 5, 5]
        );
        assert_eq!(
            five_sort(&mut [5, 5, 5, 1, 1, 1, 4]),
            &[4, 1, 1, 1, 5, 5, 5]
        );
        assert_eq!(
            five_sort(&mut [5, 5, 6, 5, 5, 5, 5]),
            &[6, 5, 5, 5, 5, 5, 5]
        );
        assert_eq!(
            five_sort(&mut [5, 1, 2, 5, 5, 3, 2, 5, 1, 5, 5, 5, 4, 5]),
            &[4, 1, 2, 1, 2, 3, 5, 5, 5, 5, 5, 5, 5, 5]
        );
    }
}
