use std::collections::HashMap;

/// PairProduct takes in an array and a target
/// product as arguments. The function should
/// return an array containing a pair of
/// indices whose elements multiply to the given
/// target. The indices returned must be unique.
/// Be sure to return the indices, not the
/// elements themselves.
///
/// There is guaranteed to be one such pair
/// whose product is the target.
///
/// Complexity:
/// T: O(n)
/// S: O(n)
pub fn pair_product(num_list: &[u32], target: u32) -> [u32; 2] {
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
        // obvs its not part of its "product"
        if *num > target {
            continue;
        }

        // stores the value of
        // target divided by(/) num_list[i]
        let complement = target / *num;
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
    fn test_pair_product() {
        assert_eq!(pair_product(&[3, 2, 5, 4, 1], 8), [1, 3]);
        assert_eq!(pair_product(&[3, 2, 5, 4, 1], 10), [1, 2]);
        assert_eq!(pair_product(&[4, 7, 9, 2, 5, 1], 5), [4, 5]);
        assert_eq!(pair_product(&[4, 7, 9, 2, 5, 1], 35), [1, 4]);
        assert_eq!(pair_product(&[4, 6, 8, 2], 16), [2, 3]);
    }
}
