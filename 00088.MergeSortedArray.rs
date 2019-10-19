struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m + n - 1;
        let mut m = m - 1;
        let mut n = n - 1;
        while n >= 0 {
            if m >= 0 && nums1[m as usize] >= nums2[n as usize] {
                nums1[i as usize] = nums1[m as usize];
                m -= 1;
            } else {
                nums1[i as usize] = nums2[n as usize];
                n -= 1;
            }
            i -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums1a = vec![0];
        let mut nums2a = vec![2];
        Solution::merge(&mut nums1a, 0, &mut nums2a, 1);
        assert_eq!(nums1a, vec![2]);

        let mut nums1b = vec![2];
        let mut nums2b = vec![];
        Solution::merge(&mut nums1b, 1, &mut nums2b, 0);
        assert_eq!(nums1b, vec![2]);

        let mut nums1c = vec![1, 2, 3, 0, 0, 0];
        let mut nums2c = vec![2, 5, 6];
        Solution::merge(&mut nums1c, 3, &mut nums2c, 3);
        assert_eq!(nums1c, vec![1, 2, 2, 3, 5, 6]);

        let mut nums1d = vec![4, 0, 0, 0];
        let mut nums2d = vec![1, 2, 5];
        Solution::merge(&mut nums1d, 1, &mut nums2d, 3);
        assert_eq!(nums1d, vec![1, 2, 4, 5]);
    }
}
