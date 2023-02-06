use std::usize;

/// takes in a string as an argument.
/// The input string will be formatted
/// into multiple groups according to the
/// following pattern:
/// `<number><char>`
/// for example, '2c', '3a', '44f' etc.
/// The function should return an uncompressed
/// version of the string where each 'char'
/// of a group is repeated 'number' times
/// consecutively. You may assume that the
/// input string is well-formed according
/// to the previously mentioned pattern.
pub fn uncompress(pattern: &str) -> String {
    pattern
        // splits the `&str` when it encounters
        // a non-numeric alphabet, so `3n12e2z`
        // is split into `3n`, `12e`, `2z`
        .split_inclusive(char::is_alphabetic)
        .map(|val| {
            // split the `&str` e.g. `12e`
            // into num and alphabet tuple
            // by splitting at the last character
            let (num_times, character) = val.split_at(val.len() - 1);
            // conver the `&str` to `String`
            // and return the values repeated
            // `num_times`
            character.to_string().repeat(num_times.parse::<usize>().unwrap())
        })
        .collect()
}

/// Same input and output as `uncompress()`
/// but without relying so much on iterators
pub fn uncompress_without_iter(pattern: &str) -> String {
    let mut result: Vec<String> = Vec::new();

    const NUMS: &str = "0123456789";
    // index to track the number
    // in the string
    let mut i = 0usize;

    // index to track the letter
    // in the string
    let mut j = 0usize;

    while j < pattern.len() {
        // get the `j`th character from the input
        // string
        let letter = pattern.chars().nth(j).unwrap();
        if NUMS.contains(letter) {
            // if `j` is pointing to a number
            // then move on
            j += 1;
        } else {
            // if `j` is pointing to a letter
            // i.e. non-numeric char then
            // grab the number before the letter
            // and repeat the character
            let num = pattern.get(i..j).unwrap().parse::<usize>().unwrap();
            result.push(letter.to_string().repeat(num));
            j += 1;
            i = j;
        }
    }
    result.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uncompress() {
        assert_eq!(uncompress("3n12e2z"), "nnneeeeeeeeeeeezz");
        assert_eq!(uncompress("2c3a1t"), "ccaaat");
        assert_eq!(uncompress("4s2b"), "ssssbb");
        assert_eq!(uncompress("2p1o5p"), "ppoppppp");
        assert_eq!(uncompress("127y"), "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy");
        assert_eq!(
            uncompress("10a12b13c"),
            "aaaaaaaaaabbbbbbbbbbbbccccccccccccc"
        )
    }

    #[test]
    fn test_uncompress_without_iter() {
        assert_eq!(uncompress_without_iter("3n12e2z"), "nnneeeeeeeeeeeezz");
        assert_eq!(uncompress_without_iter("2c3a1t"), "ccaaat");
        assert_eq!(uncompress_without_iter("4s2b"), "ssssbb");
        assert_eq!(uncompress_without_iter("2p1o5p"), "ppoppppp");
        assert_eq!(uncompress_without_iter("127y"), "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy");
        assert_eq!(
            uncompress_without_iter("10a12b13c"),
            "aaaaaaaaaabbbbbbbbbbbbccccccccccccc"
        )
    }
}
