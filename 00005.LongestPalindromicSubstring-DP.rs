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
        let expected = "aba".to_owned();
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
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_owned();
        }

        let s: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; s.len()]; s.len()];

        for i in 0..s.len() {
            if s[i] == s[s.len() - 1] {
                dp[s.len() - 1][i] = 1;
            }
            if s[i] == s[0] {
                dp[i][0] = 1;
            }
        }

        let mut pos = 0;
        let mut max_len = 1;
        for i in (0..s.len() - 1).rev() {
            for j in 1..s.len() {
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 1;
                    if dp[i][j] as usize > max_len && (j - (dp[i][j] as usize - 1)) == i {
                        pos = j;
                        max_len = dp[i][j] as usize;
                    }
                } else {
                    dp[i][j] = 0;
                }
            }
        }

        s[pos - (max_len - 1)..=pos].iter().collect()
    }
}
