use std::collections::HashMap;

/// Takes in an amount and an array of coins. The
/// function should return the number of different ways
/// it is possible to make change for the given
/// amount using the coins.
///
/// You may reuse a coin as many times as necessary.
///
/// For example,
/// counting_change(4, [1,2,3]) -> 4
///
/// There are four different ways to make an amount of 4:
///
/// 1. 1 + 1 + 1 + 1
/// 2. 1 + 1 + 2
/// 3. 1 + 3
/// 4. 2 + 2
pub fn counting_change<const N: usize>(amount: usize, coins: [usize; N]) -> usize {
    cc_recursive(amount, coins, 0, &mut HashMap::new())
}

pub fn cc_recursive<const N: usize>(
    amount: usize,
    coins: [usize; N],
    idx: usize,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let key = format!("{},{}", amount, idx);
    // If the calculated value is in cache
    // then return it
    if let Some(&value) = memo.get(&key) {
        return value;
    }

    if amount == 0 {
        return 1;
    }

    if idx == coins.len() {
        return 0;
    }

    let coin = coins[idx];
    let mut count = 0;
    let mut qty = 0;
    loop {
        if qty * coin > amount {
            break;
        }

        let remainder = amount - (coin * qty);
        count += cc_recursive(remainder, coins, idx + 1, memo);

        qty += 1;
    }

    memo.insert(key, count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_change_00() {
        let result = counting_change(4, [1, 2, 3]);
        assert_eq!(result, 4);
    }

    #[test]
    fn counting_change_01() {
        let result = counting_change(8, [1, 2, 3]);
        assert_eq!(result, 10);
    }

    #[test]
    fn counting_change_02() {
        let result = counting_change(24, [5, 7, 3]);
        assert_eq!(result, 5);
    }

    #[test]
    fn counting_change_03() {
        let result = counting_change(13, [2, 6, 12, 10]);
        assert_eq!(result, 0);
    }

    #[test]
    fn counting_change_04() {
        let result = counting_change(512, [1, 5, 10, 25]);
        assert_eq!(result, 20119)
    }

    #[test]
    fn counting_change_05() {
        let result = counting_change(1000, [1, 5, 10, 25]);
        assert_eq!(result, 142511);
    }

    #[test]
    fn counting_change_06() {
        let result = counting_change(240, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 1525987916);
    }
}
