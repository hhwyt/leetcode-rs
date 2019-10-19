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
    fn level_order_bottom_recursive(
        root: Option<&Rc<RefCell<TreeNode>>>,
        depth: usize,
        res: &mut Vec<Vec<i32>>,
    ) {
        if let Some(root) = root {
            match res.get_mut(depth) {
                Some(ref mut vec) => {
                    vec.push(root.borrow().val);
                }
                None => {
                    res.push(vec![root.borrow().val]);
                }
            };

            Self::level_order_bottom_recursive(
                root.borrow().left.as_ref(),
                depth + 1,
                res.as_mut(),
            );
            Self::level_order_bottom_recursive(
                root.borrow().right.as_ref(),
                depth + 1,
                res.as_mut(),
            );
        }
    }

    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        if root.is_none() {
            return res;
        }

        Self::level_order_bottom_recursive(root.as_ref(), 0, res.as_mut());
        res.reverse();
        res
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
        let res0: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order_bottom(root0), res0);

        //       3
        //      / \
        //     9   20
        //        /  \
        //       15   7
        let vec1 = vec![3, 9, 20, -1, -1, 15, 7];
        let root1 = tree_from_vec(vec1);
        assert_eq!(
            Solution::level_order_bottom(root1),
            vec![vec![15, 7], vec![9, 20], vec![3]]
        );

        //       3
        //      / \
        //     9    20
        //    / \   /  \
        //   2  10 15   7
        let vec2 = vec![3, 9, 20, 2, 10, 15, 7];
        let root2 = tree_from_vec(vec2);
        assert_eq!(
            Solution::level_order_bottom(root2),
            vec![vec![2, 10, 15, 7], vec![9, 20], vec![3]]
        );
    }
}
