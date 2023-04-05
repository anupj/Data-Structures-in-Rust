use std::collections::HashMap;

/// Takes in a string as an argument. The function
/// should return the length of the longest
/// subsequence of the string that is also a palindrome.
/// A subsequence of a string can be created by deleting
/// any characters of the string, while maintaining the
/// relative order of characters.
pub fn max_palindrome(text: &str) -> usize {
    max_palindrome_recursive(text, 0, text.len() - 1, &mut HashMap::new())
}

pub fn max_palindrome_recursive(
    text: &str,
    start_idx: usize,
    end_idx: usize,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let key = format!("{},{}", start_idx, end_idx);
    // If the value is in cache,
    // the return it - memoization ftw
    if let Some(&value) = memo.get(&key) {
        return value;
    }

    if start_idx == end_idx {
        return 1;
    }

    if start_idx > end_idx {
        return 0;
    }

    let result =
        if text.chars().nth(start_idx) == text.chars().nth(end_idx) {
            2 + max_palindrome_recursive(text, start_idx + 1, end_idx - 1, memo)
        } else {
            max_palindrome_recursive(text, start_idx + 1, end_idx, memo)
                .max(max_palindrome_recursive(text, start_idx, end_idx - 1, memo))
        };

    memo.insert(key, result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_palindrome_00() {
        let result = max_palindrome("luwxult");
        assert_eq!(result, 5);
    }

    #[test]
    fn max_palindrome_01() {
        let result = max_palindrome("xyzaxxzy");
        assert_eq!(result, 6);
    }

    #[test]
    fn max_palindrome_02() {
        let result = max_palindrome("lol");
        assert_eq!(result, 3);
    }

    #[test]
    fn max_palindrome_03() {
        let result = max_palindrome("boabcdefop");
        assert_eq!(result, 3);
    }

    #[test]
    fn max_palindrome_04() {
        let result = max_palindrome("z");
        assert_eq!(result, 1);
    }

    #[test]
    fn max_palindrome_05() {
        let result = max_palindrome("chartreusepugvicefree");
        assert_eq!(result, 7);
    }

    #[test]
    fn max_palindrome_06() {
        let result = max_palindrome("qwueoiuahsdjnweuueueunasdnmnqweuzqwerty");
        assert_eq!(result, 15);
    }

    #[test]
    fn max_palindrome_07() {
        let result = max_palindrome(
            "enamelpinportlandtildecoldpressedironyflannelsemioticsedisonbulbfashionaxe",
        );
        assert_eq!(result, 31);
    }
}
