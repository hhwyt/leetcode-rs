struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // Catalan number: Cn = C(n, 2n)/(n+1)
        // https://en.wikipedia.org/wiki/Catalan_number
        let mut res = 1;
        for i in 1..=n {
            res = res * (i + n) / i;
        }
        res / (n + 1)
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
