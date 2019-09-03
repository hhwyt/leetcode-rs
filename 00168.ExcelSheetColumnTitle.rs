#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;

    /**
        Input: 1
    Output: "A"
    Example 2:

    Input: 28
    Output: "AB"
    Example 3:

    Input: 701
    Output: "ZY"
        */
    #[test]
    fn test() {
        let v1 = 1;
        assert_eq!(Solution::convert_to_title(v1), String::from("A"));

        let v2 = 28;
        assert_eq!(Solution::convert_to_title(v2), String::from("AB"));

        let v3 = 701;
        assert_eq!(Solution::convert_to_title(v3), String::from("ZY"));
    }
}

struct Solution;

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut s = String::from("");
        let mut n = n;
        while n > 0 {
            n -= 1;
            s.insert(0, ('A' as u8 + (n % 26) as u8) as char);
            n /= 26;
        }
        s
    }
}
