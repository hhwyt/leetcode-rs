#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            return 0;
        }
        if m == 1 || n == 1 {
            return 1;
        }
        Self::unique_paths(m - 1, n) + Self::unique_paths(m, n - 1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let (m, n) = (1, 1);
        let expected = 1;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_1_2() {
        let (m, n) = (1, 2);
        let expected = 1;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_2_1() {
        let (m, n) = (2, 1);
        let expected = 1;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_2_2() {
        let (m, n) = (2, 2);
        let expected = 2;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_4_4() {
        let (m, n) = (4, 4);
        let expected = 20;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }
}