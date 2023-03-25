use std::collections::{HashSet, VecDeque};

/// This function takes in a grid as an argument. The grid
/// contains water (W) and land (L). There are exactly two
/// islands in the grid. An island is a vertically or
/// horizontally connected region of land. Return the
/// minimum length bridge needed to connect the two
/// islands. A bridge does not need to form a straight line.
pub fn minimum_length_bridge<const N: usize>(grid: &[[char; N]]) -> isize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut main_island: HashSet<String> = HashSet::new();

    'outer: for (row_num, row) in grid.iter().enumerate() {
        for (col_num, _) in row.iter().enumerate() {
            let possible_island = traverse_island(
                grid,
                row_num as isize,
                col_num as isize,
                &mut visited,
            );

            if !possible_island.is_empty() {
                main_island = visited.clone();
                break 'outer;
            }
        }
    }

    let mut queue = VecDeque::new();
    for pos in &visited {
        let split_values: Vec<_> = pos
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();
        queue.push_back((
            split_values[0] as isize,
            split_values[1] as isize,
            0isize,
        ));
    }

    while !queue.is_empty() {
        let (row_num, col_num, distance) = queue.pop_front().unwrap();
        let pos = format!("{},{}", row_num, col_num);
        if grid[row_num as usize][col_num as usize] == 'L'
            && !main_island.contains(&pos)
        {
            return distance - 1;
        }
        let deltas = [[1, 0], [-1, 0], [0, 1], [0, -1]];

        for delta in deltas {
            let delta_row = delta[0];
            let delta_col = delta[1];
            let neighbour_row = row_num + delta_row;
            let neighbour_col = col_num + delta_col;
            let neighbour_pos = format!("{},{}", neighbour_row, neighbour_col);

            if is_inbounds(grid, neighbour_row, neighbour_col)
                && !visited.contains(&neighbour_pos)
            {
                visited.insert(neighbour_pos);
                queue.push_back((neighbour_row, neighbour_col, distance + 1));
            }
        }
    }
    0
}

/// Check if the `row` or `col`
/// is within the bounds of the `grid`
pub fn is_inbounds<const N: usize>(
    grid: &[[char; N]],
    row_num: isize,
    col_num: isize,
) -> bool {
    let row_in_bounds = { row_num >= 0 && row_num < grid.len() as isize };
    let col_in_bounds = { col_num >= 0 && col_num < grid[0].len() as isize };
    row_in_bounds && col_in_bounds
}

/// This function is called recursively to
/// track the land
pub fn traverse_island<'a, const N: usize>(
    grid: &[[char; N]],
    row_num: isize,
    col_num: isize,
    visited: &'a mut HashSet<String>,
) -> &'a HashSet<String> {
    // We only want to proceed if
    // `row` and `col` are inbound or
    // the grid cell is `L`
    if !is_inbounds(grid, row_num, col_num)
        || grid[row_num as usize][col_num as usize] == 'W'
    {
        return visited;
    }

    // Track the visited grid cell location as `row,col`
    let pos = format!("{},{}", row_num, col_num);
    if visited.contains(&pos) {
        // we have already visited this location
        // so return
        return visited;
    }
    visited.insert(pos);

    traverse_island(grid, row_num - 1, col_num, visited);
    traverse_island(grid, row_num + 1, col_num, visited);
    traverse_island(grid, row_num, col_num - 1, visited);
    traverse_island(grid, row_num, col_num + 1, visited);

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_island_count_test_00() {
        let grid = [
            ['W', 'W', 'W', 'L', 'L'],
            ['L', 'L', 'W', 'W', 'L'],
            ['L', 'L', 'L', 'W', 'L'],
            ['W', 'L', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W'],
        ];
        let result = minimum_length_bridge::<5>(&grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn min_island_count_test_01() {
        let grid = [
            ['W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W'],
            ['L', 'L', 'W', 'W', 'L'],
            ['W', 'L', 'W', 'W', 'L'],
            ['W', 'W', 'W', 'L', 'L'],
            ['W', 'W', 'W', 'W', 'W'],
        ];
        let result = minimum_length_bridge::<5>(&grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn min_island_count_test_02() {
        let grid = [
            ['W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'L', 'W'],
            ['L', 'W', 'W', 'W', 'W'],
        ];
        let result = minimum_length_bridge::<5>(&grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn min_island_count_test_03() {
        let grid = [
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'L', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'L', 'L', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'L', 'L', 'L', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'L', 'L', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'L', 'L', 'L'],
            ['L', 'L', 'L', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
        ];
        let result = minimum_length_bridge::<8>(&grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn min_island_count_test_04() {
        let grid = [
            ['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'L', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
            ['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
        ];
        let result = minimum_length_bridge::<8>(&grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn min_island_count_test_05() {
        let grid = [
            ['W', 'L', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'L', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'L', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'L', 'L'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'L'],
        ];
        let result = minimum_length_bridge::<8>(&grid);
        assert_eq!(result, 8);
    }
}
