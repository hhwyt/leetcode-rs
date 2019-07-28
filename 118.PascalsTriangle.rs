#![allow(dead_code)]
struct Solution;

/*
Input: 5
Output:
[
     [1],
    [1,1],
   [1,2,1],
  [1,3,3,1],
 [1,4,6,4,1]
]
*/
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for i in 0..num_rows as usize {
            result.push(vec![1; i + 1]);
            for j in 1..i {
                result[i][j] = result[i - 1][j - 1] + result[i - 1][j];
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
        let num_rows1 = 1;
        assert_eq!(Solution::generate(num_rows1), vec![vec![1]]);

        let num_rows2 = 2;
        assert_eq!(Solution::generate(num_rows2), vec![vec![1], vec![1, 1]]);

        let num_rows3 = 4;
        assert_eq!(
            Solution::generate(num_rows3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
        );
    }
}
