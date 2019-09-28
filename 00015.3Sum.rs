#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
                let sum = 0 - nums[i];

                let (mut left, mut right) = (i + 1, nums.len() - 1);
                while left < right {
                    if nums[left] + nums[right] == sum {
                        res.push(vec![nums[i], nums[left], nums[right]]);
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        left += 1;
                        right -= 1;
                    } else if nums[left] + nums[right] < sum {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(Solution::three_sum(input), expected);
    }

    #[test]
    fn test_1() {
        let input = vec![0, 0, 0];
        let expected = vec![vec![0, 0, 0]];
        assert_eq!(Solution::three_sum(input), expected);
    }
}