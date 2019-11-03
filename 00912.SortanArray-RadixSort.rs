struct Solution;

// Let there be d digits in input numbers, and b is the base for representing numbers,
// for example, for decimal system, b is 10 (if negative numbers are supported, b is 19).
// So overall time complexity is O(d*(n+b)) and space complexity is O(n+b)
impl Solution {
    fn cur_digit(num: i32, exp: i32) -> i32 {
        num / exp % 10
    }

    fn key(num: i32, min: i32) -> usize {
        (num - min) as usize
    }

    fn counting_sort(nums: Vec<i32>, exp: i32) -> Vec<i32> {
        // The length 19 is the total integers of the interval [-9, 9]
        let mut counters = vec![0; 19];
        for i in 0..nums.len() {
            let key = Self::key(Self::cur_digit(nums[i], exp), -9);
            counters[key] += 1;
        }
        for i in 1..counters.len() {
            counters[i] += counters[i - 1];
        }

        let mut outputs = vec![0; nums.len()];
        for i in (0..nums.len()).rev() {
            let key = Self::key(Self::cur_digit(nums[i], exp), -9);
            outputs[counters[key] - 1] = nums[i];
            counters[key] -= 1;
        }

        outputs
    }

    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        match nums.iter().map(|&num| num.abs()).max() {
            Some(max) => {
                let mut exp = 1;
                while max / exp > 0 {
                    nums = Self::counting_sort(nums, exp);
                    exp *= 10;
                }
            }
            None => {}
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
