struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority_element = 0;
        let mut count = 0;
        for num in nums {
            if count == 0 {
                majority_element = num;
                count += 1;
            } else {
                if num == majority_element {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
        }
        majority_element
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec0 = vec![3, 2, 3];
        assert_eq!(Solution::majority_element(vec0), 3);

        let vec1 = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element(vec1), 2);
    }
}

