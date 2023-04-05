use std::collections::HashMap;

/// takes in a string and an array of words as
/// arguments. The function should return the minimum
/// number of words needed to build the string by
/// concatenating words of the array.
/// You may reuse words of the array as many times
/// as needed.
pub fn quickest_concat<'a, const N: usize>(s: &'a str, words: [&'a str; N]) -> isize {
    let result = quickest_concat_recursive(s, words, &mut HashMap::new());
    if result == usize::MAX {
        return -1;
    }
    result as isize
}

pub fn quickest_concat_recursive<'a, const N: usize>(
    s: &'a str,
    words: [&str; N],
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    // If the calculated value is in cache
    // then return it
    if let Some(&value) = memo.get(s) {
        return value;
    }

    if s.len() == 0 {
        return 0;
    }

    let mut min = usize::MAX;
    for word in words {
        if s.starts_with(word) {
            let (_, suffix) = s.split_at(word.len());

            let attempt = quickest_concat_recursive(suffix, words, memo);
            if attempt < usize::MAX {
                min = (attempt + 1).min(min);
            }
        }
    }

    memo.insert(s, min);

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_concat_00() {
        let result = quickest_concat::<4>("caution", ["ca", "ion", "caut", "ut"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn can_concat_01() {
        let result = quickest_concat::<3>("caution", ["ion", "caut", "caution"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn can_concat_02() {
        let result =
            quickest_concat::<5>("respondorreact", ["re", "or", "spond", "act", "respond"]);
        assert_eq!(result, 4);
    }

    #[test]
    fn can_concat_03() {
        let result = quickest_concat::<4>("simchacindy", ["sim", "simcha", "acindy", "ch"]);
        assert_eq!(result, 3);
    }

    #[test]
    fn can_concat_04() {
        let result = quickest_concat::<3>("simchacindy", ["sim", "simcha", "acindy"]);
        assert_eq!(result, -1);
    }

    #[test]
    fn can_concat_05() {
        let result = quickest_concat::<4>("uuuuuu", ["u", "uu", "uuu", "uuuu"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn can_concat_06() {
        let result = quickest_concat::<2>("rongbetty", ["wrong", "bet"]);
        assert_eq!(result, -1);
    }

    #[test]
    fn can_concat_07() {
        let result = quickest_concat::<5>(
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            ["u", "uu", "uuu", "uuuu", "uuuuu"],
        );
        assert_eq!(result, 7);
    }
}
