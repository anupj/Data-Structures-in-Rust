use std::{collections::HashSet, fmt::format};

// pub fn island_count<Matrix: AsRef<[Row]>, Row: AsRef<[char]>>(grid: Matrix) -> usize {
//     let mut visited: HashSet<char> = HashSet::new();
//     let mut count = 0;
//
//     for row in grid.as_ref() {
//         for col in row.as_ref() {
//             print!("{} ", col);
//         }
//         println!("");
//     }
//
//     count
// }

pub fn island_count<const N: usize>(grid: &[[char; N]]) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut count = 0;

    for row in grid.as_ref() {
        for col in row.as_ref() {
            print!("{} ", *col);
        }
        println!("");
    }

    count
}

pub fn explore<const N: usize>(
    grid: &[[char; N]],
    row_num: usize,
    col_num: usize,
    &mut visited: HashSet<String>,
) -> bool {
    let row_in_bounds = 0 <= row_num && row_num < grid.len();
    let col_in_bounds = 0 <= col_num && col_num < grid.len();

    if !row_in_bounds || !col_in_bounds {
        return false;
    }

    if grid[row_num][col_num] == 'W' {
        return false;
    }

    let pos = format!("{},{}", row_num, col_num);
    if visited.contains(&pos) {
        return false;
    }
    visited.insert(pos);

    explore(&grid, row_num - 1, col_num, &visited);
    explore(&grid, row_num + 1, col_num, &visited);
    explore(&grid, row_num, col_num - 1, &visited);
    explore(&grid, row_num, col_num + 1, &visited);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = [['a', 'b', 'c'], ['d', 'e', 'f'], ['g', 'h', 'i']];
        let result = island_count::<3>(&arr);
        assert_eq!(result, 0);
    }
}
