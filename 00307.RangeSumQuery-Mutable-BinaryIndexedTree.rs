struct BinaryIndexedTree {
    sums: Vec<i32>
}

impl BinaryIndexedTree {
    fn new(len: usize) -> BinaryIndexedTree {
        BinaryIndexedTree {
            sums: vec![0; len + 1]
        }
    }

    fn update(&mut self, mut i: usize, delta: i32) {
        while i < self.sums.len() {
            self.sums[i] += delta;
            i += self.lowbit(i as i32) as usize;
        }
    }

    fn query(&self, mut i: usize) -> i32 {
        let mut sum = 0;
        while i > 0 {
            sum += self.sums[i];
            i -= self.lowbit(i as i32) as usize;
        }
        sum
    }

    fn lowbit(&self, x: i32) -> i32 {
        x & -x
    }
}

struct NumArray {
    tree: BinaryIndexedTree,
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut tree = BinaryIndexedTree::new(nums.len());
        for i in 0..nums.len() {
            tree.update(i + 1, nums[i]);
        }

        NumArray {
            tree,
            nums,
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        if i < 0 || i >= self.nums.len() as i32 {
            return;
        }

        self.tree.update(i as usize + 1, val - self.nums[i as usize]);
        // Don't forget to update num
        self.nums[i as usize] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        if i > j || i < 0 || j >= self.nums.len() as i32 {
            return 0;
        }

        self.tree.query(j as usize + 1) - self.tree.query(i as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general() {
        let nums = vec![1, 3, 5];
        let mut num_array = NumArray::new(nums);
        assert_eq!(num_array.sum_range(0, 2), 9);
        num_array.update(1, 2);
        assert_eq!(num_array.sum_range(0, 2), 8);
    }

    #[test]
    fn test_empty() {
        let nums = vec![];
        let mut num_array = NumArray::new(nums);
        assert_eq!(num_array.sum_range(0, 2), 0);
        num_array.update(1, 2);
    }

    #[test]
    fn test_i_grater_than_j() {
        let nums = vec![1, 2];
        let num_array = NumArray::new(nums);
        assert_eq!(num_array.sum_range(2, 0), 0);
    }

    #[test]
    fn test_update_multiple_times() {
        let nums = vec![7, 2, 7, 2, 0];
        let mut num_array = NumArray::new(nums);
        num_array.update(4, 6);
        num_array.update(0, 2);
        num_array.update(0, 9);
        assert_eq!(num_array.sum_range(4, 4), 6);
        num_array.update(3, 8);
        assert_eq!(num_array.sum_range(0, 4), 32);
    }
}