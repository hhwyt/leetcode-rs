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
    fn is_balanced_recursive(
        root: Option<&Rc<RefCell<TreeNode>>>,
        depth: usize,
        max_depth: &mut usize,
    ) -> bool {
        match root {
            Some(root) => {
                let mut left_depth = depth + 1;
                if !Self::is_balanced_recursive(
                    root.borrow().left.as_ref(),
                    depth + 1,
                    &mut left_depth,
                ) {
                    return false;
                }
                let mut right_depth = depth + 1;
                if !Self::is_balanced_recursive(
                    root.borrow().right.as_ref(),
                    depth + 1,
                    &mut right_depth,
                ) {
                    return false;
                }
                if left_depth > right_depth {
                    *max_depth = left_depth;
                    left_depth - right_depth <= 1
                } else {
                    *max_depth = right_depth;
                    right_depth - left_depth <= 1
                }
            }
            None => true,
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::is_balanced_recursive(root.as_ref(), 1, &mut 1);
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
        //    3
        //   / \
        //  9  20
        //    /  \
        //   15   7
        let vec1 = vec![3, 9, 20, -1, -1, 15, 7];
        let root1 = tree_from_vec(vec1);
        assert_eq!(Solution::is_balanced(root1), true);

        //       1
        //      / \
        //     2   2
        //    / \
        //   3   3
        //  / \
        // 4   4
        let vec2 = vec![1, 2, 2, 3, 3, -1, -1, 4, 4];
        let root2 = tree_from_vec(vec2);
        assert_eq!(Solution::is_balanced(root2), false);
    }
}
