use std::collections::{HashSet, VecDeque};

/// A knight and a pawn are on a chess board. Can
/// you figure out the minimum number of moves
/// required for the knight to travel to the same
/// position of the pawn? On a single move, the
/// knight can move in an "L" shape; two spaces
/// in any direction, then one space in a perpendicular
/// direction. This means that on a single move, a
/// knight has eight possible positions it can move to.
///
/// Write a function, knightAttack, that takes in 5 arguments:
/// `n`, `kr`, `kc`, `pr`, `pc`
///
/// n = the length of the chess board
/// kr = the starting row of the knight
/// kc = the starting column of the knight
/// pr = the row of the pawn
/// pc = the column of the pawn
///
/// The function should return a number representing
/// the minimum number of moves required for the knight
/// to land ontop of the pawn. The knight cannot move
/// out-of-bounds of the board. You can assume that
/// rows and columns are 0-indexed. This means that
/// if n = 8, there are 8 rows and 8 columns numbered
/// 0 to 7. If it is not possible for the knight to
/// attack the pawn, then return `null`.
pub fn knight_attack(n: i32, kr: i32, kc: i32, pr: i32, pc: i32) -> Option<i32> {
    let mut visited = HashSet::new();
    visited.insert(format!("{}, {}", kr, kc));
    let mut queue = VecDeque::new();
    // insert a tuple of (knight_row, knight_col, distance)
    queue.push_back((kr, kc, 0));

    while !queue.is_empty() {
        let (row, col, step) = queue.pop_front().unwrap();
        if row == pr && col == pc {
            return Some(step);
        }
        let neighbours = get_knight_moves(n, row, col);

        for neighbour in neighbours {
            let neighbour_row = neighbour[0];
            let neighbour_col = neighbour[1];

            let neighbour_pos = format!("{},{}", neighbour_row, neighbour_col);
            if !visited.contains(&neighbour_pos) {
                visited.insert(neighbour_pos);
                queue.push_back((neighbour_row, neighbour_col, step + 1));
            }
        }
    }

    None
}

fn get_knight_moves(n: i32, row: i32, col: i32) -> Vec<[i32; 2]> {
    let positions = [
        [row + 2, col + 1],
        [row - 2, col + 1],
        [row + 2, col - 1],
        [row - 2, col - 1],
        [row + 1, col + 2],
        [row - 1, col + 2],
        [row + 1, col - 2],
        [row - 1, col - 2],
    ];

    let mut inbound_positions = vec![];

    for pos in positions {
        let new_row = pos[0];
        let new_col = pos[1];
        if new_row >= 0 && new_row < n && new_col >= 0 && new_col < n {
            inbound_positions.push(pos);
        }
    }

    inbound_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knight_attack_test_00() {
        let result = knight_attack(8, 1, 1, 2, 2);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn knight_attack_test_01() {
        let result = knight_attack(8, 1, 1, 2, 3);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn knight_attack_test_02() {
        let result = knight_attack(8, 0, 3, 4, 2);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn knight_attack_test_03() {
        let result = knight_attack(8, 0, 3, 5, 2);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn knight_attack_test_04() {
        let result = knight_attack(24, 4, 7, 19, 20);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn knight_attack_test_05() {
        let result = knight_attack(100, 21, 10, 0, 0);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn knight_attack_test_06() {
        let result = knight_attack(3, 0, 0, 1, 2);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn knight_attack_test_07() {
        let result = knight_attack(3, 0, 0, 1, 1);
        assert_eq!(result, None);
    }
}
