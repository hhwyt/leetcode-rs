#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input1 = 3;
        let expected1 = 0;
        assert_eq!(Solution::trailing_zeroes(input1), expected1);

        let input2 = 5;
        let expected2 = 1;
        assert_eq!(Solution::trailing_zeroes(input2), expected2);

        // Test overflow
        let input3 = 13;
        let expected3 = 2;
        assert_eq!(Solution::trailing_zeroes(input3), expected3);
    }
}

struct Solution;

impl Solution {
    fn trailing_zeroes_recursive(n: i32, sum: i32) -> i32 {
        if n == 0 {
            sum
        } else {
            Self::trailing_zeroes_recursive(n / 5, sum + n / 5)
        }
    }

    pub fn trailing_zeroes(n: i32) -> i32 {
        Self::trailing_zeroes_recursive(n, 0)
    }
}