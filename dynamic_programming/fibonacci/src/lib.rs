use std::collections::HashMap;

/// Write a function fib that takes in a number argument, n,
/// and returns the n-th number of the Fibonacci sequence.
/// The 0-th number of the sequence is 0.
/// The 1-st number of the sequence is 1.
/// To generate further numbers of the sequence, calculate
/// the sum of previous two numbers.
/// Solve this recursively using memoization.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_00() {
        let result = fibonacci(0, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 0);
    }

    #[test]
    fn fibonacci_01() {
        let result = fibonacci(1, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1);
    }

    #[test]
    fn fibonacci_02() {
        let result = fibonacci(2, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1);
    }

    #[test]
    fn fibonacci_03() {
        let result = fibonacci(3, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 2);
    }

    #[test]
    fn fibonacci_04() {
        let result = fibonacci(4, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 3);
    }

    #[test]
    fn fibonacci_05() {
        let result = fibonacci(5, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 5);
    }

    #[test]
    fn fibonacci_06() {
        let result = fibonacci(35, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 9227465);
    }

    #[test]
    fn fibonacci_07() {
        let result = fibonacci(46, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1836311903);
    }

    #[test]
    fn fibonacci_08() {
        let result = fibonacci(10, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 55);
    }
}
