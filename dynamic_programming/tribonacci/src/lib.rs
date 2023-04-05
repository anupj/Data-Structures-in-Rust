use std::collections::HashMap;

/// Write a function `tribonacci` that takes in a number argument, n,
/// and returns the n-th number of the Tribonacci sequence.
/// The 0-th and 1-st number of the sequence is 0.
/// The 2-nd number of the sequence is 1.
/// To generate further numbers of the sequence, calculate
/// the sum of previous three numbers.
/// Solve this recursively using memoization.
pub fn tribonacci(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    // Using a `memo` `HashMap` to
    // store and retrieve a calculated value;
    // we avoid redoing unnecessary recursive calls
    // to values we have already visited.
    if memo.contains_key(&n) {
        return memo[&n];
    }

    if n == 0 || n == 1 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    let fib_n_minus_1 = tribonacci(n - 1, memo);
    let fib_n_minus_2 = tribonacci(n - 2, memo);
    let fib_n_minus_3 = tribonacci(n - 3, memo);
    memo.insert(n, fib_n_minus_1 + fib_n_minus_2 + fib_n_minus_3);

    memo[&n]
}

/// Here's a more "idiomatic" way of writing
/// the same function.
/// I'll use `cache` instead of `memo`
pub fn trib_recursive(n: usize, cache: &mut Vec<Option<usize>>) -> usize {
    // If the cache already contains the value
    // return it
    if let Some(value) = cache[n] {
        value
    } else {
        // Otherwise, compute the value recursively,
        // and store it in the cache
        let result = if n < 2 {
            // Base case: trib(0) and trib(1)
            0
        } else if n == 2 {
            // Base case: trib(2)
            1
        } else {
            // recursive case: trib(n) = trib(n-1) + trib(n-2) + trib(n-3)
            trib_recursive(n - 1, cache)
                + trib_recursive(n - 2, cache)
                + trib_recursive(n - 3, cache)
        };
        cache[n] = Some(result);
        result
    }
}

// A helper function that creates a cache and calls fib
fn trib(n: usize) -> usize {
    // Create a vector of n+1 None values
    let mut cache = vec![None; n + 2];
    // Call fib with the cache
    trib_recursive(n, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_00() {
        let result = tribonacci(0, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 0);

        let result = trib(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn fibonacci_01() {
        let result = tribonacci(1, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 0);

        let result = trib(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn fibonacci_02() {
        let result = tribonacci(2, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1);

        let result = trib(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn fibonacci_03() {
        let result = tribonacci(5, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 4);

        let result = trib(5);
        assert_eq!(result, 4);
    }

    #[test]
    fn fibonacci_04() {
        let result = tribonacci(7, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 13);

        let result = trib(7);
        assert_eq!(result, 13);
    }

    #[test]
    fn fibonacci_05() {
        let result = tribonacci(14, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 927);

        let result = trib(14);
        assert_eq!(result, 927);
    }

    #[test]
    fn fibonacci_06() {
        let result = tribonacci(20, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 35890);

        let result = trib(20);
        assert_eq!(result, 35890);
    }

    #[test]
    fn fibonacci_07() {
        let result = tribonacci(37, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1132436852);

        let result = trib(37);
        assert_eq!(result, 1132436852);
    }

    #[test]
    fn fibonacci_08() {
        let result = tribonacci(10, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 81);

        let result = trib(10);
        assert_eq!(result, 81);
    }
}
