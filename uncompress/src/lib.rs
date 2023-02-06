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
            character
                .to_string()
                .repeat(num_times.parse::<usize>().unwrap())
        })
        .collect()
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
}
