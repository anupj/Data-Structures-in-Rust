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

/// Same as `compress()` but using
/// another algorithm to achieve the
/// same outcome
pub fn compress_differently(s: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    // keeps track of the current letter
    let mut i = 0;

    // keeps track of the transition to
    // a new letter in the string
    let mut j = 0;

    // Convert `&str` to a vec of chars
    // to access the elements via index
    // more easilty
    let s_vec: Vec<char> = s.chars().collect();

    // notice that we are looping
    // to an index beyond the end of
    // `vec`
    while j <= s_vec.len() {
        // value of `s` at index i
        let ith_value = s_vec.get(i).unwrap();

        // value of `s` at index `j`
        let jth_value = if j != s_vec.len() {
            // ensure that this is not
            // last iteration where `j` is
            // equal to `vec.len()`
            s_vec.get(j).unwrap()
        } else {
            &'0'
        };

        if ith_value == jth_value {
            j += 1;
        } else {
            // if `ith_value` is not
            // equal to `jth_value`
            // means that we have
            // transitioned to a new character
            // and therefore need to insert
            // the previous char in the result
            // with the count
            let char_count = j - i;

            if char_count == 1 {
                // if only single char found,
                // don't insert the count
                result.push(ith_value.to_string());
            } else {
                // otherwise insert count and char
                result.push(char_count.to_string());
                result.push(ith_value.to_string());
            }
            i = j;

            // don't forget to increment j
            // to exit the loop
            if j == s_vec.len() {
                j += 1;
            }
        }
    }

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

    #[test]
    fn test_compress_differently() {
        assert_eq!(compress_differently("ccaaatsss"), "2c3at3s");
        assert_eq!(compress_differently("ssssbbz"), "4s2bz");
        assert_eq!(compress_differently("ppoppppp"), "2po5p");
        assert_eq!(compress_differently("nnneeeeeeeeeeeezz"), "3n12e2z");
        assert_eq!(compress_differently("nnnneeeeeeeeeeeetzzzt"), "4n12et3zt");
        assert_eq!(compress_differently("yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"),
        "127y");
    }
}
