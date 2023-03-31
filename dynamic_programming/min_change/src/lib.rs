use std::collections::HashMap;

/// Takes in an amount and an array of coins.
/// The function should return the minimum number
/// of coins required to create the amount. You
/// may use each coin as many times as necessary.
///
/// If it is not possible to create the amount, then return -1.
fn min_change(amount: usize, coins: &[usize]) -> i32 {
    // Create a vector to store the minimum number of coins for each sub-amount
    // Initialize it with amount + 1 as a large value
    let mut dp = vec![amount + 1; amount + 1];
    // The minimum number of coins for 0 is 0
    dp[0] = 0;
    // Loop through all the sub-amounts from 1 to amount
    for i in 1..=amount {
        // Loop through all the coins
        for &coin in coins {
            // If the coin is smaller than or equal to the sub-amount
            if coin <= i {
                // Update the minimum number of coins for the sub-amount
                // by taking the minimum of the current value and the
                // value obtained by using one coin
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }
    // If the minimum number of coins for the amount is larger than amount
    // then it means it is not possible to create the amount
    if dp[amount] > amount {
        // Return -1
        -1
    } else {
        // Otherwise, return the minimum number of coins for the amount as an i32
        dp[amount] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_possible_00() {
        // let result = min_change::<4>(8, [1, 5, 12, 4]);
        let result = min_change(8, &[1, 5, 12, 4]);
        assert_eq!(result, 2);
    }

    #[test]
    fn sum_possible_01() {
        // let result = min_change::<5>(13, [1, 9, 5, 14, 30]);
        let result = min_change(13, &[1, 9, 5, 14, 30]);
        assert_eq!(result, 5);
    }

    #[test]
    fn sum_possible_02() {
        // let result = min_change::<3>(23, [2, 5, 7]);
        let result = min_change(23, &[2, 5, 7]);
        assert_eq!(result, 4);
    }

    #[test]
    fn sum_possible_03() {
        // let result = min_change::<4>(102, [1, 5, 10, 25]);
        let result = min_change(102, &[1, 5, 10, 25]);
        assert_eq!(result, 6);
    }

    #[test]
    fn sum_possible_04() {
        // let result = min_change::<4>(200, [1, 5, 10, 25]);
        let result = min_change(200, &[1, 5, 10, 25]);
        assert_eq!(result, 8);
    }

    #[test]
    fn sum_possible_05() {
        // let result = min_change::<3>(2017, [4, 2, 10]);
        let result = min_change(2017, &[4, 2, 10]);
        assert_eq!(result, -1);
    }

    #[test]
    fn sum_possible_06() {
        // let result = min_change::<4>(271, [10, 8, 265, 24]);
        let result = min_change(271, &[10, 8, 265, 24]);
        assert_eq!(result, -1);
    }

    #[test]
    fn sum_possible_07() {
        // let result = min_change::<3>(0, [4, 2, 10]);
        let result = min_change(0, &[4, 2, 10]);
        assert_eq!(result, 0);
    }

    #[test]
    fn sum_possible_08() {
        // let result = min_change::<0>(0, []);
        let result = min_change(0, &[]);
        assert_eq!(result, 0);
    }

    #[test]
    fn sum_possible_09() {
        // let result = min_change::<2>(13, [13, 5]);
        let result = min_change(13, &[13, 5]);
        assert_eq!(result, 1);
    }
}
