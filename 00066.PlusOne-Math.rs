struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        if digits.is_empty() {
            digits.push(1);
            return digits;
        }
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
            }
        }
        digits[0] = 1;
        digits.push(0);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v1 = vec![1, 2, 3];
        assert_eq!(Solution::plus_one(v1), vec![1, 2, 4]);

        let v2 = vec![4, 3, 9, 9];
        assert_eq!(Solution::plus_one(v2), vec![4, 4, 0, 0]);

        let v3 = vec![9];
        assert_eq!(Solution::plus_one(v3), vec![1, 0]);

        let v4 = vec![];
        assert_eq!(Solution::plus_one(v4), vec![1]);
    }
}
