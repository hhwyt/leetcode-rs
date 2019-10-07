#![allow(dead_code)]

struct Solution;

impl Solution {
    fn word_break_recursive(s: &str, start: usize, end: usize, word_dict: &Vec<String>) -> bool {
        if start == end {
            return true;
        }

        for word in word_dict {
            if let Some(pos) = s[start..end].find(word) {
                if Self::word_break_recursive(&s, start, start + pos, word_dict)
                    && Self::word_break_recursive(&s, start + pos  + word.len(), end, word_dict) {
                    return true;
                }
            }
        }

        false
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        return Self::word_break_recursive(&s, 0, s.len(), &word_dict);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let s = "leetcode".to_owned();
        let word_dict = vec!["leet".to_owned(), "code".to_owned()];
        let expected = true;
        assert_eq!(Solution::word_break(s, word_dict), expected);
    }

    #[test]
    fn test_duplicate() {
        let s = "applepenapple".to_owned();
        let word_dict = vec!["apple".to_owned(), "pen".to_owned()];
        let expected = true;
        assert_eq!(Solution::word_break(s, word_dict), expected);
    }

    #[test]
    fn test_false() {
        let s = "catsandog".to_owned();
        let word_dict = vec!["cats".to_owned(), "dog".to_owned(), "sand".to_owned(), "and".to_owned(), "cat".to_owned()];
        let expected = false;
        assert_eq!(Solution::word_break(s, word_dict), expected);
    }
}
