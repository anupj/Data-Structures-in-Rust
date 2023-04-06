/// Takes in a compressed string as an argument. The
/// function should return the string decompressed.
///
/// The compression format of the input string is
/// 'n{subString}', where the subString within braces
/// should be repeated n times.
///
/// You may assume that every number n is guaranteed
/// to be an integer between 1 through 9.
///
/// You may assume that the input is valid and
/// the decompressed string will only contain
/// alphabetic characters.
pub fn decompress_braces(s: &str) -> String {
    let mut stack = Vec::new();
    let nums = "123456789";

    for c in s.chars() {
        if nums.contains(c) {
            // if its a number push it on
            // top of the stack
            stack.push(c.to_string());
        } else if c == '}' {
            // if its a closing brace
            // then start unspooling the
            // stack and create the expanded
            // string
            let mut segment = String::new();
            // go through the stack until you
            // hit a number
            while stack.last().unwrap().parse::<usize>().is_err() {
                let popped = stack.pop().unwrap();
                segment = format!("{}{}", popped, segment);
            }

            // Now get the number and...
            let number = stack.pop().unwrap().parse::<usize>().unwrap();

            // ..push the expanded string back on the stack
            stack.push(segment.repeat(number));
        } else if c != '{' {
            // Ignore opening brace
            // and push other characters
            // on top of the stack
            stack.push(c.to_string());
        }
    }

    // If the stack is empty means
    // we correctly matched all brackets
    stack.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_braces_00() {
        let result = decompress_braces("2{q}3{tu}v");
        assert_eq!(result, "qqtututuv");
    }

    #[test]
    fn decompress_braces_01() {
        let result = decompress_braces("ch3{ao}");
        assert_eq!(result, "chaoaoao");
    }

    #[test]
    fn decompress_braces_02() {
        let result = decompress_braces("2{y3{o}}s");
        assert_eq!(result, "yoooyooos");
    }

    #[test]
    fn decompress_braces_03() {
        let result = decompress_braces("z3{a2{xy}b}");
        assert_eq!(result, "zaxyxybaxyxybaxyxyb");
    }

    #[test]
    fn decompress_braces_04() {
        let result = decompress_braces("2{3{r4{e}r}io}");
        assert_eq!(result, "reeeerreeeerreeeerioreeeerreeeerreeeerio");
    }

    #[test]
    fn decompress_braces_05() {
        let result = decompress_braces("go3{spinn2{ing}s}");
        assert_eq!(result, "gospinningingsspinningingsspinningings");
    }

    #[test]
    fn decompress_braces_06() {
        let result = decompress_braces("2{l2{if}azu}l");
        assert_eq!(result, "lififazulififazul");
    }

    #[test]
    fn decompress_braces_07() {
        let result = decompress_braces("3{al4{ec}2{icia}}");
        assert_eq!(
            result,
            "alececececiciaiciaalececececiciaiciaalececececiciaicia"
        );
    }
}
