use std::collections::HashMap;

/// takes in a string and an array of words as
/// arguments. The function should return the minimum
/// number of words needed to build the string by
/// concatenating words of the array.
/// You may reuse words of the array as many times
/// as needed.

pub fn quickest_concat(s: &str, words: &[&str]) -> isize {
    let result = _quickest_concat(s, words, &mut HashMap::new());

    if result == isize::MAX - 1 {
        -1
    } else {
        result
    }
}

fn _quickest_concat(s: &str, words: &[&str], memo: &mut HashMap<String, isize>) -> isize {
    if let Some(&min) = memo.get(s) {
        return min;
    }

    if s.is_empty() {
        return 0;
    }

    // this is to avoid "attempt to add overflow"
    // error when we add `1` to the
    // `attempt` variable
    let mut min = isize::MAX - 1;

    for word in words {
        if let Some(suffix) = s.strip_prefix(word) {
            let attempt = 1 + _quickest_concat(suffix, words, memo);
            min = min.min(attempt);
        }
    }

    memo.insert(s.to_string(), min);
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_concat_00() {
        let result = quickest_concat("caution", &vec!["ca", "ion", "caut", "ut"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn can_concat_01() {
        let result = quickest_concat("caution", &vec!["ion", "caut", "caution"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn can_concat_02() {
        let result = quickest_concat(
            "respondorreact",
            &vec!["re", "or", "spond", "act", "respond"],
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn can_concat_03() {
        let result = quickest_concat("simchacindy", &vec!["sim", "simcha", "acindy", "ch"]);
        assert_eq!(result, 3);
    }

    #[test]
    fn can_concat_04() {
        let result = quickest_concat("simchacindy", &vec!["sim", "simcha", "acindy"]);
        assert_eq!(result, -1);
    }

    #[test]
    fn can_concat_05() {
        let result = quickest_concat("uuuuuu", &vec!["u", "uu", "uuu", "uuuu"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn can_concat_06() {
        let result = quickest_concat("rongbetty", &vec!["wrong", "bet"]);
        assert_eq!(result, -1);
    }

    #[test]
    fn can_concat_07() {
        let result = quickest_concat(
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            &vec!["u", "uu", "uuu", "uuuu", "uuuuu"],
        );
        assert_eq!(result, 7);
    }
}
