struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            return 0;
        }

        let m = m as usize;
        let n = n as usize;

        let mut dp = vec![1; n];

        for i in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }

        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let (m, n) = (1, 1);
        let expected = 1;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_1_2() {
        let (m, n) = (1, 2);
        let expected = 1;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_2_1() {
        let (m, n) = (2, 1);
        let expected = 1;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_2_2() {
        let (m, n) = (2, 2);
        let expected = 2;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_4_4() {
        let (m, n) = (4, 4);
        let expected = 20;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }
}

