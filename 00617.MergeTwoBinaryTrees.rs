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
    fn merge_trees_recursive(t1: Option<&Rc<RefCell<TreeNode>>>, t2: Option<&Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (Some(t1), Some(t2)) => {
                let root = Rc::new(RefCell::new(TreeNode::new(t1.borrow().val + t2.borrow().val)));
                root.borrow_mut().left = Self::merge_trees_recursive(t1.borrow().left.as_ref(), t2.borrow().left.as_ref());
                root.borrow_mut().right = Self::merge_trees_recursive(t1.borrow().right.as_ref(), t2.borrow().right.as_ref());
                Some(root)
            }
            (Some(t), None) | (None, Some(t)) => {
                let root = Rc::new(RefCell::new(TreeNode::new(t.borrow().val)));
                root.borrow_mut().left = Self::merge_trees_recursive(t.borrow().left.as_ref(), None);
                root.borrow_mut().right = Self::merge_trees_recursive(t.borrow().right.as_ref(), None);
                Some(root)
            }
            _ => None,
        }
    }
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::merge_trees_recursive(t1.as_ref(), t2.as_ref());
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


    // Input:
    // Tree 1                     Tree 2
    //   1                         2
    //  / \                       / \
    //  3   2                     1   3
    // /                           \   \
    // 5                            4   7
    // Output:
    // Merged tree:
    //    3
    //   / \
    //  4   5
    // / \   \
    // 5   4   7
    #[test]
    fn test_0() {
        let input1 = tree_from_vec(vec![Some(1), Some(3), Some(2), Some(5)]);
        let input2 = tree_from_vec(vec![Some(2), Some(1), Some(3), None, Some(4), None, Some(7)]);
        let expected = tree_from_vec(vec![Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)]);
        assert_eq!(Solution::merge_trees(input1, input2), expected);
    }
}