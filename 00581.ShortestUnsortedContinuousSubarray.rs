struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let len = nums.len();
        let (mut begin, mut end) = (0, -1);
        let (mut max_left, mut min_right) = (nums[0], nums[len - 1]);
        for i in 0..len {
            if nums[i] < max_left {
                end = i as i32;
            }
            if nums[len - 1 - i] > min_right {
                begin = len as i32 - 1 - i as i32;
            }
            max_left = max_left.max(nums[i]);
            min_right = min_right.min(nums[len - 1 - i]);
        }


        return end - begin + 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 3, 2, 4, 6, 5, 7];
        let expected = 5;
        assert_eq!(Solution::find_unsorted_subarray(input), expected);
    }
}