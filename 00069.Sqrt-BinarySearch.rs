struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut high = x;
        let mut low = 0;
        while low != high {
            let mid = low + (high - low) / 2;
            if low == mid {
                return if high <= x / high { high } else { low };
            }
            if mid > x / mid {
                high = mid - 1;
            } else if mid < x / mid {
                low = mid;
            } else {
                return mid;
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x0 = 0;
        assert_eq!(Solution::my_sqrt(x0), 0);

        let x1 = 1;
        assert_eq!(Solution::my_sqrt(x1), 1);

        let x4 = 4;
        assert_eq!(Solution::my_sqrt(x4), 2);

        let x8 = 8;
        assert_eq!(Solution::my_sqrt(x8), 2);

        let x_int_max = 2147483647;
        assert_eq!(Solution::my_sqrt(x_int_max), 46340);
    }
}
