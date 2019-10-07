#![allow(dead_code)]

struct Solution;

// O(N!)
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }

        let mut res = 0;
        for i in 1..=n {
            res += Self::num_trees(n - i) * Self::num_trees(i - 1);
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let input = 2;
        let expected = 2;
        assert_eq!(Solution::num_trees(input), expected);
    }

    #[test]
    fn test_3() {
        let input = 3;
        let expected = 5;
        assert_eq!(Solution::num_trees(input), expected);
    }

    #[test]
    fn test_4() {
        let input = 5;
        let expected = 42;
        assert_eq!(Solution::num_trees(input), expected);
    }
}
