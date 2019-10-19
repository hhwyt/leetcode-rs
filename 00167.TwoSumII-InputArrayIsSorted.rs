struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.is_empty() {
            return vec![];
        }
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i != j {
            let sum = numbers[i] + numbers[j];
            if sum == target {
                return vec![i as i32 + 1, j as i32 + 1];
            } else if sum < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec0 = vec![];
        let target0 = 0;
        assert_eq!(Solution::two_sum(vec0, 0), vec![]);

        let vec1 = vec![2, 7, 11, 15];
        let target1 = 9;
        assert_eq!(Solution::two_sum(vec1, target1), vec![1, 2]);

        let vec2 = vec![-1, 0];
        let target2 = -1;
        assert_eq!(Solution::two_sum(vec2, target2), vec![1, 2]);
    }
}
