use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution;

impl Solution {
    fn inorder_traversal_recursive(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        match root {
            Some(root) => {
                let root = RefCell::borrow(root);
                Self::inorder_traversal_recursive(&root.left, v);
                v.push(root.val);
                Self::inorder_traversal_recursive(&root.right, v);
            }
            _ => {}
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Self::inorder_traversal_recursive(&root, &mut v);
        v
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
    fn test() {
        //      1
        //    2   3
        //  4  6   5
        let v = vec![Some(1), Some(2), Some(3), Some(4), Some(6), None, Some(5)];
        let root = vec_2_tree(v);
        let expected = vec![4, 2, 6, 1, 3, 5];
        assert_eq!(Solution::inorder_traversal(root), expected);
    }
}