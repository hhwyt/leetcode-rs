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

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    fn invert_tree_recursive(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.take().map(|root| {
            let left = Self::invert_tree_recursive(&mut root.borrow_mut().left);
            let right = Self::invert_tree_recursive(&mut root.borrow_mut().right);
            root.borrow_mut().left = right;
            root.borrow_mut().right = left;
            root
        })
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        return Self::invert_tree_recursive(&mut root);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn tree_from_vec_recursive(vec: &Vec<Option<i32>>, start: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let some_rc_refcell_treenode = |val: i32| Some(Rc::new(RefCell::new(TreeNode::new(val))));
        let root = some_rc_refcell_treenode(vec[start].unwrap());

        let left_child = start * 2 + 1;
        if let Some(left_child_val) = vec.get(left_child) {
            if let Some(ref root) = root {
                if left_child_val.is_none() {
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
                if right_child_val.is_none() {
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

    fn tree_from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() {
            None
        } else {
            Some(tree_from_vec_recursive(&vec, 0).unwrap())
        }
    }

    //     4
//   /   \
//  2     7
// / \   / \
//1   3 6   9
    #[test]
    fn test_0() {
        let input = tree_from_vec(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
        let expected = tree_from_vec(vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)]);
        assert_eq!(Solution::invert_tree(input), expected);
    }
}