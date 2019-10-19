struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if !s[i].is_ascii_alphanumeric() {
                i += 1;
                continue;
            }
            if !s[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if s[i].to_ascii_lowercase() != s[j].to_ascii_lowercase() {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s0 = String::from("");
        assert_eq!(Solution::is_palindrome(s0), true);

        let s1 = String::from("A man, a plan, a canal Panama");
        assert_eq!(Solution::is_palindrome(s1), true);

        let s2 = String::from("race a car");
        assert_eq!(Solution::is_palindrome(s2), false);

        let s3 = String::from("0P");
        assert_eq!(Solution::is_palindrome(s3), false);
    }
}
