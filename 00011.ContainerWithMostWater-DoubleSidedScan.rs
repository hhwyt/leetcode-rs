#![allow(dead_code)]

struct Solution;

// O(N)
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        while left < right {
            let cur = height[left].min(height[right]);
            res = res.max((right - left) as i32 * cur);
            while height[left] <= cur && left < right {
                left += 1;
            }
            while height[right] <= cur && left < right {
                right -= 1;
            }
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn test_empty() {
            let input = vec![];
            let expected = 0;
            assert_eq!(Solution::max_area(input), expected);
        }
    }

    #[test]
    fn test_0() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;
        assert_eq!(Solution::max_area(input), expected);
    }
}
