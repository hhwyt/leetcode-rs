use std::mem;

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut res = nums[0];
        let (mut min, mut max) = (nums[0], nums[0]);
        for &num in nums[1..].into_iter() {
            if num < 0 {
                mem::swap(&mut min, &mut max);
            }

            min = num.min(min * num);
            max = num.max(max * num);

            res = res.max(max);
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let nums = vec![2, 3, -2, 4];
        let expected = 6;
        assert_eq!(Solution::max_product(nums), expected);
    }

    #[test]
    fn test_1() {
        let nums = vec![-2, 0, -1];
        let expected = 0;
        assert_eq!(Solution::max_product(nums), expected);
    }
}
