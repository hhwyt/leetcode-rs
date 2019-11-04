struct Solution;

impl Solution {
    fn heapify(nums: &mut [i32], i: usize) {
        let mut largest = i;
        let left = i * 2 + 1;
        let right = i * 2 + 2;

        if left < nums.len() && nums[left] > nums[largest] {
            largest = left
        }
        if right < nums.len() && nums[right] > nums[largest] {
            largest = right
        }
        if largest != i {
            nums.swap(largest, i);

            // Recursive heapify the affected sub-tree
            Self::heapify(nums, largest);
        }
    }

    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }

        // Build max heap
        for i in (0..=nums.len() / 2 - 1).rev() {
            Self::heapify(&mut nums, i)
        }

        // One by one extract element from heap
        for i in (1..nums.len()).rev() {
            nums.swap(0, i);
            Self::heapify(nums[0..i].as_mut(), 0)
        }

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
