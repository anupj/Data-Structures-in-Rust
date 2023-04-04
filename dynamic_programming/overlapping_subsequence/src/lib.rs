use std::collections::HashMap;

/// Takes in two strings as arguments. The function
/// should return the length of the longest overlapping
/// subsequence.
///
/// A subsequence of a string can be created by
/// deleting any characters of the string, while
/// maintaining the relative order of characters.
fn longest_overlapping_subsequence(str1: &str, str2: &str) -> usize {
    let str1_chars: Vec<char> = str1.chars().collect();
    let str2_chars: Vec<char> = str2.chars().collect();
    let mut memo = HashMap::new();
    los_recursive(
        &str1_chars,
        str1_chars.len(),
        &str2_chars,
        str2_chars.len(),
        &mut memo,
    )
}

fn los_recursive(
    str1: &[char],
    str1_len: usize,
    str2: &[char],
    str2_len: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if str1_len == 0 || str2_len == 0 {
        return 0;
    }

    if let Some(&value) = memo.get(&(str1_len, str2_len)) {
        return value;
    }

    let result = if str1[str1_len - 1] == str2[str2_len - 1] {
        1 + los_recursive(str1, str1_len - 1, str2, str2_len - 1, memo)
    } else {
        std::cmp::max(
            los_recursive(str1, str1_len - 1, str2, str2_len, memo),
            los_recursive(str1, str1_len, str2, str2_len - 1, memo),
        )
    };

    memo.insert((str1_len, str2_len), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_overlapping_subsequence_00() {
        let result = longest_overlapping_subsequence("dogs", "daogt");
        assert_eq!(result, 3);
    }

    #[test]
    fn longest_overlapping_subsequence_01() {
        let result = longest_overlapping_subsequence("xcyats", "criaotsi");
        assert_eq!(result, 4);
    }

    #[test]
    fn longest_overlapping_subsequence_02() {
        let result = longest_overlapping_subsequence("xfeqortsver", "feeeuavoeqr");
        assert_eq!(result, 5);
    }

    #[test]
    fn longest_overlapping_subsequence_03() {
        let result =
            longest_overlapping_subsequence("kinfolklivemustache", "bespokekinfolksnackwave");
        assert_eq!(result, 11);
    }

    #[test]
    fn longest_overlapping_subsequence_04() {
        let result = longest_overlapping_subsequence(
            "mumblecorebeardleggingsauthenticunicorn",
            "succulentspughumblemeditationlocavore",
        );
        assert_eq!(result, 15);
    }
}
