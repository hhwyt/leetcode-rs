struct Solution;

impl Solution {
    pub fn max_sub_array(v: Vec<i32>) -> i32 {
        if v.is_empty() {
            return 0;
        }
        let mut sum = 0;
        let mut max = v[0];
        for i in 0..v.len() {
            if sum < 0 {
                sum = v[i];
            } else {
                sum += v[i];
            }
            if sum > max {
                max = sum;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v0 = vec![];
        assert_eq!(Solution::max_sub_array(v0), 0);

        let v1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(v1), 6);
    }
}
