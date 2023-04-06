/// Takes  in an array an argument. The function
/// should return a 2D array where each subarray
/// represents one of the possible permutations of
/// the array.
///
/// The subarrays may be returned in any order.
///
/// You may assume that the input array contains
/// unique elements.
pub fn permutations(items: &[char]) -> Vec<Vec<char>> {
    if items.is_empty() {
        return vec![vec![]];
    }

    let first = items[0];
    let remaining_items = &items[1..];
    let partial_perms = permutations(remaining_items);

    let mut full_perms: Vec<Vec<char>> = Vec::new();
    for perm in partial_perms {
        for i in 0..=perm.len() {
            let mut new_perm = perm[..i].to_vec();
            new_perm.push(first);
            new_perm.extend_from_slice(&perm[i..]);
            full_perms.push(new_perm);
        }
    }

    full_perms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets_00() {
        let result = permutations(&['a', 'b', 'c']);
        let response: Vec<Vec<char>> = vec![
            vec!['a', 'b', 'c'],
            vec!['b', 'a', 'c'],
            vec!['b', 'c', 'a'],
            vec!['a', 'c', 'b'],
            vec!['c', 'a', 'b'],
            vec!['c', 'b', 'a'],
        ];

        assert_eq!(result, response);
    }

    #[test]
    fn subsets_01() {
        let result = permutations(&['r', 'b']);
        let response: Vec<Vec<char>> = vec![vec!['r', 'b'], vec!['b', 'r']];

        assert_eq!(result, response);
    }

    #[test]
    fn subsets_02() {
        let result = permutations(&['8', '2', '1', '4']);
        let response: Vec<Vec<char>> = vec![
            vec!['8', '2', '1', '4'],
            vec!['2', '8', '1', '4'],
            vec!['2', '1', '8', '4'],
            vec!['2', '1', '4', '8'],
            vec!['8', '1', '2', '4'],
            vec!['1', '8', '2', '4'],
            vec!['1', '2', '8', '4'],
            vec!['1', '2', '4', '8'],
            vec!['8', '1', '4', '2'],
            vec!['1', '8', '4', '2'],
            vec!['1', '4', '8', '2'],
            vec!['1', '4', '2', '8'],
            vec!['8', '2', '4', '1'],
            vec!['2', '8', '4', '1'],
            vec!['2', '4', '8', '1'],
            vec!['2', '4', '1', '8'],
            vec!['8', '4', '2', '1'],
            vec!['4', '8', '2', '1'],
            vec!['4', '2', '8', '1'],
            vec!['4', '2', '1', '8'],
            vec!['8', '4', '1', '2'],
            vec!['4', '8', '1', '2'],
            vec!['4', '1', '8', '2'],
            vec!['4', '1', '2', '8'],
        ];

        assert_eq!(result, response);
    }

    #[test]
    fn subsets_03() {
        let result = permutations(&[]);
        let response: Vec<Vec<char>> = vec![vec![]];
        assert_eq!(result, response);
    }
}
