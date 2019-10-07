#![allow(dead_code)]

struct Solution;

impl Solution {
    fn word_break_recursive(s: &str, word_dict: &Vec<String>) -> bool {
        if s.is_empty() {
            return true;
        }

        for word in word_dict {
            if let Some(pos) = s.find(word) {
                if Self::word_break_recursive(&s[0..pos], word_dict)
                    && Self::word_break_recursive(&s[pos + word.len()..], word_dict) {
                    return true;
                }
            }
        }

        false
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        return Self::word_break_recursive(&s, &word_dict);
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

/*
WORD_BREAK(s, word_dict):
    if s.is_empty then
        return true

    for word in word_dict do
        if s.contains(word) then
            word_pos <- s.pos(word)
            if WORD_BREAK(s[0..word_pos], word_dict)
                && WORD_BREAK(s[word_pos+word.len()..], word_dict)
                return true
    return false
*/
