struct Solution;

impl Solution {
    fn swap_slice(nums: &mut Vec<i32>, left_begin: usize, right_begin: usize, len: usize) {
        let (left, right) = nums[left_begin..].split_at_mut(right_begin - left_begin);
        left[..len].swap_with_slice(&mut right[..len]);
    }

    fn rotate_recursive(nums: &mut Vec<i32>, k: i32, s: usize, e: usize) {
        let right_begin = e - k as usize + 1;
        if k as usize <= (right_begin - s) {
            Self::swap_slice(nums, s, right_begin, e - right_begin + 1);
            if (k as usize) < (right_begin - s) {
                Self::rotate_recursive(nums, k, s + k as usize, e)
            }
        } else {
            let swap_len = e - (s + k as usize) + 1;
            Self::swap_slice(nums, s, s + k as usize, swap_len);
            Self::rotate_recursive(nums, k - swap_len as i32, s, e - swap_len);
        }
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if !nums.is_empty() && nums.len() != k as usize && k != 0 {
            Self::rotate_recursive(nums, k % nums.len() as i32, 0, nums.len() - 1)
        }
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

