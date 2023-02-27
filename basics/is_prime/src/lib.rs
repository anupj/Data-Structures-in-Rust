use std::usize;

/// Primes are integers greater than one
/// with no positive divisors besides one
/// and itself
/// Time complexity: O(sqrt(n))
/// Space: O(1)
pub fn is_prime(num: usize) -> bool {
    if num < 2 {
        return false;
    };

    let sqrt_num = f64::sqrt(num as f64) as usize;

    for i in 2..=sqrt_num {
        if num % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(25), false);
        assert_eq!(is_prime(31), true);
        assert_eq!(is_prime(2017), true);
        assert_eq!(is_prime(2048), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(713), false);
    }
}
