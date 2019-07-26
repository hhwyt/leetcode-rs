struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n <= 0 {
            return String::new();
        }
        let mut s = String::from("1");
        if n == 1 {
            return s;
        }

        for _ in 0..n - 1 {
            let mut counter = 0;
            let mut tmp = String::from("");
            let s1 = s.as_bytes();
            for i in 0..s1.len() {
                counter += 1;
                if i == s1.len() - 1 {
                    tmp.push_str(&counter.to_string());
                    tmp.push(s1[i] as char);
                    break;
                }
                if s1[i] != s1[i + 1] {
                    tmp.push_str(&counter.to_string());
                    tmp.push(s1[i] as char);
                    counter = 0
                }
            }
            s = tmp;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        /*
          1.     1
          2.     11
          3.     21
          4.     1211
          5.     111221
        */
        assert_eq!(Solution::count_and_say(1), "1".to_owned());
        assert_eq!(Solution::count_and_say(2), "11".to_owned());
        assert_eq!(Solution::count_and_say(3), "21".to_owned());
        assert_eq!(Solution::count_and_say(4), "1211".to_owned());
        assert_eq!(Solution::count_and_say(5), "111221".to_owned());
        assert_eq!(Solution::count_and_say(6), "312211".to_owned());
    }
}
