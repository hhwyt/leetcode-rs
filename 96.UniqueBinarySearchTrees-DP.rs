#![allow(dead_code)]

struct Solution;

// O(N^2)
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in 1..=i {
                dp[i as usize] += dp[i as usize - j as usize] * dp[j as usize - 1];
            }
        }

        dp[n as usize]
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
