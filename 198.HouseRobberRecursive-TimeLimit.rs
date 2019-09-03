#![allow(dead_code)]

struct Solution;

impl Solution {
    fn rob_recursive(nums: &Vec<i32>, cur: usize) -> i32 {
        if cur + 2 >= nums.len() {
            return if cur + 1 >= nums.len() {
                nums[cur]
            } else {
                nums[cur].max(nums[cur + 1])
            };
        }

        let sum = nums[cur] + Self::rob_recursive(&nums, cur + 2);
        if cur + 3 >= nums.len() {
            sum.max(nums[cur + 1])
        } else {
            sum.max(nums[cur + 1] + Self::rob_recursive(&nums, cur + 3))
        }
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            0
        } else if nums.len() <= 2 {
            nums.into_iter().max_by(|x, y| x.cmp(y)).unwrap()
        } else {
            Self::rob_recursive(&nums, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = vec![];
        let expected = 0;
        assert_eq!(Solution::rob(input), expected);
    }

    #[test]
    fn test_0() {
        let input = vec![1, 2, 3, 1];
        let expected = 4;
        assert_eq!(Solution::rob(input), expected);
    }

    #[test]
    fn test_1() {
        let input = vec![2, 7, 9, 3, 1];
        let expected = 12;
        assert_eq!(Solution::rob(input), expected);
    }

    #[test]
    fn test_2() {
        let input = vec![2, 1, 1, 2];
        let expected = 4;
        assert_eq!(Solution::rob(input), expected);
    }

    #[test]
    fn test_time_limit() {
        let input = vec![226, 174, 214, 16, 218, 48, 153, 131, 128, 17, 157, 142, 88, 43, 37, 157, 43, 221, 191, 68, 206, 23, 225, 82, 54, 118, 111, 46, 80, 49, 245, 63, 25, 194, 72, 80, 143, 55, 209, 18, 55, 122, 65, 66, 177, 101, 63, 201, 172, 130, 103, 225, 142, 46, 86, 185, 62, 138, 212, 192, 125, 77, 223, 188, 99, 228, 90, 25, 193, 211, 84, 239, 119, 234, 85, 83, 123, 120, 131, 203, 219, 10, 82, 35, 120, 180, 249, 106, 37, 169, 225, 54, 103, 55, 166, 124];
        Solution::rob(input);
    }
}