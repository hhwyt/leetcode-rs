struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut new_end_index: i32 = -1;
        for i in 0..nums.len() {
            if val != nums[i] {
                new_end_index += 1;
                nums[new_end_index as usize] = nums[i];
            }
        }

        (new_end_index + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut v1 = vec![3, 2, 2, 3];
        let val1 = 3;
        assert_eq!(Solution::remove_element(&mut v1, 3), 2);
        assert_eq!(v1, vec![2, 2, 2, 3]);

        let mut v2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val2 = 2;
        assert_eq!(Solution::remove_element(&mut v2, val2), 5);
        assert_eq!(v2, vec![0, 1, 3, 0, 4, 0, 4, 2]);

        let mut v3 = Vec::new();
        let val3 = 8;
        assert_eq!(Solution::remove_element(&mut v3, val3), 0);
        assert_eq!(v3, Vec::new());
    }
}
