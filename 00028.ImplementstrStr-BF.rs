struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        for i in 0..haystack.len() {
            for j in 0..needle.len() {
                if i + j >= haystack.len() {
                    return -1;
                }
                if haystack[i + j] != needle[j] {
                    break;
                }
                if j == needle.len() - 1 {
                    return i as i32;
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
    fn test() {
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
