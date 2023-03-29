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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_00() {
        let result = tribonacci(0, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 0);
    }

    #[test]
    fn fibonacci_01() {
        let result = tribonacci(1, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 0);
    }

    #[test]
    fn fibonacci_02() {
        let result = tribonacci(2, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1);
    }

    #[test]
    fn fibonacci_03() {
        let result = tribonacci(5, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 4);
    }

    #[test]
    fn fibonacci_04() {
        let result = tribonacci(7, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 13);
    }

    #[test]
    fn fibonacci_05() {
        let result = tribonacci(14, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 927);
    }

    #[test]
    fn fibonacci_06() {
        let result = tribonacci(20, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 35890);
    }

    #[test]
    fn fibonacci_07() {
        let result = tribonacci(37, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 1132436852);
    }

    #[test]
    fn fibonacci_08() {
        let result = tribonacci(10, &mut HashMap::<usize, usize>::new());
        assert_eq!(result, 81);
    }
}
