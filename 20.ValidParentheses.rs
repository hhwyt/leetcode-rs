struct Solution;

impl Solution {
    fn is_pair_of_ordered_brackets(bracket1: char, bracket2: char) -> bool {
        match bracket1 {
            '{' => bracket2 == '}',
            '[' => bracket2 == ']',
            '(' => bracket2 == ')',
            _ => false,
        }
    }
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            if let Some(&b) = stack.last() {
                if Self::is_pair_of_ordered_brackets(b as char, c as char) {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(Solution::is_valid("".to_owned()));
        assert!(Solution::is_valid("{}".to_owned()));
        assert!(Solution::is_valid("[]{}()".to_owned()));
        assert!(Solution::is_valid("{[()]}".to_owned()));
        assert!(Solution::is_valid("{[]()}".to_owned()));
        assert!(Solution::is_valid("{[]}[]".to_owned()));
    }

    #[test]
    fn test_false() {
        assert!(!Solution::is_valid("x".to_owned()));
        assert!(!Solution::is_valid("([]".to_owned()));
        assert!(!Solution::is_valid("[(]".to_owned()));
        assert!(!Solution::is_valid("(".to_owned()));
        assert!(!Solution::is_valid(")(".to_owned()));
    }
}
