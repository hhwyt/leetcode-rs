struct Solution;

impl Solution {
    fn partition(nums: &mut Vec<i32>, start: i32, end: i32) -> i32 {
        let mut low = start - 1;
        for high in start..end {
            if nums[high as usize] < nums[end as usize] {
                low += 1;
                if low != high {
                    nums.swap(low as usize, high as usize);
                }
            }
        }
        low += 1;
        nums.swap(low as usize, end as usize);
        low
    }

    fn quick_sort(nums: &mut Vec<i32>, start: i32, end: i32) {
        if start >= end {
            return;
        }

        let mid = Self::partition(nums, start, end);
        Self::quick_sort(nums, start, mid - 1);
        Self::quick_sort(nums, mid + 1, end);
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let len = nums.len() as i32;
        Self::quick_sort(&mut nums, 0, len - 1);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v0 = vec![];
        assert_eq!(Solution::sort_array(v0), vec![]);

        let v1 = vec![5, 2, 3, 1];
        assert_eq!(Solution::sort_array(v1), vec![1, 2, 3, 5]);

        let v2 = vec![5, 1, 1, 2, 0, 0];
        assert_eq!(Solution::sort_array(v2), vec![0, 0, 1, 1, 2, 5]);
    }
}
