use std::collections::HashSet;

/// This function takes in a grid containing Ws and Ls. W
/// represents water and L represents land. The function should
/// returns the size of the smallest island. An island is a
/// vertically or horizontally connected region of land.
pub fn minimum_island_count<const N: usize>(grid: &[[char; N]]) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut min_size = usize::MAX;

    for (row_num, row) in grid.iter().enumerate() {
        for (col_num, _) in row.iter().enumerate() {
            let size = explore(
                grid,
                row_num as isize,
                col_num as isize,
                &mut visited,
            );
            if size > 0 && size < min_size {
                min_size = size;
            }
        }
    }

    min_size
}

/// This function is called recursively to
/// track the land
pub fn explore<const N: usize>(
    grid: &[[char; N]],
    row_num: isize,
    col_num: isize,
    visited: &mut HashSet<String>,
) -> usize {
    // This check is required because you
    // could call this function recursively with
    // -1 or grid.len() value for row or column.
    let row_in_bounds = { row_num >= 0 && row_num < grid.len() as isize };
    let col_in_bounds = { col_num >= 0 && col_num < grid[0].len() as isize };

    // if the row or col num value is
    // out of bounds return false
    if !row_in_bounds || !col_in_bounds {
        return 0;
    }

    // We only want to proceed to true if
    // the grid cell is `L`
    if grid[row_num as usize][col_num as usize] == 'W' {
        return 0;
    }

    // Track the visited grid cell location as `row,col`
    let pos = format!("{},{}", row_num, col_num);
    if visited.contains(&pos) {
        // we have already visited this location
        // so return
        return 0;
    }
    visited.insert(pos);

    let mut size = 1;

    size += explore(grid, row_num - 1, col_num, visited);
    size += explore(grid, row_num + 1, col_num, visited);
    size += explore(grid, row_num, col_num - 1, visited);
    size += explore(grid, row_num, col_num + 1, visited);

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_island_count_test_00() {
        let grid = [
            ['W', 'L', 'W', 'W', 'W'],
            ['W', 'L', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'L', 'W'],
            ['W', 'W', 'L', 'L', 'W'],
            ['L', 'W', 'W', 'L', 'L'],
            ['L', 'L', 'W', 'W', 'W'],
        ];
        let result = minimum_island_count::<5>(&grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn min_island_count_test_01() {
        let grid = [
            ['L', 'W', 'W', 'L', 'W'],
            ['L', 'W', 'W', 'L', 'L'],
            ['W', 'L', 'W', 'L', 'W'],
            ['W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'L', 'L', 'L'],
        ];
        let result = minimum_island_count::<5>(&grid);
        assert_eq!(result, 1);
    }
    #[test]
    fn min_island_count_test_02() {
        let grid = [['L', 'L', 'L'], ['L', 'L', 'L'], ['L', 'L', 'L']];
        let result = minimum_island_count::<3>(&grid);
        assert_eq!(result, 9);
    }
    #[test]
    fn min_island_count_test_03() {
        let grid = [['W', 'W'], ['W', 'W'], ['W', 'W']];
        let result = minimum_island_count::<2>(&grid);
        assert_eq!(result, usize::MAX);
    }

    #[test]
    fn min_island_count_test_04() {
        let grid = [['W', 'W'], ['L', 'L'], ['W', 'W'], ['W', 'L']];
        let result = minimum_island_count::<2>(&grid);
        assert_eq!(result, 1);
    }
}
