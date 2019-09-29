#![allow(dead_code)]

struct Solution;

impl Solution {
    fn rob_recursive(nums: &Vec<i32>, i: usize) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        return (nums[i] + Self::rob_recursive(&nums, i + 2)).max(Self::rob_recursive(&nums, i + 1));
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            0
        } else {
            Self::rob_recursive(&nums, 0)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = vec![];
        let expected = 0;
        assert_eq!(Solution::rob(input), expected);
    }

    #[test]
    fn test_0() {
        let input = vec![1, 2, 3, 1];
        let expected = 4;
        assert_eq!(Solution::rob(input), expected);
    }

    #[test]
    fn test_1() {
        let input = vec![2, 7, 9, 3, 1];
        let expected = 12;
        assert_eq!(Solution::rob(input), expected);
    }

    #[test]
    fn test_2() {
        let input = vec![2, 1, 1, 2];
        let expected = 4;
        assert_eq!(Solution::rob(input), expected);
    }
}