struct Solution;

impl Solution {
    fn count_index(num: i32, min: i32) -> usize {
        (num - min) as usize
    }

    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }

        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let mut counters = vec![0; (max - min) as usize + 1];
        let mut outputs = vec![0; nums.len()];

        for i in 0..nums.len() {
            let count_index = Self::count_index(nums[i], min);
            counters[count_index] += 1;
        }

        for i in 1..counters.len() {
            counters[i] += counters[i - 1];
        }


        for i in 0..nums.len() {
            let count_index = Self::count_index(nums[i], min);
            let counter = counters[count_index];
            outputs[counter - 1] = nums[i];
            counters[count_index] -= 1;
        }

        for i in 0..nums.len() {
            nums[i] = outputs[i];
        }

        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general() {
        let nums = vec![1, 3, 1, 8, 2, 2];
        let expected = vec![1, 1, 2, 2, 3, 8];
        assert_eq!(Solution::sort_array(nums), expected);
    }

    #[test]
    fn test_negative() {
        let nums = vec![-1, -3, -3, 2, 1];
        let expected = vec![-3, -3, -1, 1, 2];
        assert_eq!(Solution::sort_array(nums), expected);
    }

    #[test]
    fn test_empty() {
        let nums = vec![];
        let expected = vec![];
        assert_eq!(Solution::sort_array(nums), expected);
    }
}
