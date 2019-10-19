struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.as_bytes();
        let mut length = 0;
        let mut len = s.len();
        for i in (0..s.len()).rev() {
            if s[i] != ' ' as u8 {
                break;
            } else {
                len = len - 1;
            }
        }
        for i in (0..len).rev() {
            if s[i] != ' ' as u8 {
                length += 1;
            } else {
                break;
            }
        }
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s0 = String::from("");
        assert_eq!(Solution::length_of_last_word(s0), 0);

        let s1 = String::from("Hello World");
        assert_eq!(Solution::length_of_last_word(s1), 5);

        let s1 = String::from("a ");
        assert_eq!(Solution::length_of_last_word(s1), 1);
    }
}
