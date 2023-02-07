use std::collections::HashMap;

/// Takes in a string as an argument.
/// The function should return the most
/// frequent character of the string.
/// If there are ties, return the
/// character that appears earlier in the string.
///You can assume that the input string is non-empty.
pub fn most_frequent_char(s: &str) -> char {
    let mut char_map: HashMap<char, u32> = HashMap::new();
    for ch in s.chars() {
        char_map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut result = &'a';
    let mut count = 0u32;
    for ch in char_map.keys() {
        if char_map.get(ch).unwrap() > &count {
            result = ch;
            count = *char_map.get(ch).unwrap();
        }
    }
    result.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_frequent_char() {
        assert_eq!(most_frequent_char("bookeeper"), 'e');
        assert_eq!(most_frequent_char("david"), 'd');
        assert_eq!(most_frequent_char("abby"), 'b');
        assert_eq!(most_frequent_char("mississippi"), 'i');
        assert_eq!(most_frequent_char("potato"), 'o');
        assert_eq!(most_frequent_char("eleventennine"), 'e');
        assert_eq!(most_frequent_char("riverbed"), 'r');
    }
}
