struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut new_end_index = 0;
        for i in (1..nums.len()) {
            if nums[i] != nums[new_end_index] {
                new_end_index += 1;
                nums[new_end_index] = nums[i];
            }
        }

        (new_end_index + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut v1 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut v1), 2);
        assert_eq!(&v1[0..2], [1, 2]);

        let mut v2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut v2), 5);
        assert_eq!(&v2[0..5], [0, 1, 2, 3, 4]);

        let mut v3: Vec<i32> = Vec::new();
        assert_eq!(Solution::remove_duplicates(&mut v3), 0);
        assert_eq!(&v3[..], []);
    }
}
