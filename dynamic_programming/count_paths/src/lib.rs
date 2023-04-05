use std::collections::HashMap;

/// Takes in a grid as an argument. In the grid,
/// 'X' represents walls and 'O' represents open
/// spaces. You may only move down or to the right
/// and cannot pass through walls. The function
/// should return the number of ways possible to
/// travel from the top-left corner of the grid
/// to the bottom-right corner.
///
pub fn count_paths<const N: usize>(grid: &[[char; N]]) -> usize {
    cp_recursive(grid, 0, 0, &mut HashMap::new())
}

pub fn cp_recursive<const N: usize>(
    grid: &[[char; N]],
    row_num: usize,
    col_num: usize,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let pos = format!("{},{}", row_num, col_num);

    // If the calculated value is in cache
    // then return it
    if let Some(&value) = memo.get(&pos) {
        return value;
    }

    if row_num == grid.len() || col_num == grid[0].len() || grid[row_num][col_num] == 'X' {
        return 0;
    }

    if row_num == grid.len() - 1 && col_num == grid[0].len() - 1 {
        return 1;
    }

    let down_path = cp_recursive(grid, row_num + 1, col_num, memo);
    let right_path = cp_recursive(grid, row_num, col_num + 1, memo);

    let result = down_path + right_path;
    memo.insert(pos, result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_paths_00() {
        let grid = [['O', 'O'], ['O', 'O']];
        let result = count_paths::<2>(&grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn count_paths_01() {
        let grid = [['O', 'O', 'X'], ['O', 'O', 'O'], ['O', 'O', 'O']];
        let result = count_paths::<3>(&grid);
        assert_eq!(result, 5);
    }

    #[test]
    fn count_paths_02() {
        let grid = [['O', 'O', 'O'], ['O', 'O', 'X'], ['O', 'O', 'O']];
        let result = count_paths::<3>(&grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn count_paths_03() {
        let grid = [['O', 'O', 'O'], ['O', 'X', 'X'], ['O', 'O', 'O']];
        let result = count_paths::<3>(&grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_paths_04() {
        let grid = [
            ['O', 'O', 'X', 'O', 'O', 'O'],
            ['O', 'O', 'X', 'O', 'O', 'O'],
            ['X', 'O', 'X', 'O', 'O', 'O'],
            ['X', 'X', 'X', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O', 'O', 'O'],
        ];
        let result = count_paths::<6>(&grid);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_paths_05() {
        let grid = [
            ['O', 'O', 'X', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O', 'O', 'X'],
            ['X', 'O', 'O', 'O', 'O', 'O'],
            ['X', 'X', 'X', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O', 'O', 'O'],
        ];
        let result = count_paths::<6>(&grid);
        assert_eq!(result, 42);
    }

    #[test]
    fn count_paths_06() {
        let grid = [
            ['O', 'O', 'X', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O', 'O', 'X'],
            ['X', 'O', 'O', 'O', 'O', 'O'],
            ['X', 'X', 'X', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O', 'O', 'X'],
        ];
        let result = count_paths::<6>(&grid);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_paths_07() {
        let grid = [
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
        ];
        let result = count_paths::<15>(&grid);
        assert_eq!(result, 40116600);
    }

    #[test]
    fn count_paths_08() {
        let grid = [
            [
                'O', 'O', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'X', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'X', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'X', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'X', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'X', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'X', 'X', 'X', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'X', 'X', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'X', 'X', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
            [
                'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'O',
            ],
        ];
        let result = count_paths::<15>(&grid);
        assert_eq!(result, 3190434);
    }
}
