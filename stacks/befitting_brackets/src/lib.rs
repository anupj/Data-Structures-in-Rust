/// That takes in a string as an argument. The
/// function should return a boolean indicating whether
/// or not the string contains correctly matched brackets.
///
/// You may assume the string contains only characters: ( ) [ ] { }
pub fn befitting_brackets(s: &str) -> bool {
    let mut stack = Vec::new();

    let brackets = vec![('(', ')'), ('[', ']'), ('{', '}')];

    for c in s.chars() {
        if let Some(&(_, closing_bracket)) = brackets
            .iter()
            .find(|&&(opening_bracket, _)| opening_bracket == c)
        {
            // If you find the opening bracket
            // then push the closing bracket on
            // the stack
            stack.push(closing_bracket);
        } else {
            // If `c` is closing bracket then
            // `pop` the last item from the
            // stack to see if its a match
            // If its not then return false
            match stack.pop() {
                Some(last) if last == c => (),
                _ => return false,
            }
        }
    }

    // If the stack is empty means
    // we correctly matched all brackets
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_parentheses_00() {
        let result = befitting_brackets("(){}[](())");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_01() {
        let result = befitting_brackets("({[]})");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_02() {
        let result = befitting_brackets("[][}");
        assert_eq!(result, false);
    }

    #[test]
    fn paired_parentheses_03() {
        let result = befitting_brackets("{[]}({}");
        assert_eq!(result, false);
    }

    #[test]
    fn paired_parentheses_04() {
        let result = befitting_brackets("[]{}(}[]");
        assert_eq!(result, false);
    }

    #[test]
    fn paired_parentheses_05() {
        let result = befitting_brackets("[]{}()[]");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_06() {
        let result = befitting_brackets("]{}");
        assert_eq!(result, false);
    }

    #[test]
    fn paired_parentheses_07() {
        let result = befitting_brackets("");
        assert_eq!(result, true);
    }

    #[test]
    fn paired_parentheses_08() {
        let result = befitting_brackets("{[(}])");
        assert_eq!(result, false);
    }
}
