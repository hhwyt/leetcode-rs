struct Solution;

impl Solution {
    fn merge(arr1: &[i32], arr2: &[i32], ret: &mut [i32]) {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < arr1.len() && j < arr2.len() {
            if arr1[i] <= arr2[j] {
                ret[k] = arr1[i];
                i += 1;
            } else {
                ret[k] = arr2[j];
                j += 1;
            }
            k += 1;
        }
        if i < arr1.len() {
            ret[k..].copy_from_slice(&arr1[i..]);
        }
        if j < arr2.len() {
            ret[k..].copy_from_slice(&arr2[j..]);
        }
    }


    fn merge_sort(nums: &mut [i32]) {
        if nums.len() <= 1 {
            return;
        }

        let mid = nums.len() / 2;
        Self::merge_sort(&mut nums[..mid]);
        Self::merge_sort(&mut nums[mid..]);

        let mut ret = nums.to_vec();
        Self::merge(&nums[..mid], &nums[mid..], &mut ret);
        nums.copy_from_slice(&ret);
    }

    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(&mut nums);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general() {
        let nums = vec![11, 9, 10, 9, 11];
        let expected = vec![9, 9, 10, 11, 11];
        assert_eq!(Solution::sort_array(nums), expected);
    }

    #[test]
    fn test_negative() {
        let nums = vec![-1, 2, -8, -10];
        let expected = vec![-10, -8, -1, 2];
        assert_eq!(Solution::sort_array(nums), expected);
    }

    #[test]
    fn test_empty() {
        let nums = vec![];
        let expected = vec![];
        assert_eq!(Solution::sort_array(nums), expected);
    }
}
