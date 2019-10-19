struct Solution;

// 1
// 1  1
// 1  2  1
// 1  3  3  1
// 1  4  6   4  1
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..=row_index as usize {
            result.push(1);
            for j in (1..i).rev() {
                result[j] += result[j - 1];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let row_index0 = 0;
        assert_eq!(Solution::get_row(row_index0), vec![1]);

        let row_index1 = 1;
        assert_eq!(Solution::get_row(row_index1), vec![1, 1]);

        let row_index2 = 2;
        assert_eq!(Solution::get_row(row_index2), vec![1, 2, 1]);

        let row_index3 = 3;
        assert_eq!(Solution::get_row(row_index3), vec![1, 3, 3, 1]);
    }
}
