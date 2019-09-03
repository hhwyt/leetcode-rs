#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    fn choose_max(mut nums: &Vec<i32>) -> i32 {

    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        Self::choose_max(&mut nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let input = vec![1, 2, 3, 1];
        let expected = 4;
        assert_eq!(Solution::rob(input), expected);
    }

    #[test]
    fn test1() {
        let input = vec![2, 7, 9, 3, 1];
        let expected = 12;
        assert_eq!(Solution::rob(input), expected);
    }
}