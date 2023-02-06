/// takes in a string as an argument.
/// The function should return a
/// compressed version of the string
/// where consecutive occurrences of
/// the same characters are compressed
/// into the number of occurrences
/// followed by the character.
/// Single character occurrences
/// should not be changed.
/// 'aaa' compresses to '3a'
/// 'cc' compresses to '2c'
/// 't' should remain as 't'
pub fn compress(s: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut prev_ch: char = s.chars().next().unwrap();
    let mut ch_count = 1;
    for (loop_count, ch) in s.get(1..s.len()).unwrap().chars().enumerate() {
        if ch == prev_ch {
            ch_count += 1;
        }

        if ch != prev_ch {
            if ch_count > 1 {
                result.push(ch_count.to_string());
            }
            result.push(prev_ch.to_string());
            ch_count = 1;
        }

        if loop_count == s.len() - 2 {
            if ch_count > 1 {
                result.push(ch_count.to_string());
            }
            result.push(ch.to_string());
        }

        prev_ch = ch;
    }
    println!("the hash map output is {:?}", result.join(""));
    // "".to_string()
    result.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        assert_eq!(compress("ccaaatsss"), "2c3at3s");
        assert_eq!(compress("ssssbbz"), "4s2bz");
        assert_eq!(compress("ppoppppp"), "2po5p");
        assert_eq!(compress("nnneeeeeeeeeeeezz"), "3n12e2z");
        assert_eq!(compress("nnnneeeeeeeeeeeetzzzt"), "4n12et3zt");
        assert_eq!(compress("yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"),
        "127y");
    }
}
