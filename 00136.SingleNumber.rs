#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v0 = vec![];
        assert_eq!(Solution::single_number(v0), 0);

        let v1 = vec![2, 2, 1];
        assert_eq!(Solution::single_number(v1), 1);

        let v2 = vec![4, 1, 2, 1, 2];
        assert_eq!(Solution::single_number(v2), 4);
    }
}
