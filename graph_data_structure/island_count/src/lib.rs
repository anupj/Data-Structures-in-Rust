use std::collections::HashSet;

pub fn island_count<const N: usize>(grid: &[[char; N]]) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut count = 0;

    for (row_num, row) in grid.iter().enumerate() {
        for (col_num, _) in row.iter().enumerate() {
            if explore(grid, row_num as isize, col_num as isize, &mut visited)
            {
                count += 1;
            }
        }
    }

    count
}

pub fn explore<const N: usize>(
    grid: &[[char; N]],
    row_num: isize,
    col_num: isize,
    visited: &mut HashSet<String>,
) -> bool {
    let row_in_bounds = { row_num >= 0 && row_num < grid.len() as isize };
    let col_in_bounds = { col_num >= 0 && col_num < grid[0].len() as isize };

    if !row_in_bounds || !col_in_bounds {
        return false;
    }

    if grid[row_num as usize][col_num as usize] == 'W' {
        return false;
    }

    let pos = format!("{},{}", row_num, col_num);
    if visited.contains(&pos) {
        return false;
    }
    visited.insert(pos);

    explore(grid, row_num - 1, col_num, visited);
    explore(grid, row_num + 1, col_num, visited);
    explore(grid, row_num, col_num - 1, visited);
    explore(grid, row_num, col_num + 1, visited);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn island_count_test_00() {
        let grid = [
            ['W', 'L', 'W', 'W', 'W'],
            ['W', 'L', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'L', 'W'],
            ['W', 'W', 'L', 'L', 'W'],
            ['L', 'W', 'W', 'L', 'L'],
            ['L', 'L', 'W', 'W', 'W'],
        ];
        let result = island_count::<5>(&grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn island_count_test_01() {
        let grid = [
            ['L', 'W', 'W', 'L', 'W'],
            ['L', 'W', 'W', 'L', 'L'],
            ['W', 'L', 'W', 'L', 'W'],
            ['W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'L', 'L', 'L'],
        ];
        let result = island_count::<5>(&grid);
        assert_eq!(result, 4);
    }
    #[test]
    fn island_count_test_02() {
        let grid = [['L', 'L', 'L'], ['L', 'L', 'L'], ['L', 'L', 'L']];
        let result = island_count::<3>(&grid);
        assert_eq!(result, 1);
    }
    #[test]
    fn island_count_test_03() {
        let grid = [['W', 'W'], ['W', 'W'], ['W', 'W']];
        let result = island_count::<2>(&grid);
        assert_eq!(result, 0);
    }
}
