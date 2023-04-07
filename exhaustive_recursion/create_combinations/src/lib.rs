/// Takes in an array and a length as arguments. The
/// function should return a 2D array representing
/// all of the combinations of the specifized length.
///
/// The items within the combinations and the
/// combinations themselves may be returned in any
/// order.
/// You may assume that the input array contains unique elements and 1 <= k <= items.length.
pub fn create_combinations(items: &[char], k: usize) -> Vec<Vec<char>> {
    if items.len() < k {
        return Vec::new();
    }

    if k == 0 {
        return vec![vec![]];
    }

    let first = items[0];
    let remaining_items = &items[1..];

    let mut combos_with_first: Vec<Vec<char>> = Vec::new();
    for combo in create_combinations(remaining_items, k - 1) {
        let mut new_combo = vec![first];
        new_combo.extend(combo);
        combos_with_first.push(new_combo);
    }

    let combos_without_first = create_combinations(remaining_items, k);
    let mut result = combos_with_first;
    result.extend(combos_without_first);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets_00() {
        let result = create_combinations(&['a', 'b', 'c'], 2);
        let response: Vec<Vec<char>> = vec![vec!['a', 'b'], vec!['a', 'c'], vec!['b', 'c']];

        assert_eq!(result, response);
    }

    #[test]
    fn subsets_01() {
        let result = create_combinations(&['q', 'r', 's', 't'], 2);
        let response: Vec<Vec<char>> = vec![
            vec!['q', 'r'],
            vec!['q', 's'],
            vec!['q', 't'],
            vec!['r', 's'],
            vec!['r', 't'],
            vec!['s', 't'],
        ];

        assert_eq!(result, response);
    }

    #[test]
    fn subsets_02() {
        let result = create_combinations(&['q', 'r', 's', 't'], 3);
        let response: Vec<Vec<char>> = vec![
            vec!['q', 'r', 's'],
            vec!['q', 'r', 't'],
            vec!['q', 's', 't'],
            vec!['r', 's', 't'],
        ];

        assert_eq!(result, response);
    }

    #[test]
    fn subsets_03() {
        let result = create_combinations(&['a', 'b', 'c'], 3);
        let response: Vec<Vec<char>> = vec![vec!['a', 'b', 'c']];
        assert_eq!(result, response);
    }
}
