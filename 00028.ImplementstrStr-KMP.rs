struct Solution;

// Time complexity: worst case is O(n+m)
// Space complexity: O(m)
impl Solution {
    fn lps(s: &Vec<u8>) -> Vec<usize> {
        let mut lps = vec![0; s.len()];
        let mut len = 0;
        let mut i = 1;
        while i < s.len() {
            if s[i] == s[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    // This is tricky. Consider the example AAABAAAA and i = 7. You have to go
                    // back.
                    len = lps[len - 1];
                } else {
                    len = 0;
                    lps[i] = len;
                    i += 1;
                }
            }
        }
        lps
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();

        let lps = Solution::lps(&needle);

        let mut i = 0;
        let mut j = 0;
        while j < needle.len() {
            if i + j >= haystack.len() {
                return -1;
            }
            if haystack[i + j] != needle[j] {
                if j == 0 {
                    i += 1;
                } else {
                    i += j - lps[j - 1];
                    j = lps[j - 1];
                }
            } else {
                j += 1;
            }
        }
        return i as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let haystack0a = String::from("aacaadaacaaa");
        let needle0a = String::from("aacaaa");
        assert_eq!(Solution::str_str(haystack0a, needle0a), 6);

        let haystack0a = String::from("dbddbadbddbd");
        let needle0a = String::from("dbddbd");
        assert_eq!(Solution::str_str(haystack0a, needle0a), 6);

        let haystack0c = String::from("ADABADABADAA");
        let needle0c = String::from("ADABADAA");
        assert_eq!(Solution::str_str(haystack0c, needle0c), 4);

        let haystack1 = String::from("hello");
        let needle1 = String::from("ll");
        assert_eq!(Solution::str_str(haystack1, needle1), 2);

        let haystack2 = String::from("aaaaaa");
        let needle2 = String::from("bba");
        assert_eq!(Solution::str_str(haystack2, needle2), -1);

        let haystack3 = String::from("");
        let needle3 = String::from("bba");
        assert_eq!(Solution::str_str(haystack3, needle3), -1);

        let haystack4 = String::from("bba");
        let needle4 = String::from("");
        assert_eq!(Solution::str_str(haystack4, needle4), 0);

        let haystack5 = String::from("");
        let needle5 = String::from("");
        assert_eq!(Solution::str_str(haystack5, needle5), 0);

        // 测试遍历 haystack 是否会越界
        let haystack6 = String::from("aaa");
        let needle6 = String::from("aaaa");
        assert_eq!(Solution::str_str(haystack6, needle6), -1);
    }
}
