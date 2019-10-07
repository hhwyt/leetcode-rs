struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = std::i32::MIN;
        for i in 0..nums.len() {
            let mut cur = 1;
            for j in i..nums.len() {
                cur *= nums[j];
                res = res.max(cur);
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
