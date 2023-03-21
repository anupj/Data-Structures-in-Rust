use std::collections::{HashSet, VecDeque};

/// This function takes in a grid, a starting row, and a
/// starting column. In the grid, 'X's are walls,
/// 'O's are open spaces, and 'C's are carrots. The
/// function should return a number representing the length
/// of the shortest path from the starting position to
/// a carrot. You may move up, down, left, or right,
/// but cannot pass through walls (X). If there is
/// no possible path to a carrot, then return -1.
pub fn closest_carrot<const N: usize>(
    grid: &[[char; N]],
    start_row: isize,
    start_col: isize,
) -> isize {
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col, 0));

    let mut visited: HashSet<String> = HashSet::new();

    let pos = format!("{},{}", start_row, start_col);
    visited.insert(pos);

    while !queue.is_empty() {
        let (row, col, distance) = queue.pop_front().unwrap();

        if grid[row as usize][col as usize] == 'C' {
            return distance;
        }

        let deltas = [[1, 0], [-1, 0], [0, 1], [0, -1]];

        for delta in deltas {
            let delta_row = delta[0];
            let delta_col = delta[1];
            let neighbour_row = row + delta_row;
            let neighbour_col = col + delta_col;
            let neighbour_pos = format!("{},{}", neighbour_row, neighbour_col);
            let row_in_bounds =
                { neighbour_row >= 0 && neighbour_row < grid.len() as isize };
            let col_in_bounds = {
                neighbour_col >= 0 && neighbour_col < grid[0].len() as isize
            };

            if row_in_bounds
                && col_in_bounds
                && !visited.contains(&neighbour_pos)
                && grid[neighbour_row as usize][neighbour_col as usize] != 'X'
            {
                visited.insert(neighbour_pos);
                queue.push_back((neighbour_row, neighbour_col, distance + 1));
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn island_count_test_00() {
        let grid = [
            ['O', 'O', 'O', 'O', 'O'],
            ['O', 'X', 'O', 'O', 'O'],
            ['O', 'X', 'X', 'O', 'O'],
            ['O', 'X', 'C', 'O', 'O'],
            ['O', 'X', 'X', 'O', 'O'],
            ['C', 'O', 'O', 'O', 'O'],
        ];
        let result = closest_carrot::<5>(&grid, 1, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn island_count_test_01() {
        let grid = [
            ['O', 'O', 'O', 'O', 'O'],
            ['O', 'X', 'O', 'O', 'O'],
            ['O', 'X', 'X', 'O', 'O'],
            ['O', 'X', 'C', 'O', 'O'],
            ['O', 'X', 'X', 'O', 'O'],
            ['C', 'O', 'O', 'O', 'O'],
        ];
        let result = closest_carrot::<5>(&grid, 0, 0);
        assert_eq!(result, 5);
    }

    #[test]
    fn island_count_test_02() {
        let grid = [
            ['O', 'O', 'X', 'X', 'X'],
            ['O', 'X', 'X', 'X', 'C'],
            ['O', 'X', 'O', 'X', 'X'],
            ['O', 'O', 'O', 'O', 'O'],
            ['O', 'X', 'X', 'X', 'X'],
            ['O', 'O', 'O', 'O', 'O'],
            ['O', 'O', 'C', 'O', 'O'],
            ['O', 'O', 'O', 'O', 'O'],
        ];
        let result = closest_carrot::<5>(&grid, 3, 4);
        assert_eq!(result, 9);
    }

    #[test]
    fn island_count_test_03() {
        let grid = [
            ['O', 'O', 'X', 'O', 'O'],
            ['O', 'X', 'X', 'X', 'O'],
            ['O', 'X', 'C', 'C', 'O'],
        ];
        let result = closest_carrot::<5>(&grid, 1, 4);
        assert_eq!(result, 2);
    }

    #[test]
    fn island_count_test_04() {
        let grid = [
            ['O', 'O', 'X', 'O', 'O'],
            ['O', 'X', 'X', 'X', 'O'],
            ['O', 'X', 'C', 'C', 'O'],
        ];
        let result = closest_carrot::<5>(&grid, 2, 0);
        assert_eq!(result, -1);
    }
}
