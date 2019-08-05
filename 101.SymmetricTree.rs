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
    fn is_symmetric1(
        root1: Option<&Rc<RefCell<TreeNode>>>,
        root2: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (Some(root1), Some(root2)) => {
                let (root1, root2) = (root1.borrow(), root2.borrow());
                let a=  Self::is_symmetric1(root1.left.as_ref(), root2.right.as_ref());
                let b  = root1.val == root2.val;
                return a && b && Self::is_symmetric1(root1.right.as_ref(), root2.left.as_ref())
            }
            (None, None) => true,
            _ => false,
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::is_symmetric1(root.as_ref(), root.as_ref());
    }
}

fn tree_from_vec_recursive(vec: &Vec<i32>, start: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let some_rc_refcell_treenode = |val: i32| Some(Rc::new(RefCell::new(TreeNode::new(val))));
    let root = some_rc_refcell_treenode(vec[start]);

    let left_child = start * 2 + 1;
    if let Some(left_child_val) = vec.get(left_child) {
        if let Some(ref root) = root  {
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
        //
        //         1
        //        / \
        //       2   2
        //      / \ / \
        //     3  4 4  3
        let vec1 = vec![1, 2, 2, 3, 4, 4, 3];
        let root1 = tree_from_vec(vec1);
        assert_eq!(Solution::is_symmetric(root1), true);

        //  1
        //  /
        // 2
        let vec2 = vec![1, 2];
        let root2 = tree_from_vec(vec2);
        assert_eq!(Solution::is_symmetric(root2), false);

        //       1
        //      / \
        //     2   2
        //      \   \
        //       3   3
        let vec3 = vec![1, 2, 2, -1, 3, -3, 3];
        let root3 = tree_from_vec(vec3);
        assert_eq!(Solution::is_symmetric(root3), false);
    }
}
