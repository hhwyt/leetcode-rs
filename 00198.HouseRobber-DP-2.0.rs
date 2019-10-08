struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];
        for i in 0..nums.len() {
            dp[i + 2] = (nums[i] + dp[i]).max(dp[i + 1]);
        }
        dp[nums.len() + 1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let nums = vec![1, 2, 3, 1];
        //                       n2  n1
        let expected = 4;
        assert_eq!(Solution::rob(nums), expected);
    }

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 9, 3, 1];
        let expected = 12;
        assert_eq!(Solution::rob(nums), expected);
    }
}
