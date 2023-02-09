use std::collections::HashSet;

/// The function takes in two arrays, a,b,
/// as arguments. The function should return
/// a new array containing elements that
/// are in both of the two arrays.
///
/// You may assume that each input array
/// does not contain duplicate elements.
pub fn intersection<'a>(left: &'a [u32], right: &'a [u32]) -> Vec<u32> {
    let left_set: HashSet<u32> = left.iter().cloned().collect();
    let right_set: HashSet<u32> = right.iter().cloned().collect();

    let mut result: Vec<u32> =
        left_set.intersection(&right_set).into_iter().cloned().collect();
    result.sort();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(intersection(&[4, 2, 1, 6], &[3, 6, 9, 2, 10]), &[2, 6]);
        assert_eq!(intersection(&[2, 4, 6], &[4, 2]), &[2, 4]);
        assert_eq!(intersection(&[4, 2, 1], &[1, 2, 4, 6]), &[1, 2, 4]);
        assert_eq!(intersection(&[0, 1, 2], &[10, 11]), &[]);
    }

    #[test]
    fn test_intersection_large() {
        let a: &mut [u32; 50000] = &mut [0; 50000];
        let b: &mut [u32; 50000] = &mut [0; 50000];
        let mut c: Vec<u32> = Vec::new();
        for i in 0..50000 {
            a[i] = i as u32;
            b[i] = i as u32;
            c.push(i as u32);
        }
        assert_eq!(intersection(a, b), c);
    }
}
