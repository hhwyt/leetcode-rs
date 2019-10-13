use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let len = queue.len();
            let mut cur_level = Vec::new();
            for _ in 0..len {
                match queue.pop_front().unwrap() {
                    Some(root) => {
                        let mut root = (*root).borrow_mut();
                        cur_level.push(root.val);

                        let left = root.left.take();
                        if left.is_some() {
                            queue.push_back(left);
                        }

                        let right = root.right.take();
                        if right.is_some() {
                            queue.push_back(right);
                        }
                    }
                    None => {}
                }
            }
            res.push(cur_level);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_2_tree_recursive(v: &Vec<Option<i32>>, cur: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if v.get(cur).is_none() || v.get(cur).unwrap().is_none() {
            return None;
        }

        // left_child = parent * 2 + 1
        let mut root = TreeNode::new(v.get(cur).unwrap().unwrap());
        let left_child = cur * 2 + 1;
        root.left = tests::vec_2_tree_recursive(v, left_child);
        root.right = tests::vec_2_tree_recursive(v, left_child + 1);
        Some(Rc::new(RefCell::new(root)))
    }

    fn vec_2_tree(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        return tests::vec_2_tree_recursive(v.as_ref(), 0);
    }

    #[test]
    fn test_normal() {
        let v = vec![Some(1), Some(2), Some(3)];
        let root = tests::vec_2_tree(v);
        let expected = vec![vec![1], vec![2, 3]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn test_empty_tree() {
        let v = vec![];
        let root = tests::vec_2_tree(v);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(root), expected);
    }
}