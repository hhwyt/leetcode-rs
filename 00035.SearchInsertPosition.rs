struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l != r {
            let mid = l + (r - l) / 2;
            if nums[mid] > target {
                r = mid;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                return mid as i32;
            }
        }

        if nums[l] >= target {
            l as i32
        } else {
            l as i32 + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v0 = vec![3];
        let t0 = 3;
        assert_eq!(Solution::search_insert(v0, t0), 0);

        // [0, 3] m=1
        // [2, 3] m=2
        let v1 = vec![1, 3, 5, 6];
        let t1 = 5;
        assert_eq!(Solution::search_insert(v1, t1), 2);

        // [0, 3] m=1
        // [0]
        let v2 = vec![1, 3, 5, 6];
        let t2 = 2;
        assert_eq!(Solution::search_insert(v2, t2), 1);

        // [0, 3]  m=1
        // [2, 3]  m=2
        // [3]
        let v3 = vec![1, 3, 5, 6];
        let t3 = 7;
        assert_eq!(Solution::search_insert(v3, t3), 4);

        // [0, 3] m=1
        // [0, 1] m=0
        // [0]
        let v4 = vec![1, 3, 5, 6];
        let t4 = 0;
        assert_eq!(Solution::search_insert(v4, t4), 0);

        let v5 = Vec::new();
        let t5 = 1;
        assert_eq!(Solution::search_insert(v5, t5), 0);
    }
}
