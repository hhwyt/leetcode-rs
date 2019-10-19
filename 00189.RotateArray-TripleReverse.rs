struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.is_empty() || nums.len() == k as usize && k == 0 {
            return;
        }
       
        let k = k % nums.len() as i32;
        nums[..].reverse();
        nums[0..k as usize].reverse();
        nums[k as usize..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input1 = vec![1, 2, 3, 4, 5, 6, 7];
        let expected1 = vec![5, 6, 7, 1, 2, 3, 4];
        Solution::rotate(&mut input1, 3);
        assert_eq!(input1, expected1);

        let mut input2 = vec![-1, -100, 3, 99];
        let expected2 = vec![3, 99, -1, -100];
        Solution::rotate(&mut input2, 2);
        assert_eq!(input2, expected2);
    }
}
