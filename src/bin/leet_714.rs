// You are given an array prices where prices[i] is the price of a given stock on the ith day, and an integer fee representing a transaction fee.
//
// Find the maximum profit you can achieve. You may complete as many transactions as you like, but you need to pay the transaction fee for each transaction.
//
// Note:
//
// - You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
// - The transaction fee is only charged once for each stock purchase and sale.

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut hold = -prices[0];
        let mut sold = 0;
        for p in &prices[1..] {
            hold = hold.max(sold - p);
            sold = sold.max(hold + p - fee);
        }
        sold
    }
}

// Notes
//
// 1. Not sure why this falls under the category of Dynamic Programming. We only have two historic values we keep track of.
// 2. The other insight is that we don't actually need to think in terms of buy/sell pairs, but rather we subtract the current price the day we buy and add the price (minus fee) at whatever price is current when we sell.
//
// The inductive step then becomes: hold[N] = max(hold[N-1], sold[N-1]+price[N]) and sold[N] = max(sold[N-1], hold[N-1]+price[N]-fee), i.e., either you keep holding or sell if it is more profitable. Same for selling.
//
// Why this actually works is still surprising to me.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_max_profit() {
        let mut tests = vec![
            (vec![1, 3, 2, 8, 4, 9], 2, 8),
            (vec![1, 3, 7, 5, 10, 3], 3, 6),
        ];
        let mut rng = StdRng::seed_from_u64(42);

        let prices: Vec<i32> = (0..1_000).map(|_| rng.random_range(1..50_000)).collect();
        tests.push((prices, 5, 8138237));

        for (input, fee, result) in tests {
            assert_eq!(Solution::max_profit(input, fee), result);
        }
    }
}
