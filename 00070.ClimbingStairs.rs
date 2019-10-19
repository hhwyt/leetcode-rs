struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n).fold((0, 1), |acc, _| (acc.1, acc.0 + acc.1)).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v0 = 0;
        assert_eq!(Solution::climb_stairs(v0), 1);

        let v1 = 1;
        assert_eq!(Solution::climb_stairs(v1), 1);

        let v2 = 2;
        assert_eq!(Solution::climb_stairs(v2), 2);

        let v4 = 4;
        assert_eq!(Solution::climb_stairs(v4), 5);

        let v5 = 5;
        assert_eq!(Solution::climb_stairs(v5), 8);

        let v35 = 35;
        assert_eq!(Solution::climb_stairs(v35), 14930352);

        let v44 = 44;
        assert_eq!(Solution::climb_stairs(v44), 1134903170);
    }
}
