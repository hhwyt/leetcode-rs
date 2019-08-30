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
    fn test_one_character() {
        let input = "a".to_owned();
        let expected = "a".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_normal_string_1() {
        let input = "babad".to_owned();
        let expected = "bab".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_normal_string_2() {
        let input = "abcda".to_owned();
        let expected = "a".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_duplicate_string() {
        let input = "cbbd".to_owned();
        let expected = "bb".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_duplicate_string_2() {
        let input = "abb".to_owned();
        let expected = "bb".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_loooooooog_string() {
        let input = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg".to_owned();
        let expected = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_loooooooog_string_2() {
        let input = "tfekavrnnptlawqponffseumswvdtjhrndkkjppgiajjhklqpskuubeyofqwubiiduoylurzlorvnfcibxcjjzvlzfvsvwknjkzwthxxrowidmyudbtquktmyunoltklkdvzplxnpkoiikfijgulbxfxhaxnldvwmzpgaiumnvpdirlrutsqenwtihptnhghobrmmzcsrhqgdgzrvvitzgsolsxjxfeencvpnltxeetmtzlwnhlvgtbhkicivylfjhhfqteyxewmnewhmsnfdyneqoywgsgptwdlzbraksgajciebdchindegdfmayvfkwwkkfyxqjcv".to_owned();
        let expected = "kwwk";
        assert_eq!(Solution::longest_palindrome(input), expected);
    }

    #[test]
    fn test_suffix_is_the_reverse_of_suffix() {
        let input = "aacdfcaa".to_owned();
        let expected = "aa".to_owned();
        assert_eq!(Solution::longest_palindrome(input), expected);
    }
}

struct Solution;

impl Solution {
    fn is_palindrome(s: &[char]) -> bool {
        let mut first_idx = 0;
        let mut last_idx = s.len() - 1;
        while first_idx < last_idx {
            if s[first_idx] != s[last_idx] {
                return false;
            }
            first_idx += 1;
            last_idx -= 1;
        }
        true
    }

    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_owned();
        }
        let s: Vec<char> = s.chars().collect();
        let (mut pos, mut max_len) = (0, 1);
        for i in 0..s.len() {
            for j in i + 1..s.len() {
                if Self::is_palindrome(&s[i..=j]) {
                    if s[i..=j].len() > max_len {
                        max_len = s[i..=j].len();
                        pos = j;
                    }
                }
            }
        }
        s[pos - (max_len - 1)..=pos].iter().collect()
    }
}
