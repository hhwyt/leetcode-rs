struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut max_profit = 0;
        for i in 0..prices.len() - 1 {
            if prices[i] < prices[i + 1] {
                max_profit += prices[i + 1] - prices[i];
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v0 = vec![];
        assert_eq!(Solution::max_profit(v0), 0);

        let v1 = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(v1), 7);

        let v2 = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(v2), 4);

        let v3 = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(v3), 0);
    }
}
