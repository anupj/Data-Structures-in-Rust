use std::collections::HashMap;

/// Write a function fib that takes in a number argument, n,
/// and returns the n-th number of the Fibonacci sequence.
/// The 0-th number of the sequence is 0.
/// The 1-st number of the sequence is 1.
/// To generate further numbers of the sequence, calculate
/// the sum of previous two numbers.
/// Solve this recursively using memoization.
/// Read `memo` as `cache`
pub fn fibonacci(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    // Using a `memo` `HashMap` to
    // store and retrieve a calculated value;
    // we avoid redoing unnecessary recursive calls
    // to values we have already visited.
    if memo.contains_key(&n) {
        return memo[&n];
    }

    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let fib_n_minus_1 = fibonacci(n - 1, memo);
    let fib_n_minus_2 = fibonacci(n - 2, memo);
    memo.insert(n, fib_n_minus_1 + fib_n_minus_2);

    memo[&n]
}

/// Here's a more "idiomatic" way of writing
/// the same function.
/// I'll use `cache` instead of `memo`
pub fn fib(n: usize, cache: &mut Vec<Option<usize>>) -> usize {
    // If the cache already contains the value
    // return it
    if let Some(value) = cache[n] {
        value
    } else {
        // Otherwise, compute the value recursively,
        // and store it in the cache
        let result = if n < 2 {
            // Base case: fib(0) and fib(1)
            n
        } else {
            // recursive case: fib(n) = fib(n-1) + fib(n-2)
            fib(n - 1, cache) + fib(n - 2, cache)
        };
        cache[n] = Some(result);
        result
    }
}

// A helper function that creates a cache and calls fib
fn fib_with_cache(n: usize) -> usize {
    // Create a vector of n+1 None values
    let mut cache = vec![None; n + 1];
    // Call fib with the cache
    fib(n, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_00() {
        let result = fibonacci(0, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 0);
        let result = fib_with_cache(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn fibonacci_01() {
        let result = fibonacci(1, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1);
        let result = fib_with_cache(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn fibonacci_02() {
        let result = fibonacci(2, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1);
        let result = fib_with_cache(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn fibonacci_03() {
        let result = fibonacci(3, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 2);
        let result = fib_with_cache(3);
        assert_eq!(result, 2);
    }

    #[test]
    fn fibonacci_04() {
        let result = fibonacci(4, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 3);
        let result = fib_with_cache(4);
        assert_eq!(result, 3);
    }

    #[test]
    fn fibonacci_05() {
        let result = fibonacci(5, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 5);
        let result = fib_with_cache(5);
        assert_eq!(result, 5);
    }

    #[test]
    fn fibonacci_06() {
        let result = fibonacci(35, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 9227465);
        let result = fib_with_cache(35);
        assert_eq!(result, 9227465);
    }

    #[test]
    fn fibonacci_07() {
        let result = fibonacci(46, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1836311903);
        let result = fib_with_cache(46);
        assert_eq!(result, 1836311903);
    }

    #[test]
    fn fibonacci_08() {
        let result = fibonacci(10, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 55);
        let result = fib_with_cache(10);
        assert_eq!(result, 55);
    }
}
