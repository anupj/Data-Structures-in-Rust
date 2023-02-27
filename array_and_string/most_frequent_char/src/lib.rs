use std::collections::HashMap;

/// Takes in a string as an argument.
/// The function should return the most
/// frequent character of the string.
/// If there are ties, return the
/// character that appears earlier in the string.
///You can assume that the input string is non-empty.
/// Complexity:
/// Time = O(n)
/// Space = O(n)
pub fn most_frequent_char(s: &str) -> char {
    let mut char_map: HashMap<char, u32> = HashMap::new();

    // Insert the individual characters
    // and its frequency of occurrence
    // in a map
    for ch in s.chars() {
        char_map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut most_frequent = ' ';

    // parse the original string
    // and check the count for each
    // in order;
    for ch in s.chars() {
        if most_frequent == ' '
            || char_map.get(&ch) > char_map.get(&most_frequent)
        {
            most_frequent = ch;
        }
    }

    most_frequent
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
