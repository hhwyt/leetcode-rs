struct Solution;

impl Solution {
    fn to_i32(s: char) -> i32 {
        match s {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut last: i32 = 0;
        let mut result: i32 = 0;
        for c in s.chars() {
            let current = Solution::to_i32(c);
            if current > last {
                result -= 2 * last;
            }
            result += current;
            last = current;
        }
        result
    }
}

fn main() {
    println!("{}", Solution::roman_to_int(String::from("I")));
    println!("{}", Solution::roman_to_int(String::from("VII")));
    println!("{}", Solution::roman_to_int(String::from("IV")));
    println!("{}", Solution::roman_to_int(String::from("IX")));
    println!("{}", Solution::roman_to_int(String::from("XX")));
    println!("{}", Solution::roman_to_int(String::from("XLIV")));
    println!("{}", Solution::roman_to_int(String::from("LVIII")));
    println!("{}", Solution::roman_to_int(String::from("MCMXCIV")));
}
