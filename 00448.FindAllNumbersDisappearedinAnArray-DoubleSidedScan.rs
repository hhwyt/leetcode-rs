#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            if nums[j] > 0 {
                nums[j] = -nums[j];
            }
        }

        let mut res = Vec::new();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                res.push(i as i32 + 1);
            }
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let expected = vec![5, 6];
        assert_eq!(Solution::find_disappeared_numbers(input), expected);
    }
}