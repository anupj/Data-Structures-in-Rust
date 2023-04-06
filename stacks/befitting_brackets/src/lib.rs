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
    fn befitting_brackets_00() {
        let result = befitting_brackets("(){}[](())");
        assert_eq!(result, true);
    }

    #[test]
    fn befitting_brackets_01() {
        let result = befitting_brackets("({[]})");
        assert_eq!(result, true);
    }

    #[test]
    fn befitting_brackets_02() {
        let result = befitting_brackets("[][}");
        assert_eq!(result, false);
    }

    #[test]
    fn befitting_brackets_03() {
        let result = befitting_brackets("{[]}({}");
        assert_eq!(result, false);
    }

    #[test]
    fn befitting_brackets_04() {
        let result = befitting_brackets("[]{}(}[]");
        assert_eq!(result, false);
    }

    #[test]
    fn befitting_brackets_05() {
        let result = befitting_brackets("[]{}()[]");
        assert_eq!(result, true);
    }

    #[test]
    fn befitting_brackets_06() {
        let result = befitting_brackets("]{}");
        assert_eq!(result, false);
    }

    #[test]
    fn befitting_brackets_07() {
        let result = befitting_brackets("");
        assert_eq!(result, true);
    }

    #[test]
    fn befitting_brackets_08() {
        let result = befitting_brackets("{[(}])");
        assert_eq!(result, false);
    }
}
