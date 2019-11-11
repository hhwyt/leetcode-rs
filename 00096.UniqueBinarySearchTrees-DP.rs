struct Solution;

// O(n^2)
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n < 0 {
            return 0;
        }

        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for k in 0..=i - 1 {
                dp[i] += dp[k] * dp[i - k - 1]
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let input = 2;
        let expected = 2;
        assert_eq!(Solution::num_trees(input), expected);
    }

    #[test]
    fn test_3() {
        let input = 3;
        let expected = 5;
        assert_eq!(Solution::num_trees(input), expected);
    }

    #[test]
    fn test_4() {
        let input = 5;
        let expected = 42;
        assert_eq!(Solution::num_trees(input), expected);
    }
}
