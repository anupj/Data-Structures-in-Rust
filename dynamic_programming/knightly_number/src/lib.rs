use std::collections::HashMap;

/// A knight is on a chess board. Can you figure out
/// the total number of ways the knight can move to a
/// target position in exactly m moves? On a single
/// move, the knight can move in an "L" shape; two
/// spaces in any direction, then one space in a
/// perpendicular direction. This means that on a
/// single move, a knight has eight possible positions
/// it can move to.
///
/// Write a function, knightlyNumber, that takes in 6 arguments:
///
/// n, m, kr, kc, pr, pc
///
/// n = the length of the chess board
/// m = the number of moves that must be used
/// kr = the starting row of the knight
/// kc = the starting column of the knight
/// pr = the target row
/// pc = the target column
/// The function should return the number of different
/// ways the knight can move to the target in exactly
/// m moves. The knight can revisit positions of the
/// board if needed. The knight cannot move out-of-bounds
/// of the board. You can assume that rows and columns
/// are 0-indexed. This means that if n = 8, there are
/// 8 rows and 8 columns numbered 0 to 7.
pub fn knightly_number(n: isize, m: isize, kr: isize, kc: isize, pr: isize, pc: isize) -> usize {
    knightly_number_recursive(n, m, kr, kc, pr, pc, &mut HashMap::new()) as usize
}

pub fn knightly_number_recursive(
    n: isize,
    m: isize,
    kr: isize,
    kc: isize,
    pr: isize,
    pc: isize,
    memo: &mut HashMap<String, isize>,
) -> isize {
    let key = format!("{},{},{}", m, kr, kc);
    if let Some(&value) = memo.get(&key) {
        return value;
    }

    if kr < 0 || kr >= n || kc < 0 || kc >= n {
        return 0;
    }

    if m == 0 {
        if kr == pr && kc == pc {
            return 1;
        } else {
            return 0;
        }
    }

    let neighbours = [
        [kr + 2, kc + 1],
        [kr - 2, kc + 1],
        [kr + 2, kc - 1],
        [kr - 2, kc - 1],
        [kr + 1, kc + 2],
        [kr - 1, kc + 2],
        [kr + 1, kc - 2],
        [kr - 1, kc - 2],
    ];

    let mut count = 0;
    for neighbour in neighbours {
        let neighbour_row = neighbour[0];
        let neighbour_col = neighbour[1];
        count += knightly_number_recursive(n, m - 1, neighbour_row, neighbour_col, pr, pc, memo);
    }
    memo.insert(key, count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knightly_number_00() {
        let result = knightly_number(8, 2, 4, 4, 5, 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn knightly_number_01() {
        let result = knightly_number(8, 2, 7, 1, 7, 1);
        assert_eq!(result, 3);
    }

    #[test]
    fn knightly_number_02() {
        let result = knightly_number(8, 2, 5, 4, 5, 4);
        assert_eq!(result, 8);
    }

    #[test]
    fn knightly_number_03() {
        let result = knightly_number(8, 3, 5, 2, 4, 4);
        assert_eq!(result, 21);
    }

    #[test]
    fn knightly_number_04() {
        let result = knightly_number(20, 6, 18, 7, 10, 15);
        assert_eq!(result, 60);
    }

    #[test]
    fn knightly_number_05() {
        let result = knightly_number(20, 12, 8, 3, 9, 14);
        assert_eq!(result, 98410127);
    }

    #[test]
    fn knightly_number_06() {
        let result = knightly_number(8, 2, 0, 0, 1, 1);
        assert_eq!(result, 0);
    }
}
