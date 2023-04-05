use std::collections::HashMap;

/// takes in a string and an array of words as
/// arguments. The function should return boolean
/// indicating whether or not it is possible to
/// concatenate words of the array together to form the
/// string.
/// You may reuse words of the array as many times
/// as needed.
pub fn can_concat<'a, const N: usize>(s: &'a str, words: [&'a str; N]) -> bool {
    can_concat_recursive(s, words, &mut HashMap::new())
}

pub fn can_concat_recursive<'a, const N: usize>(
    s: &'a str,
    words: [&str; N],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // If the calculated value is in cache
    // then return it
    if let Some(&value) = memo.get(s) {
        return value;
    }

    if s.len() == 0 {
        return true;
    }

    for word in words {
        if s.starts_with(word) {
            let (_, suffix) = s.split_at(word.len());

            if can_concat_recursive(suffix, words, memo) {
                memo.insert(s, true);
                return true;
            }
        }
    }

    memo.insert(s, false);

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_concat_00() {
        let result = can_concat::<3>("oneisnone", ["one", "none", "is"]);
        assert_eq!(result, true);
    }

    #[test]
    fn can_concat_01() {
        let result = can_concat::<3>("oneisnone", ["on", "e", "is"]);
        assert_eq!(result, false);
    }

    #[test]
    fn can_concat_02() {
        let result = can_concat::<4>("oneisnone", ["on", "e", "is", "n"]);
        assert_eq!(result, true);
    }

    #[test]
    fn can_concat_03() {
        let result = can_concat::<4>("foodisgood", ["is", "g", "ood", "f"]);
        assert_eq!(result, true);
    }

    #[test]
    fn can_concat_04() {
        let result = can_concat::<2>("santahat", ["santah", "hat"]);
        assert_eq!(result, false);
    }

    #[test]
    fn can_concat_05() {
        let result = can_concat::<4>("santahat", ["santah", "san", "hat", "tahat"]);
        assert_eq!(result, true);
    }

    #[test]
    fn can_concat_06() {
        let result = can_concat::<6>(
            "rrrrrrrrrrrrrrrrrrrrrrrrrrx",
            ["r", "rr", "rrr", "rrrr", "rrrrr", "rrrrrr"],
        );
        assert_eq!(result, false);
    }

    #[test]
    fn can_concat_07() {
        let result = can_concat::<5>("fooisgood", ["foo", "is", "g", "ood", "f"]);
        assert_eq!(result, true);
    }
}
