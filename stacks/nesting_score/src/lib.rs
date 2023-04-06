/// Takes in a string of brackets as an argument. The
/// function should return the score of the string
/// according to the following rules:
///
/// [] is worth 1 point
/// XY is worth m + n points where X, Y are substrings
/// of well-formed brackets and m, n are their respective
/// scores
/// [S] is worth 2 * k points where S is a substring of
/// well-formed brackets and k is the score of that substring
/// You may assume that the input only contains
/// well-formed square brackets.
pub fn nesting_score(s: &str) -> usize {
    let mut stack: Vec<usize> = vec![0];

    for c in s.chars() {
        match c {
            '[' => {
                stack.push(0);
            }
            ']' => {
                let popped = stack.pop().unwrap();
                if popped == 0 {
                    if let Some(last_element) = stack.last_mut() {
                        *last_element += 1;
                    }
                } else if let Some(last_element) = stack.last_mut() {
                    *last_element += 2 * popped;
                }
            }
            _ => {}
        }
    }
    stack[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nesting_score_00() {
        let result = nesting_score("[]");
        assert_eq!(result, 1);
    }

    #[test]
    fn nesting_score_01() {
        let result = nesting_score("[][][]");
        assert_eq!(result, 3);
    }

    #[test]
    fn nesting_score_02() {
        let result = nesting_score("[[]]");
        assert_eq!(result, 2);
    }

    #[test]
    fn nesting_score_03() {
        let result = nesting_score("[[][]]");
        assert_eq!(result, 4);
    }

    #[test]
    fn nesting_score_04() {
        let result = nesting_score("[[][][]]");
        assert_eq!(result, 6);
    }

    #[test]
    fn nesting_score_05() {
        let result = nesting_score("[[][]][]");
        assert_eq!(result, 5);
    }

    #[test]
    fn nesting_score_06() {
        let result = nesting_score("[][[][]][[]]");
        assert_eq!(result, 7);
    }

    #[test]
    fn nesting_score_07() {
        let result = nesting_score("[[[[[[[][]]]]]]][]");
        assert_eq!(result, 129);
    }
}
