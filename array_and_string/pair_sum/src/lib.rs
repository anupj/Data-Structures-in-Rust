use std::collections::HashMap;

/// PairSum takes in an array and a target
/// sum as arguments. The function should
/// return an array containing a pair of
/// indices whose elements sum to the given
/// target. The indices returned must be unique.
/// Be sure to return the indices, not the
/// elements themselves.
///
/// There is guaranteed to be one such pair
/// that sums to the target.
/// Complexity:
/// T: O(n)
/// S: O(n)
pub fn pair_sum(num_list: &[u32], target: u32) -> [u32; 2] {
    // Stores the indices of the
    // matching elements
    let mut result: [u32; 2] = [0; 2];

    // Stores the index of the numbers
    // that we've covered so far
    // key: num ; value: index
    let mut prev_num_index_map: HashMap<u32, u32> = HashMap::new();

    for (index, num) in num_list.iter().enumerate() {
        // if the num in array is greater
        // than the target then
        // obvs its not part of its "sum"
        if *num > target {
            continue;
        }

        // stores the value of
        // target minus(-) num_list[i]
        let complement = target - *num;
        if let Some(val) = prev_num_index_map.get(&complement) {
            result[0] = *val;
            result[1] = index as u32;
            return result;
        }
        prev_num_index_map.insert(*num, index as u32);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_sum() {
        assert_eq!(pair_sum(&[3, 2, 5, 4, 1], 8), [0, 2]);
        assert_eq!(pair_sum(&[4, 7, 9, 2, 5, 1], 5), [0, 5]);
        assert_eq!(pair_sum(&[4, 7, 9, 2, 5, 1], 3), [3, 5]);
        assert_eq!(pair_sum(&[1, 6, 7, 2], 13), [1, 2]);
        assert_eq!(pair_sum(&[9, 9], 18), [0, 1]);
        assert_eq!(pair_sum(&[6, 4, 2, 8], 12), [1, 3]);
    }
}
