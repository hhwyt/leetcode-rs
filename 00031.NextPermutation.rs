struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // Find index k such that nums[k] < nums[k+1]
        let mut k = -1;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                k = i as i32;
                break;
            }
        }

        if k != -1 {
            // Find the largest index l such that nums[k] < nums[l]
            let mut l = k as usize + 1;
            for i in (k as usize + 1..nums.len()).rev() {
                if nums[k as usize] < nums[i] {
                    l = i;
                    break;
                }
            }

            nums.swap(k as usize, l);
            nums[k as usize + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let mut nums = vec![1, 2, 3];
        let expected = vec![1, 3, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 1];
        let expected = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![1, 1, 5];
        let expected = vec![1, 5, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, expected);
    }
}
