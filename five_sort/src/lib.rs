use std::borrow::Cow;

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
pub fn five_sort(nums: &[u32]) -> Cow<[u32]> {
    let mut slice =
        nums.iter().filter(|x| **x != 5).cloned().collect::<Vec<u32>>();

    slice.extend(nums.iter().filter(|x| **x == 5));
    slice.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_sort() {
        assert_eq!(
            five_sort(&[12, 5, 1, 5, 12, 7]).to_vec(),
            &[12, 1, 12, 7, 5, 5]
        );
        assert_eq!(
            five_sort(&[5, 2, 5, 6, 5, 1, 10, 2, 5, 5]).to_vec(),
            &[2, 6, 1, 10, 2, 5, 5, 5, 5, 5]
        );
        assert_eq!(
            five_sort(&[5, 5, 5, 1, 1, 1, 4]).to_vec(),
            &[1, 1, 1, 4, 5, 5, 5]
        );
        assert_eq!(
            five_sort(&[5, 5, 6, 5, 5, 5, 5]).to_vec(),
            &[6, 5, 5, 5, 5, 5, 5]
        );
        assert_eq!(
            five_sort(&[5, 1, 2, 5, 5, 3, 2, 5, 1, 5, 5, 5, 4, 5]).to_vec(),
            &[1, 2, 3, 2, 1, 4, 5, 5, 5, 5, 5, 5, 5, 5]
        );
    }
}
