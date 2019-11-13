struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let mut res = 1 as u64;
        let n = n as u64;
        for i in 1..=n {
            res = res * (n + i) / i;
        }
        (res / (n + 1)) as i32
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
