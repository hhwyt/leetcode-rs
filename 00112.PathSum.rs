#![allow(dead_code)]

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            Some(root) => {
                let sum = sum - root.borrow().val;
                match (root.borrow().left.clone(), root.borrow().right.clone()) {
                    (None, None) => sum == 0,
                    (left, right) => {
                        Self::has_path_sum(left, sum) || Self::has_path_sum(right, sum)
                    }
                }
            }
            None => false,
        }
    }
}

fn tree_from_vec_recursive(vec: &Vec<i32>, start: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let some_rc_refcell_treenode = |val: i32| Some(Rc::new(RefCell::new(TreeNode::new(val))));
    let root = some_rc_refcell_treenode(vec[start]);

    let left_child = start * 2 + 1;
    if let Some(left_child_val) = vec.get(left_child) {
        if let Some(ref root) = root {
            if *left_child_val == -1 {
                root.borrow_mut().left = None;
            } else {
                root.borrow_mut().left = match tree_from_vec_recursive(vec, left_child) {
                    Some(left) => Some(left),
                    None => None,
                }
            }
        }
    } else {
        if let Some(ref root) = root {
            root.borrow_mut().left = None;
        }
    }
    let right_child = start * 2 + 2;
    if let Some(right_child_val) = vec.get(right_child) {
        if let Some(ref root) = root {
            if *right_child_val == -1 {
                root.borrow_mut().right = None;
            } else {
                root.borrow_mut().right = match tree_from_vec_recursive(vec, right_child) {
                    Some(right) => Some(right),
                    None => None,
                }
            }
        }
    } else {
        if let Some(ref root) = root {
            root.borrow_mut().right = None;
        }
    }
    root
}

pub fn tree_from_vec(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() {
        None
    } else {
        Some(tree_from_vec_recursive(&vec, 0).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec0 = vec![];
        let root0 = tree_from_vec(vec0);
        assert_eq!(Solution::has_path_sum(root0, 0), false);

        //      5
        //     / \
        //    4   8
        //   /   / \
        //  11  13  4
        // /  \      \
        //7    2      1
        let vec1 = vec![5, 4, 8, 11, -1, 13, 4, 7, 2, -1, -1, -1, 1];
        let root1 = tree_from_vec(vec1);
        assert_eq!(Solution::has_path_sum(root1, 22), true);

        let vec2 = vec![1, 2];
        let root2 = tree_from_vec(vec2);
        assert_eq!(Solution::has_path_sum(root2, 1), false);
    }
}
