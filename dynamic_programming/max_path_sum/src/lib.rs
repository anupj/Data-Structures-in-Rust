use std::collections::HashMap;

/// Takes in a grid as an argument. In the grid,
/// 'X' represents walls and 'O' represents open
/// spaces. You may only move down or to the right
/// and cannot pass through walls. The function
/// should return the number of ways possible to
/// travel from the top-left corner of the grid
/// to the bottom-right corner.
///
pub fn max_path_sum<const N: usize>(grid: &[[usize; N]]) -> usize {
    mps_recursive(grid, 0, 0, &mut HashMap::new())
}

pub fn mps_recursive<const N: usize>(
    grid: &[[usize; N]],
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

    if row_num == grid.len() || col_num == grid[0].len() {
        return usize::MIN;
    }

    if row_num == grid.len() - 1 && col_num == grid[0].len() - 1 {
        return grid[row_num][col_num];
    }

    let down_path = mps_recursive(grid, row_num + 1, col_num, memo);
    let right_path = mps_recursive(grid, row_num, col_num + 1, memo);

    let result = grid[row_num][col_num] + down_path.max(right_path);
    memo.insert(pos, result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_paths_00() {
        let grid = [[1, 3, 12], [5, 1, 1], [3, 6, 1]];
        let result = max_path_sum::<3>(&grid);
        assert_eq!(result, 18);
    }

    #[test]
    fn count_paths_01() {
        let grid = [[1, 2, 8, 1], [3, 1, 12, 10], [4, 0, 6, 3]];
        let result = max_path_sum::<4>(&grid);
        assert_eq!(result, 36);
    }

    #[test]
    fn count_paths_02() {
        let grid = [[1, 2, 8, 1], [3, 10, 12, 10], [4, 0, 6, 3]];
        let result = max_path_sum::<4>(&grid);
        assert_eq!(result, 39);
    }

    #[test]
    fn count_paths_03() {
        let grid = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        let result = max_path_sum::<13>(&grid);
        assert_eq!(result, 27);
    }

    #[test]
    fn count_paths_04() {
        let grid = [
            [1, 1, 3, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 2, 1, 1, 6, 1, 1, 5, 1, 1, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 5, 1, 1, 1, 1, 0, 1, 1, 1, 1],
            [2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [2, 1, 1, 1, 1, 8, 1, 1, 1, 1, 1, 1, 1],
            [2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 9, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 8, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 42, 1, 1, 1, 1, 1, 1, 1, 8, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        let result = max_path_sum::<13>(&grid);
        assert_eq!(result, 82);
    }
}
