use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let nums = vec![];
        let expected = false;
        assert_eq!(Solution::contains_duplicate(nums), expected);
    }

    #[test]
    fn test_duplicate() {
        let nums = vec![1, 2, 3, 2];
        let expected = true;
        assert_eq!(Solution::contains_duplicate(nums), expected);
    }

    #[test]
    fn test_no_duplicate() {
        let nums = vec![1, 2, 3];
        let expected = false;
        assert_eq!(Solution::contains_duplicate(nums), expected);
    }
}