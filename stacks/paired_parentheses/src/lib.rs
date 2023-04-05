/// Takes in a string as an argument. The function
/// should return a boolean indicating whether or not
/// the string has well-formed parentheses.
///
/// You may assume the string contains only alphabetic
/// characters, '(', or ')'.
pub fn paired_parentheses(s: &str) -> bool {
    let mut count = 0;

    for ch in s.chars() {
        if ch == '(' {
            count += 1;
        } else if ch == ')' {
            if count == 0 {
                return false;
            }
            count -= 1;
        }
    }

    count == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_parentheses_00() {
        let result = paired_parentheses("(david)((abby))");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_01() {
        let result = paired_parentheses("()rose(jeff");
        assert_eq!(result, false);
    }

    #[test]
    fn paired_parentheses_02() {
        let result = paired_parentheses(")(");
        assert_eq!(result, false);
    }

    #[test]
    fn paired_parentheses_03() {
        let result = paired_parentheses("()");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_04() {
        let result = paired_parentheses("(((potato())))");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_05() {
        let result = paired_parentheses("(())(water)()");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_06() {
        let result = paired_parentheses("(())(water()()");
        assert_eq!(result, false);
    }

    #[test]
    fn paired_parentheses_07() {
        let result = paired_parentheses("");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_08() {
        let result = paired_parentheses("))()");
        assert_eq!(result, false);
    }
}
