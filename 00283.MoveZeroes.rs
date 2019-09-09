#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut low = 0;
        for high in 0..nums.len() {
            if nums[high] != 0 {
                if low != high {
                    nums.swap(low, high);
                }
                low += 1;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut input = vec![];
        let expected = vec![];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_0() {
        let mut input = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_1() {
        let mut input = vec![1, 2, 3];
        let expected = vec![1, 2, 3];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_2() {
        let mut input = vec![0, 0, 0];
        let expected = vec![0, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_3() {
        let mut input = vec![1, 0, 1];
        let expected = vec![1, 1, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }
}