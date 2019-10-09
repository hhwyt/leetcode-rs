use std::collections::HashMap;

struct Solution;

impl Solution {
    // Sunday algorithm: http://wiki.jikexueyuan.com/project/kmp-algorithm/sunday.html
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        let mut sunday = HashMap::new();
        for (i, c) in needle.iter().enumerate() {
            sunday.insert(c, i);
        }

        let mut i = 0;
        while i <= haystack.len() as i32 - needle.len() as i32 {
            let mut j = 0;
            while j < needle.len() {
                if haystack[i as usize + j] != needle[j] {
                    break;
                }
                j += 1;
            }

            if j == needle.len() {
                return i as i32;
            }

            if i as usize + needle.len() >= haystack.len() {
                return -1;
            }

            let next_of_end = haystack[i as usize + needle.len()];
            match sunday.get(&next_of_end) {
                Some(&k) => {
                    i += needle.len() as i32 - k as i32;
                }
                None => {
                    i += needle.len() as i32 + 1;
                }
            }
        }
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haystack_is_empty() {
        let haystack = "".to_owned();
        let needle = "a".to_owned();
        let expected = -1;
        assert_eq!(Solution::str_str(haystack, needle), expected);
    }

    #[test]
    fn test_needle_is_empty() {
        let haystack = "".to_owned();
        let needle = "".to_owned();
        let expected = 0;
        assert_eq!(Solution::str_str(haystack, needle), expected);
    }

    #[test]
    fn test_needle_longer_than_haystack() {
        let haystack = "aaa".to_owned();
        let needle = "aaaa".to_owned();
        let expected = -1;
        assert_eq!(Solution::str_str(haystack, needle), expected);
    }

    #[test]
    fn test_exist() {
        let haystack = "hello".to_owned();
        let needle = "ll".to_owned();
        let expected = 2;
        assert_eq!(Solution::str_str(haystack, needle), expected);
    }

    #[test]
    fn test_not_exist() {
        let haystack = "aabaa".to_owned();
        let needle = "bab".to_owned();
        let expected = -1;
        assert_eq!(Solution::str_str(haystack, needle), expected);
    }
}