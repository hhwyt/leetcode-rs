struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = std::i32::MAX;
        let mut max_profit = 0;
        for i in 0..prices.len() {
            if prices[i] < min {
                min = prices[i];
            } else if prices[i] - min > max_profit {
                max_profit = prices[i] - min;
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
        let v1 = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(v1), 5);

        let v2 = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(v2), 0);
    }
}
