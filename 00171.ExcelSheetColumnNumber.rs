struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        s.chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                (((c as u8 - 64) % 27) as i32 * 26_i32.pow(i as u32))
            }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input1 = String::from("A");
        let expected1 = 1;
        assert_eq!(Solution::title_to_number(input1), expected1);

        let input2 = String::from("AB");
        let expected2 = 28;
        assert_eq!(Solution::title_to_number(input2), expected2);

        let input3 = String::from("ZY");
        let expected3 = 701;
        assert_eq!(Solution::title_to_number(input3), expected3);

        let input4 = String::from("AAA");
        let expected4 = 703;
        assert_eq!(Solution::title_to_number(input4), expected4);
    }
}

