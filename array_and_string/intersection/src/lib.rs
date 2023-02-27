use std::{borrow::Cow, collections::HashSet};

/// The function takes in two arrays, a,b,
/// as arguments. The function should return
/// a new array containing elements that
/// are in both of the two arrays.
///
/// You may assume that each input array
/// does not contain duplicate elements.
pub fn intersection<'a>(left: &'a [u32], right: &'a [u32]) -> Vec<u32> {
    let mut result: Vec<u32> = left
        .iter()
        .collect::<HashSet<_>>()
        .intersection(&right.iter().collect())
        .map(|&&v| v)
        .collect();
    result.sort();
    result
}

/// Alternate implementation using `Cow` 🐄
/// see [here]( https://stackoverflow.com/questions/75394760/returning-an-array-slice-of-u32-from-a-function )
pub fn intersection_with_cow<'a>(
    left: &'a [u32],
    right: &'a [u32],
) -> Cow<'a, [u32]> {
    Cow::Owned(
        left.iter()
            .collect::<HashSet<_>>()
            .intersection(&right.iter().collect())
            .map(|&&v| v)
            .collect(),
    )
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

    #[test]
    fn test_intersection_with_cow() {
        let mut result =
            intersection_with_cow(&[4, 2, 1, 6], &[3, 6, 9, 2, 10]).to_vec();
        result.sort();
        assert_eq!(result, &[2, 6]);
    }
}
