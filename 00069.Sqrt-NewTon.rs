struct Solution;

impl Solution {
    pub fn my_sqrt(n: i32) -> i32 {
        if n == 0 || n == 1 { return n; }
        let mut x0 = n / 2;
        while x0 > n / x0 {
            x0 = (x0 + n / x0) / 2;
        }
        x0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x0 = 0;
        assert_eq!(Solution::my_sqrt(x0), 0);

        let x1 = 1;
        assert_eq!(Solution::my_sqrt(x1), 1);

        let x4 = 4;
        assert_eq!(Solution::my_sqrt(x4), 2);

        let x8 = 8;
        assert_eq!(Solution::my_sqrt(x8), 2);

        let x_int_max = 2147483647;
        assert_eq!(Solution::my_sqrt(x_int_max), 46340);
    }
}
