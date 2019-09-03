//use std::str;

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }

        let mut common_prefix: Vec<u8> = strs[0].as_bytes().to_vec();

        for s in &strs[1..strs.len()] {
            let len = s.len();
            if len < common_prefix.len() {
                common_prefix = common_prefix[..len].to_vec();
            }
            for i in 0..len {
                if let Some(c) = s.as_bytes().get(i) {
                    if let Some(p) = common_prefix.get(i) {
                        if c != p {
                            common_prefix = common_prefix[..i].to_vec();
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        common_prefix.into_iter().map(|s| s as char).collect()
        //str::from_utf8(&common_prefix[..]).unwrap().to_string()
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_common_prefix(vec![String::from("")])
    );
    println!(
        "{}",
        Solution::longest_common_prefix(vec![String::from("a"), String::from("b")])
    );

    println!(
        "p1: {}",
        Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ])
    );
    println!(
        "p2: {}",
        Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flo")
        ])
    );

    println!(
        "p2: {}",
        Solution::longest_common_prefix(vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car")
        ])
    );
    println!(
        "p3: {}",
        Solution::longest_common_prefix(vec![
            String::from("fower"),
            String::from("flow"),
            String::from("flight")
        ])
    );
}
