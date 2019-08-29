#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = "".to_owned();
        let expected = "".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_normal() {
        let input = "babad".to_owned();
        let expected = "aba".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_duplicate_string() {
        let input = "cbbd".to_owned();
        let expected = "bb".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }
}

struct Solution;

impl Solution {

    pub fn longest_palindrome(s: String) -> String {

    }
}
