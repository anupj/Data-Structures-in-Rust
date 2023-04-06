pub fn subsets(elements: &[char]) -> Vec<Vec<char>> {
    if elements.is_empty() {
        return vec![vec![]];
    }

    let first = elements[0];
    let remaining_elements = &elements[1..];
    let subsets_without_first = subsets(remaining_elements);

    let mut subsets_with_first: Vec<Vec<char>> = Vec::new();
    for subset in &subsets_without_first {
        let mut new_subset = vec![first];
        new_subset.extend(subset);
        subsets_with_first.push(new_subset);
    }

    let mut result: Vec<Vec<char>> = Vec::new();
    result.extend(subsets_without_first);
    result.extend(subsets_with_first);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets_00() {
        let result = subsets(&['a', 'b']);
        let response: Vec<Vec<char>> = vec![vec![], vec!['b'], vec!['a'], vec!['a', 'b']];
        assert_eq!(result, response);
    }

    #[test]
    fn subsets_01() {
        let result = subsets(&['a', 'b', 'c']);
        let response: Vec<Vec<char>> = vec![
            vec![],
            vec!['c'],
            vec!['b'],
            vec!['b', 'c'],
            vec!['a'],
            vec!['a', 'c'],
            vec!['a', 'b'],
            vec!['a', 'b', 'c'],
        ];

        assert_eq!(result, response);
    }

    #[test]
    fn subsets_02() {
        let result = subsets(&['x']);
        let response: Vec<Vec<char>> = vec![vec![], vec!['x']];
        assert_eq!(result, response);
    }

    #[test]
    fn subsets_03() {
        let result = subsets(&[]);
        let response: Vec<Vec<char>> = vec![vec![]];
        assert_eq!(result, response);
    }

    #[test]
    fn subsets_04() {
        let result = subsets(&['q', 'r', 's', 't']);
        let response: Vec<Vec<char>> = vec![
            vec![],
            vec!['t'],
            vec!['s'],
            vec!['s', 't'],
            vec!['r'],
            vec!['r', 't'],
            vec!['r', 's'],
            vec!['r', 's', 't'],
            vec!['q'],
            vec!['q', 't'],
            vec!['q', 's'],
            vec!['q', 's', 't'],
            vec!['q', 'r'],
            vec!['q', 'r', 't'],
            vec!['q', 'r', 's'],
            vec!['q', 'r', 's', 't'],
        ];
        assert_eq!(result, response);
    }
}
