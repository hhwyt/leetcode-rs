struct Solution;

// O(N^2)
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut res = 0;
        for j in 0..len {
            for i in j..len {
                let cur = (i - j) as i32 * height[i].min(height[j]);
                res = res.max(cur);
            }
        }

        return res;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn test_empty() {
            let input = vec![];
            let expected = 0;
            assert_eq!(Solution::max_area(input), expected);
        }
    }

    #[test]
    fn test_0() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;
        assert_eq!(Solution::max_area(input), expected);
    }
}
