#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if m == 0 || n == 0 {
            return 0;
        }

        let mut dp = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    dp[j] = grid[i][j];
                } else if i == 0 {
                    dp[j] = dp[j - 1] + grid[i][j];
                } else if j == 0 {
                    dp[j] = dp[j] + grid[i][j];
                } else {
                    dp[j] = dp[j - 1].min(dp[j]) + grid[i][j];
                }
            }
        }

        dp[n - 1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let expected = 7;
        assert_eq!(Solution::min_path_sum(input), expected);
    }
}