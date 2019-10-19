struct Solution;

impl Solution {
    /*
        [4,3,2,7,8,2,3,1] 4

         _ 3 2 4 8 2 3 1  7
         _ 3 2 4 8 2 7 1  3
         _ 3 3 4 8 2 7 1  2
         _ 2 3 4 8 2 7 1  3
         _ 2 3 4 _ 2 7 8  1
         1 2 3 4 _ _ 7 8
    */
    // Time Complexity: O(3n)
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            let mut j = i;
            let mut moved = 0;
            while nums[j] != j as i32 + 1 {
                let tmp = moved;
                moved = nums[j];
                nums[j] = tmp;
                if moved == 0 {
                    break;
                }
                j = moved as usize - 1;
            }
        }

        nums.into_iter()
            .enumerate()
            .filter(|&(_, v)| v == 0)
            .map(|(i, _)| i as i32 + 1)
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input = vec![];
        let expected = vec![];
        assert_eq!(Solution::find_disappeared_numbers(input), expected);
    }

    #[test]
    fn test() {
        let input = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let expected = vec![5, 6];
        assert_eq!(Solution::find_disappeared_numbers(input), expected);
    }
}
