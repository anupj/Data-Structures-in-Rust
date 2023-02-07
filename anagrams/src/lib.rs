use std::collections::HashMap;

pub fn anagrams(left: &str, right: &str) -> bool {
    let mut left_map: HashMap<char, u32> = HashMap::new();
    let mut right_map: HashMap<char, u32> = HashMap::new();
    for ch in left.chars() {
        left_map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }
    for ch in right.chars() {
        right_map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }
    left_map.eq(&right_map)
    // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagrams() {
        assert_eq!(anagrams("restful", "fluster"), true);
        assert_eq!(anagrams("cats", "tocs"), false);
        assert_eq!(anagrams("monkeyswrite", "newyorktimes"), true);
        assert_eq!(anagrams("paper", "reapa"), false);
        assert_eq!(anagrams("elbow", "below"), true);
        assert_eq!(anagrams("tax", "taxi"), false);
        assert_eq!(anagrams("taxi", "tax"), false);
        assert_eq!(anagrams("night", "thing"), true);
        assert_eq!(anagrams("abbc", "aabc"), false);
        assert_eq!(anagrams("po", "popp"), false);
        assert_eq!(anagrams("pp", "oo"), false);
    }
}
